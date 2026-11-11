//! Guard for R3: the app must never see keystrokes, only lock key toggles.

use std::fs;
use std::path::Path;

/// APIs that would let the process observe every key typed system-wide.
const FORBIDDEN: [&str; 3] = [
    "SetWindowsHookEx",
    "RegisterRawInputDevices",
    "GetAsyncKeyState",
];

fn rust_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn source_has_no_keyboard_hook() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    for dir in ["src", "examples"] {
        let dir = root.join(dir);
        if dir.is_dir() {
            rust_files(&dir, &mut files);
        }
    }
    assert!(!files.is_empty());
    for file in files {
        let text = fs::read_to_string(&file).unwrap();
        for api in FORBIDDEN {
            assert!(!text.contains(api), "{} uses {api}", file.display());
        }
    }
}
