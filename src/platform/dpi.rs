//! DPI awareness, so the tray icon is drawn at its real pixel size.

use windows_sys::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext,
};

/// Best effort: on failure Windows bitmap-scales the icon, which only blurs it.
pub fn enable_per_monitor_awareness() {
    // SAFETY: called once at startup, before any window is created.
    unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
}
