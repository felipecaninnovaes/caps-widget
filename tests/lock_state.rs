use caps_widget::LockState;

#[test]
fn tooltip_reports_each_key_independently() {
    let state = LockState {
        caps: true,
        num: false,
    };
    assert_eq!(state.tooltip(), "Caps Lock: on | Num Lock: off");
}

#[test]
fn default_state_is_all_off() {
    assert_eq!(
        LockState::default().tooltip(),
        "Caps Lock: off | Num Lock: off"
    );
}
