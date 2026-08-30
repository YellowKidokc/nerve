//! Mouse trigger for the selection surfaces.
//!
//! Stratum's AHK glue put its popup on the middle mouse button, and that is
//! the convention this machine already has muscle memory for. This module
//! reproduces it natively with a low-level mouse hook, so the popup is one
//! click away rather than a chord.
//!
//! Same shape as the hotstring engine: a dedicated thread owns the hook and
//! pumps messages, because a low-level hook callback is delivered on the
//! thread that installed it.

use crate::config::Config;
use crate::AppEvent;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use tao::event_loop::EventLoopProxy;
use tracing::info;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

/// Install the mouse hook and pump messages. Runs until the process exits.
pub fn engine(proxy: EventLoopProxy<AppEvent>, cfg: Arc<Mutex<Config>>) -> Result<()> {
    // The hook callback runs on this thread, so thread-local state is enough
    // and avoids a global lock on the mouse path.
    HOOK_STATE.with(|hs| {
        *hs.borrow_mut() = Some(HookState { proxy, cfg });
    });

    let hook = unsafe { SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), None, 0)? };

    info!("Selection mouse hook installed");

    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        let _ = UnhookWindowsHookEx(hook);
    }

    Ok(())
}

struct HookState {
    proxy: EventLoopProxy<AppEvent>,
    cfg: Arc<Mutex<Config>>,
}

thread_local! {
    static HOOK_STATE: std::cell::RefCell<Option<HookState>> =
        std::cell::RefCell::new(None);
    /// Last click of the trigger button: which button, and when (ms since
    /// boot, from the hook's own timestamp). Used to detect a double-click.
    static LAST_CLICK: std::cell::RefCell<Option<(Button, u32)>> =
        std::cell::RefCell::new(None);
}

/// Did this click complete a multi-click within `window_ms`?
///
/// Uses the hook's event timestamp rather than wall clock, so it measures the
/// gap between the actual input events.
fn completes_multi_click(button: Button, time: u32, needed: u32, window_ms: u32) -> bool {
    if needed <= 1 {
        return true;
    }

    LAST_CLICK.with(|lc| {
        let mut last = lc.borrow_mut();

        let is_second = matches!(
            *last,
            Some((prev_button, prev_time))
                if prev_button == button && time.wrapping_sub(prev_time) <= window_ms
        );

        if is_second {
            // Consume the pair so a third click starts a fresh one rather
            // than firing again immediately.
            *last = None;
            true
        } else {
            *last = Some((button, time));
            false
        }
    })
}

/// The system double-click time, used when the config leaves the window at 0.
fn system_double_click_ms() -> u32 {
    use windows::Win32::UI::Input::KeyboardAndMouse::GetDoubleClickTime;
    unsafe { GetDoubleClickTime() }
}

/// Which physical button a hook message represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Button {
    Middle,
    X1,
    X2,
}

impl Button {
    /// Match against a configured button name. Unknown names match nothing,
    /// which disables the trigger rather than silently falling back to a
    /// button the user did not ask for.
    fn matches(self, name: &str) -> bool {
        matches!(
            (self, name.trim().to_lowercase().as_str()),
            (Button::Middle, "middle" | "mbutton" | "middle_click")
                | (Button::X1, "x1" | "xbutton1" | "back")
                | (Button::X2, "x2" | "xbutton2" | "forward")
        )
    }
}

