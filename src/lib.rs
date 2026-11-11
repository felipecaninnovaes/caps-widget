//! Core, platform-independent logic for caps-widget.
//!
//! Platform adapters (Win32 key state, tray icon) feed a [`LockState`] in and
//! render what it describes; nothing here touches the OS.

/// Snapshot of the lock keys the widget reports on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LockState {
    pub caps: bool,
    pub num: bool,
}

impl LockState {
    /// Human-readable summary used as the tray tooltip.
    pub fn tooltip(self) -> String {
        format!(
            "Caps Lock: {} | Num Lock: {}",
            on_off(self.caps),
            on_off(self.num)
        )
    }
}

fn on_off(enabled: bool) -> &'static str {
    if enabled { "on" } else { "off" }
}
