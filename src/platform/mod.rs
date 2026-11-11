//! Win32 adapters. The only place in the crate that calls the OS or uses `unsafe`.

pub mod autostart_registry;
pub mod keyboard;
pub mod locale;
mod registry;
pub mod theme;

/// NUL-terminated UTF-16 copy of `s`, for `PCWSTR` arguments.
pub(crate) fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
