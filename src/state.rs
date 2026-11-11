//! Lock key state and how it is described to the user.

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
