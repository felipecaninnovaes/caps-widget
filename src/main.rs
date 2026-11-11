// No console window in any build (R11).
#![cfg_attr(windows, windows_subsystem = "windows")]

use std::process::ExitCode;

#[cfg(windows)]
fn main() -> ExitCode {
    use caps_widget::platform::{dpi, instance, tray};

    // Must happen before any window exists, or the icon renders blurry.
    dpi::enable_per_monitor_awareness();

    let _guard = match instance::acquire() {
        Ok(Some(guard)) => guard,
        // Another instance owns the tray icon: leave silently (R8).
        Ok(None) => return ExitCode::SUCCESS,
        Err(_) => return ExitCode::FAILURE,
    };
    match tray::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

#[cfg(not(windows))]
fn main() -> ExitCode {
    eprintln!("caps-widget only runs on Windows.");
    ExitCode::FAILURE
}
