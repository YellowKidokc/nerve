#Requires AutoHotkey v2.0
#SingleInstance Force
; ============================================================================
; nerve TTS shim — CapsLock hotkeys for nerve's speech engine
; ============================================================================
;
; Windows will not register CapsLock as a hotkey modifier: RegisterHotKey, which
; nerve uses for every other binding, only accepts Ctrl/Alt/Shift/Win. Only a
; low-level keyboard hook can claim CapsLock, and AutoHotkey already installs
; one for the rest of this setup.
;
; So rather than duplicating that hook inside nerve, this shim owns the CapsLock
; chords and forwards each one to the ordinary nerve hotkey behind it. nerve does
; all the real work — copying the selection, chunking, speaking, interrupting.
; This file is only a key translation, which is why it is this short.
;
; Setup: the CapsLock hotkeys in BetterTTS must be disabled first, or both
; scripts will fight for the same chord. See modules\bettertts_tab.ahk.
; ============================================================================

; nerve's corresponding bindings, from its config.json "hotkeys" list.
; Change these here if you rebind them in nerve.
NERVE_READ  := "^!t"    ; Ctrl+Alt+T  → tts_read_selection
NERVE_STOP  := "^!{F8}" ; Ctrl+Alt+F8 → tts_stop
NERVE_PAUSE := "^!{F7}" ; Ctrl+Alt+F7 → tts_pause

; CapsLock alone stays neutral: without this, every chord below would also
; toggle capitals, and a stuck CapsLock is worse than no hotkey at all.
SetCapsLockState("AlwaysOff")
CapsLock::SetCapsLockState("AlwaysOff")

; Read the selection aloud. nerve copies it itself and interrupts whatever is
; already speaking, so this may be pressed repeatedly without waiting.
CapsLock & c:: {
    global NERVE_READ
    SetCapsLockState("AlwaysOff")
    Send(NERVE_READ)
}

CapsLock & s:: {
    global NERVE_STOP
    SetCapsLockState("AlwaysOff")
    Send(NERVE_STOP)
}

CapsLock & p:: {
    global NERVE_PAUSE
    SetCapsLockState("AlwaysOff")
    Send(NERVE_PAUSE)
}
