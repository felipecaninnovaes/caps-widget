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

#[test]
fn small_icon_size_follows_dpi() {
    use caps_widget::platform::dpi::small_icon_size;
    assert_eq!(small_icon_size(96), 16);
    assert_eq!(small_icon_size(144), 24);
    assert_eq!(small_icon_size(192), 32);
}
