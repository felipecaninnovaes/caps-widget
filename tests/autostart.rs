use std::io;
use std::path::Path;

use caps_widget::autostart::{self, AutostartStore};

const EXE: &str = r"C:\Tools\caps-widget.exe";

/// In-memory stand-in for the registry `Run` value; counts writes.
#[derive(Default)]
struct FakeStore {
    value: Option<String>,
    writes: usize,
    fail_writes: bool,
}

impl AutostartStore for FakeStore {
    fn read(&self) -> io::Result<Option<String>> {
        Ok(self.value.clone())
    }

    fn write(&mut self, command: &str) -> io::Result<()> {
        if self.fail_writes {
            return Err(io::Error::from(io::ErrorKind::PermissionDenied));
        }
        self.writes += 1;
        self.value = Some(command.to_owned());
        Ok(())
    }

    fn delete(&mut self) -> io::Result<()> {
        if self.fail_writes {
            return Err(io::Error::from(io::ErrorKind::PermissionDenied));
        }
        self.writes += 1;
        self.value = None;
        Ok(())
    }
}

fn exe() -> &'static Path {
    Path::new(EXE)
}

#[test]
fn disabled_by_default() {
    let store = FakeStore::default();
    assert!(!autostart::is_enabled(&store, exe()).unwrap());
}

#[test]
fn enable_then_disable() {
    let mut store = FakeStore::default();

    autostart::set_enabled(&mut store, exe(), true).unwrap();
    assert_eq!(
        store.value.as_deref(),
        Some(r#""C:\Tools\caps-widget.exe""#)
    );
    assert!(autostart::is_enabled(&store, exe()).unwrap());

    autostart::set_enabled(&mut store, exe(), false).unwrap();
    assert_eq!(store.value, None);
    assert!(!autostart::is_enabled(&store, exe()).unwrap());
}

#[test]
fn path_comparison_ignores_case_and_quotes() {
    for stored in [
        r"C:\Tools\caps-widget.exe",
        r#""c:\tools\CAPS-WIDGET.EXE""#,
        r#"  "C:\Tools\caps-widget.exe"  "#,
    ] {
        let store = FakeStore {
            value: Some(stored.to_owned()),
            ..FakeStore::default()
        };
        assert!(autostart::is_enabled(&store, exe()).unwrap(), "{stored:?}");
    }
}

#[test]
fn moved_exe_reads_as_disabled() {
    let mut store = FakeStore {
        value: Some(r#""D:\Old\caps-widget.exe""#.to_owned()),
        ..FakeStore::default()
    };
    assert!(!autostart::is_enabled(&store, exe()).unwrap());

    autostart::set_enabled(&mut store, exe(), true).unwrap();
    assert!(autostart::is_enabled(&store, exe()).unwrap());
}

#[test]
fn idempotent() {
    let mut store = FakeStore::default();
    autostart::set_enabled(&mut store, exe(), false).unwrap();
    assert_eq!(store.writes, 0, "disabling when already off writes nothing");

    autostart::set_enabled(&mut store, exe(), true).unwrap();
    autostart::set_enabled(&mut store, exe(), true).unwrap();
    assert_eq!(store.writes, 1, "enabling when already on writes nothing");
    assert!(autostart::is_enabled(&store, exe()).unwrap());
}

#[test]
fn store_failure_keeps_disabled() {
    let mut store = FakeStore {
        fail_writes: true,
        ..FakeStore::default()
    };
    assert!(autostart::set_enabled(&mut store, exe(), true).is_err());
    assert!(!autostart::is_enabled(&store, exe()).unwrap());
}
