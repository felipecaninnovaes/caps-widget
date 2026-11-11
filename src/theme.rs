//! Taskbar color theme, which decides the icon's foreground color.

/// Taskbar theme. Unknown themes fall back to [`Theme::Dark`], the Windows default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

impl Theme {
    /// Maps the `SystemUsesLightTheme` registry flag (absent → `None`) to a theme.
    pub fn from_light_flag(flag: Option<u32>) -> Theme {
        match flag {
            Some(0) | None => Theme::Dark,
            Some(_) => Theme::Light,
        }
    }
}
