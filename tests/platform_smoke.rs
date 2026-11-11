#![cfg(windows)]

use caps_widget::platform::{locale, theme};

#[test]
fn locale_tag_not_empty() {
    let tag = locale::ui_locale_tag();
    assert!(!tag.is_empty());
    assert!(tag.contains('-'), "expected a BCP-47 tag, got {tag:?}");
}

#[test]
fn theme_reads_without_panic() {
    let _ = theme::taskbar_theme();
}
