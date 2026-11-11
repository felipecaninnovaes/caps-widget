//! DPI awareness, so the tray icon is drawn at its real pixel size.

use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, GetDpiForWindow, GetSystemMetricsForDpi,
    SetProcessDpiAwarenessContext,
};
use windows_sys::Win32::UI::WindowsAndMessaging::SM_CXSMICON;

/// 100% scaling.
const DEFAULT_DPI: u32 = 96;

/// Best effort: on failure Windows bitmap-scales the icon, which only blurs it.
pub fn enable_per_monitor_awareness() {
    // SAFETY: called once at startup, before any window is created.
    unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
}

/// Current DPI of `hwnd`'s monitor; 96 if it cannot be determined.
pub(crate) fn window_dpi(hwnd: HWND) -> u32 {
    // SAFETY: returns 0 for an invalid handle, never faults.
    match unsafe { GetDpiForWindow(hwnd) } {
        0 => DEFAULT_DPI,
        dpi => dpi,
    }
}

/// Small-icon (tray) size in pixels at `dpi`.
///
/// Unlike `GetSystemMetrics`, which in a per-monitor-aware process stays at
/// the DPI the session started with, this follows scale changes.
pub fn small_icon_size(dpi: u32) -> u32 {
    // SAFETY: plain integer arguments, no preconditions.
    let size = unsafe { GetSystemMetricsForDpi(SM_CXSMICON, dpi) };
    size.clamp(16, 256) as u32
}
