//! One instance per user session (R8).

use std::io;
use std::ptr;

use windows_sys::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, HANDLE};
use windows_sys::Win32::System::Threading::CreateMutexW;

use super::wide;

/// `Local\` = per session. Part of the contract between versions: changing it
/// would let two different versions run side by side.
const MUTEX_NAME: &str = r"Local\caps-widget-single-instance";

/// Held for the whole process lifetime; the OS releases it on exit.
pub struct InstanceGuard(HANDLE);

impl Drop for InstanceGuard {
    fn drop(&mut self) {
        // SAFETY: the handle came from CreateMutexW and is closed only here.
        unsafe { CloseHandle(self.0) };
    }
}

/// `Ok(Some(guard))` if this is the first instance in the session,
/// `Ok(None)` if another one is already running.
pub fn acquire() -> io::Result<Option<InstanceGuard>> {
    let name = wide(MUTEX_NAME);
    // SAFETY: `name` is NUL-terminated and outlives the call; null security
    // attributes mean the default descriptor.
    let handle = unsafe { CreateMutexW(ptr::null(), 0, name.as_ptr()) };
    // SAFETY: no preconditions; must run right after CreateMutexW.
    let last_error = unsafe { GetLastError() };
    if handle.is_null() {
        return Err(io::Error::from_raw_os_error(last_error as i32));
    }
    let guard = InstanceGuard(handle);
    if last_error == ERROR_ALREADY_EXISTS {
        return Ok(None);
    }
    Ok(Some(guard))
}
