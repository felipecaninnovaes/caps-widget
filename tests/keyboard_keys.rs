#![cfg(windows)]

use caps_widget::platform::keyboard::QUERIED_KEYS;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{VK_CAPITAL, VK_NUMLOCK};

#[test]
fn queries_only_caps_and_num() {
    assert_eq!(QUERIED_KEYS, [VK_CAPITAL, VK_NUMLOCK]);
}

#[test]
fn reading_the_keys_does_not_panic() {
    let _ = caps_widget::platform::keyboard::read();
}
