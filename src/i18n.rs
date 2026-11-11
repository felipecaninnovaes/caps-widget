//! User-facing strings in the supported UI languages.

/// UI language. Portuguese for any `pt-*` Windows UI language, English otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    Pt,
    #[default]
    En,
}

/// Every user-visible string, for one language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Strings {
    pub on: &'static str,
    pub off: &'static str,
    pub start_with_windows: &'static str,
    pub exit: &'static str,
}

const PT: Strings = Strings {
    on: "ligado",
    off: "desligado",
    start_with_windows: "Iniciar com o Windows",
    exit: "Sair",
};

const EN: Strings = Strings {
    on: "on",
    off: "off",
    start_with_windows: "Start with Windows",
    exit: "Exit",
};

impl Lang {
    /// Maps a BCP-47 locale tag (e.g. `pt-BR`) to a UI language.
    ///
    /// Only the primary subtag matters; anything unknown or empty is English.
    pub fn from_locale_tag(tag: &str) -> Lang {
        let primary = tag.split(['-', '_']).next().unwrap_or_default();
        if primary.eq_ignore_ascii_case("pt") {
            Lang::Pt
        } else {
            Lang::En
        }
    }

    pub fn strings(self) -> &'static Strings {
        match self {
            Lang::Pt => &PT,
            Lang::En => &EN,
        }
    }
}
