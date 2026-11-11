//! Reads the Caps Lock and Num Lock toggle state.
//!
//! Deliberately polls just these two keys instead of installing a keyboard
//! hook: the process never sees what is typed (R3).

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, VIRTUAL_KEY, VK_CAPITAL, VK_NUMLOCK,
};

use crate::LockState;

/// Every key this module ever queries.
pub const QUERIED_KEYS: [VIRTUAL_KEY; 2] = [VK_CAPITAL, VK_NUMLOCK];

/// Current toggle state of Caps Lock and Num Lock.
pub fn read() -> LockState {
    let [caps, num] = QUERIED_KEYS.map(is_toggled);
    LockState { caps, num }
}

fn is_toggled(key: VIRTUAL_KEY) -> bool {
    // SAFETY: GetKeyState takes a plain virtual-key code and has no pointer
    // arguments or other preconditions.
    let state = unsafe { GetKeyState(i32::from(key)) };
    state & 1 != 0
}
