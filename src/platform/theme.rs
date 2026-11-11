//! Reads the taskbar theme, which is what the tray icon sits on.

use super::registry;
use crate::Theme;

const PERSONALIZE_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
/// Taskbar/Start theme. `AppsUseLightTheme` is for app windows, not the tray.
const LIGHT_FLAG: &str = "SystemUsesLightTheme";

/// Current taskbar theme; dark if it cannot be read.
pub fn taskbar_theme() -> Theme {
    Theme::from_light_flag(registry::get_dword(PERSONALIZE_KEY, LIGHT_FLAG).unwrap_or(None))
}
