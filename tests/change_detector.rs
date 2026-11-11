use std::time::Duration;

use caps_widget::{ChangeDetector, LockState, POLL_INTERVAL};

const OFF: LockState = LockState {
    caps: false,
    num: false,
};
const CAPS: LockState = LockState {
    caps: true,
    num: false,
};

#[test]
fn first_reading_is_reported() {
    let mut detector = ChangeDetector::default();
    assert_eq!(detector.observe(OFF), Some(OFF));
}

#[test]
fn emits_once_per_change() {
    let mut detector = ChangeDetector::default();
    detector.observe(OFF);

    let emitted: Vec<_> = [CAPS, CAPS, CAPS]
        .into_iter()
        .filter_map(|s| detector.observe(s))
        .collect();
    assert_eq!(emitted, vec![CAPS]);
    assert_eq!(detector.observe(OFF), Some(OFF));
}

#[test]
fn repeated_reading_emits_nothing() {
    let mut detector = ChangeDetector::default();
    detector.observe(CAPS);
    for _ in 0..10 {
        assert_eq!(detector.observe(CAPS), None);
    }
}

#[test]
fn poll_interval_meets_latency_target() {
    assert!(POLL_INTERVAL > Duration::ZERO);
    assert!(POLL_INTERVAL <= Duration::from_millis(50));
}
