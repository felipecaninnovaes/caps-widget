//! Manual check for the main risk of spec 0001: does polling see lock key
//! changes while another window has focus? Prints each change, for ~`secs`
//! seconds (first argument, default 30).

#[cfg(windows)]
fn main() {
    use caps_widget::{ChangeDetector, Lang, POLL_INTERVAL, platform::keyboard};
    use std::time::{Duration, Instant};

    let secs = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let deadline = Instant::now() + Duration::from_secs(secs);
    let mut detector = ChangeDetector::default();
    while Instant::now() < deadline {
        if let Some(state) = detector.observe(keyboard::read()) {
            println!("{}", state.tooltip(Lang::En));
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

#[cfg(not(windows))]
fn main() {
    eprintln!("poll_keys only runs on Windows");
}
