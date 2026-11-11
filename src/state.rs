//! Lock key state and how it is described to the user.

use std::time::Duration;

use crate::i18n::Lang;

/// Snapshot of the lock keys the widget reports on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LockState {
    pub caps: bool,
    pub num: bool,
}

impl LockState {
    /// Human-readable summary used as the tray tooltip, in `lang`.
    pub fn tooltip(self, lang: Lang) -> String {
        let s = lang.strings();
        let word = |enabled: bool| if enabled { s.on } else { s.off };
        format!(
            "Caps Lock: {} | Num Lock: {}",
            word(self.caps),
            word(self.num)
        )
    }
}

/// How often the platform adapter reads the lock keys.
///
/// Half of the 100 ms latency target (R2), leaving room for timer jitter.
pub const POLL_INTERVAL: Duration = Duration::from_millis(50);

/// Turns a stream of polled readings into a stream of changes.
#[derive(Debug, Default)]
pub struct ChangeDetector {
    last: Option<LockState>,
}

impl ChangeDetector {
    /// Returns `Some(state)` when `state` differs from the previous reading
    /// (or is the first one), `None` otherwise.
    pub fn observe(&mut self, state: LockState) -> Option<LockState> {
        if self.last == Some(state) {
            return None;
        }
        self.last = Some(state);
        Some(state)
    }
}
