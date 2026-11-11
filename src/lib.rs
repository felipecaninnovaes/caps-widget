//! Core, platform-independent logic for caps-widget.
//!
//! Platform adapters (Win32 key state, tray icon) feed a [`LockState`] in and
//! render what it describes; nothing in the core modules touches the OS.

pub mod i18n;
pub mod state;

pub use i18n::{Lang, Strings};
pub use state::{ChangeDetector, LockState, POLL_INTERVAL};
