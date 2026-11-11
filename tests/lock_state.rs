use caps_widget::{Lang, LockState};

#[test]
fn tooltip_reports_each_key_independently() {
    let state = LockState {
        caps: false,
        num: true,
    };
    assert_eq!(state.tooltip(Lang::En), "Caps Lock: off | Num Lock: on");
    assert_eq!(
        state.tooltip(Lang::Pt),
        "Caps Lock: desligado | Num Lock: ligado"
    );
}

#[test]
fn tooltip_default_state_is_all_off() {
    assert_eq!(
        LockState::default().tooltip(Lang::En),
        "Caps Lock: off | Num Lock: off"
    );
}