unsafe extern "system" fn mouse_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let info = &*(lparam.0 as *const MSLLHOOKSTRUCT);

    // Never react to input we injected ourselves — the capture path sends
    // keystrokes, and a feedback loop here would be very hard to diagnose.
    if info.flags & LLMHF_INJECTED != 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    // Fire on button *up*. Acting on the down event would open a window in the
    // middle of a click the user might still be using for something else.
    let button = match wparam.0 as u32 {
        WM_MBUTTONUP => Some(Button::Middle),
        WM_XBUTTONUP => match (info.mouseData >> 16) as u16 {
            XBUTTON1 => Some(Button::X1),
            XBUTTON2 => Some(Button::X2),
            _ => None,
        },
        _ => None,
    };

    let button = match button {
        Some(b) => b,
        None => return CallNextHookEx(None, code, wparam, lparam),
    };

    let mut swallow = false;

    HOOK_STATE.with(|hs| {
        let borrow = hs.borrow();
        let state = match borrow.as_ref() {
            Some(s) => s,
            None => return,
        };

        let (enabled, want_button, modifier, want_swallow, target, clicks, window) = {
            let cfg = match state.cfg.lock() {
                Ok(c) => c,
                Err(_) => return,
            };
            let m = &cfg.selection.mouse;
            (
                cfg.selection.enabled && m.enabled,
                m.button.clone(),
                m.modifier.clone(),
                m.swallow,
                m.target.clone(),
                m.clicks,
                m.double_window_ms,
            )
        };

        if !enabled || !button.matches(&want_button) || !modifier_held(&modifier) {
            return;
        }

        // The button is ours from here on. Swallow every click of it, not just
        // the one that fires: letting the first of a double-click through
        // would close a browser tab on the way to opening the popup.
        swallow = want_swallow;

        let window = if window == 0 {
            system_double_click_ms()
        } else {
            window
        };

        if !completes_multi_click(button, info.time, clicks, window) {
            return;
        }

        let _ = state.proxy.send_event(AppEvent::MouseTrigger(target));
    });

    if swallow {
        // Non-zero stops the click reaching the foreground application.
        return LRESULT(1);
    }

    CallNextHookEx(None, code, wparam, lparam)
}

/// Is the configured modifier held? An empty name means "no modifier
/// required", which is the common case.
fn modifier_held(modifier: &str) -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_CONTROL, VK_MENU, VK_SHIFT,
    };

    let key = match modifier.trim().to_lowercase().as_str() {
        "" | "none" => return true,
        "ctrl" | "control" => VK_CONTROL,
        "alt" => VK_MENU,
        "shift" => VK_SHIFT,
        // An unrecognised modifier must not silently behave as "no modifier".
        _ => return false,
    };

    unsafe { (GetAsyncKeyState(key.0 as i32) as u16 & 0x8000) != 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_names_and_aliases_match() {
        assert!(Button::Middle.matches("middle"));
        assert!(Button::Middle.matches("MButton"));
        assert!(Button::X1.matches("x1"));
        assert!(Button::X2.matches("forward"));
    }

    #[test]
    fn buttons_do_not_cross_match() {
        assert!(!Button::Middle.matches("x1"));
        assert!(!Button::X1.matches("middle"));
    }

    #[test]
    fn unknown_button_name_disables_rather_than_defaults() {
        assert!(!Button::Middle.matches("wheel"));
        assert!(!Button::Middle.matches(""));
    }

    #[test]
    fn single_click_mode_fires_immediately() {
        assert!(completes_multi_click(Button::Middle, 1000, 1, 500));
    }

    #[test]
    fn double_click_needs_two_clicks_inside_the_window() {
        // First click arms, does not fire.
        assert!(!completes_multi_click(Button::Middle, 1000, 2, 500));
        // Second click inside the window fires.
        assert!(completes_multi_click(Button::Middle, 1200, 2, 500));
    }

    #[test]
    fn slow_second_click_rearms_instead_of_firing() {
        assert!(!completes_multi_click(Button::X1, 1000, 2, 500));
        // Too late to pair — becomes the start of a new attempt.
        assert!(!completes_multi_click(Button::X1, 9000, 2, 500));
        assert!(completes_multi_click(Button::X1, 9100, 2, 500));
    }

    #[test]
    fn a_third_click_does_not_fire_again() {
        assert!(!completes_multi_click(Button::X2, 100, 2, 500));
        assert!(completes_multi_click(Button::X2, 200, 2, 500));
        // Pair consumed, so the next click arms rather than fires.
        assert!(!completes_multi_click(Button::X2, 300, 2, 500));
    }

    #[test]
    fn empty_modifier_always_passes() {
        assert!(modifier_held(""));
        assert!(modifier_held("none"));
    }

    #[test]
    fn unknown_modifier_blocks_rather_than_passes() {
        // A typo in config must not silently make the trigger unconditional.
        assert!(!modifier_held("contrl"));
    }
}
