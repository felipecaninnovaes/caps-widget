//! Rules for "Start with Windows": whether it is on, and how to toggle it.
//!
//! The actual storage (the registry `Run` key) sits behind [`AutostartStore`],
//! so these rules are testable without touching the system.

use std::io;
use std::path::Path;

/// The single per-user value that makes the app start at logon.
pub trait AutostartStore {
    /// The stored command line, or `None` if there is none.
    fn read(&self) -> io::Result<Option<String>>;
    fn write(&mut self, command: &str) -> io::Result<()>;
    fn delete(&mut self) -> io::Result<()>;
}

/// Command line stored for `exe`: the path, quoted (it may contain spaces).
pub fn command_for(exe: &Path) -> String {
    format!("\"{}\"", exe.display())
}

/// On only if the stored command points at `exe` — a value left behind by a
/// moved executable counts as off.
pub fn is_enabled(store: &impl AutostartStore, exe: &Path) -> io::Result<bool> {
    Ok(store
        .read()?
        .is_some_and(|stored| same_path(&stored, &exe.to_string_lossy())))
}

/// Turns autostart on or off for `exe`. Does nothing if already in that state.
///
/// On error the stored value is whatever the store left; callers should
/// re-read it with [`is_enabled`] rather than assume the requested state.
pub fn set_enabled(store: &mut impl AutostartStore, exe: &Path, enabled: bool) -> io::Result<()> {
    if is_enabled(store, exe)? == enabled {
        return Ok(());
    }
    if enabled {
        store.write(&command_for(exe))
    } else {
        store.delete()
    }
}

/// Windows paths are case-insensitive; quotes and surrounding blanks are not
/// part of the path.
fn same_path(stored: &str, exe: &str) -> bool {
    let normalize = |s: &str| s.trim().trim_matches('"').to_lowercase();
    normalize(stored) == normalize(exe)
}
