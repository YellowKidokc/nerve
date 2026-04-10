use tao::window::Window;
#[allow(unused_imports)]
use tracing::warn;

/// Apply dark title bar to a window using DWM (Windows 11+)
pub fn set_dark_titlebar(window: &Window) {
    #[cfg(target_os = "windows")]
    {
        use tao::platform::windows::WindowExtWindows;
        use windows::Win32::Foundation::BOOL;
        use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE};

        let hwnd = window.hwnd();
        let hwnd = windows::Win32::Foundation::HWND(hwnd as *mut _);
        let value = BOOL::from(true);

        unsafe {
            let result = DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &value as *const BOOL as *const _,
                std::mem::size_of::<BOOL>() as u32,
            );
            if result.is_err() {
                warn!("Failed to set dark title bar");
            }
        }
    }
}

/// Set a window to always-on-top
pub fn set_always_on_top(window: &Window, on_top: bool) {
    window.set_always_on_top(on_top);
}

/// Get window position for saving
pub fn get_window_position(window: &Window) -> Option<(i32, i32)> {
    match window.outer_position() {
        Ok(pos) => Some((pos.x, pos.y)),
        Err(_) => None,
    }
}

/// Pin/unpin an external window to always-on-top.
/// Waits `delay_ms` for the user to click on a target window, then pins it.
/// Returns the window title on success.
pub fn pin_external_window(delay_ms: u64) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetForegroundWindow, GetWindowTextW, SetWindowPos,
            HWND_TOPMOST, HWND_NOTOPMOST, SWP_NOMOVE, SWP_NOSIZE,
            GetWindowLongW, GWL_EXSTYLE, WS_EX_TOPMOST,
        };

        // Wait for user to click on the target window
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));

        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0.is_null() {
                return None;
            }

            // Get window title
            let mut buf = [0u16; 256];
            let len = GetWindowTextW(hwnd, &mut buf);
            let title = String::from_utf16_lossy(&buf[..len as usize]);

            // Check if already topmost
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
            let is_topmost = (ex_style & WS_EX_TOPMOST.0) != 0;

            if is_topmost {
                // Unpin it
                let _ = SetWindowPos(
                    hwnd,
                    HWND_NOTOPMOST,
                    0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE,
                );
                Some(format!("UNPINNED: {}", title))
            } else {
                // Pin it
                let _ = SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE,
                );
                Some(format!("PINNED: {}", title))
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}
