//! Reads the Windows UI language.

use windows_sys::Win32::Globalization::{GetUserDefaultUILanguage, LCIDToLocaleName};

use crate::Lang;

/// `LOCALE_NAME_MAX_LENGTH` from winnls.h, in UTF-16 units including the NUL.
const LOCALE_NAME_MAX_LENGTH: usize = 85;

/// BCP-47 tag of the user's UI language (e.g. `pt-BR`), or empty if unknown.
pub fn ui_locale_tag() -> String {
    // SAFETY: no arguments, no preconditions.
    let lang_id = unsafe { GetUserDefaultUILanguage() };
    let mut buf = [0u16; LOCALE_NAME_MAX_LENGTH];
    // SAFETY: `buf` is writable for `buf.len()` UTF-16 units. A LANGID widened
    // to u32 is MAKELCID(lang, SORT_DEFAULT), a valid LCID.
    let len =
        unsafe { LCIDToLocaleName(u32::from(lang_id), buf.as_mut_ptr(), buf.len() as i32, 0) };
    // `len` counts the terminating NUL; 0 means failure.
    match usize::try_from(len) {
        Ok(n) if n > 1 => String::from_utf16_lossy(&buf[..n - 1]),
        _ => String::new(),
    }
}

/// UI language for user-facing strings.
pub fn ui_lang() -> Lang {
    Lang::from_locale_tag(&ui_locale_tag())
}
