#![cfg(windows)]

use std::path::Path;
use std::process::Command;

use caps_widget::autostart::{self, AutostartStore};
use caps_widget::platform::autostart_registry::RegistryRunKey;

/// Removes the temporary key even if the test panics.
struct TempKey(String);

impl Drop for TempKey {
    fn drop(&mut self) {
        let _ = Command::new("reg")
            .args(["delete", &format!(r"HKCU\{}", self.0), "/f"])
            .output();
    }
}

#[test]
fn round_trip_in_temp_key() {
    let key = TempKey(format!(r"Software\caps-widget-test-{}", std::process::id()));
    let mut store = RegistryRunKey::new(&key.0, "caps-widget");
    let exe = Path::new(r"C:\Program Files\caps widget\caps-widget.exe");

    assert_eq!(store.read().unwrap(), None);
    assert!(!autostart::is_enabled(&store, exe).unwrap());

    autostart::set_enabled(&mut store, exe, true).unwrap();
    assert_eq!(
        store.read().unwrap().as_deref(),
        Some(r#""C:\Program Files\caps widget\caps-widget.exe""#)
    );
    assert!(autostart::is_enabled(&store, exe).unwrap());

    autostart::set_enabled(&mut store, exe, false).unwrap();
    assert_eq!(store.read().unwrap(), None);
    // Deleting a missing value is not an error.
    store.delete().unwrap();
}

#[test]
fn run_key_points_at_the_user_run_key() {
    let store = RegistryRunKey::current_user_run();
    assert_eq!(
        store.subkey(),
        r"Software\Microsoft\Windows\CurrentVersion\Run"
    );
    assert_eq!(store.value_name(), "caps-widget");
}
