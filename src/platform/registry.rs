//! Minimal `HKEY_CURRENT_USER` access: read, write and delete one value.

use std::io;
use std::ptr;

use windows_sys::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_MORE_DATA, ERROR_SUCCESS};
use windows_sys::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_OPTION_NON_VOLATILE, REG_ROUTINE_FLAGS, REG_SZ,
    RRF_RT_REG_DWORD, RRF_RT_REG_SZ, RegCloseKey, RegCreateKeyExW, RegDeleteKeyValueW,
    RegGetValueW, RegSetValueExW,
};

use super::wide;

fn os_error(code: u32) -> io::Error {
    io::Error::from_raw_os_error(code as i32)
}

/// Raw bytes of `HKCU\subkey\name`, or `None` if the key or value is missing.
fn get(subkey: &str, name: &str, flags: REG_ROUTINE_FLAGS) -> io::Result<Option<Vec<u8>>> {
    let subkey = wide(subkey);
    let name = wide(name);
    let mut buf: Vec<u8> = Vec::new();
    loop {
        let mut size = u32::try_from(buf.len()).map_err(|_| os_error(ERROR_MORE_DATA))?;
        let data = if buf.is_empty() {
            ptr::null_mut()
        } else {
            buf.as_mut_ptr().cast()
        };
        // SAFETY: `subkey` and `name` are NUL-terminated and outlive the call;
        // `data` is either null (size query) or points to `size` writable bytes.
        let err = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                name.as_ptr(),
                flags,
                ptr::null_mut(),
                data,
                &mut size,
            )
        };
        match err {
            ERROR_SUCCESS if !buf.is_empty() || size == 0 => {
                buf.truncate(size as usize);
                return Ok(Some(buf));
            }
            // Size query answered, or the value grew in between: retry with room.
            ERROR_SUCCESS | ERROR_MORE_DATA => buf = vec![0; size as usize],
            ERROR_FILE_NOT_FOUND => return Ok(None),
            other => return Err(os_error(other)),
        }
    }
}

pub fn get_dword(subkey: &str, name: &str) -> io::Result<Option<u32>> {
    Ok(get(subkey, name, RRF_RT_REG_DWORD)?
        .and_then(|bytes| Some(u32::from_le_bytes(bytes.get(..4)?.try_into().ok()?))))
}

pub fn get_string(subkey: &str, name: &str) -> io::Result<Option<String>> {
    Ok(get(subkey, name, RRF_RT_REG_SZ)?.map(|bytes| {
        let units: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .take_while(|&u| u != 0)
            .collect();
        String::from_utf16_lossy(&units)
    }))
}

/// Writes a `REG_SZ` value, creating `subkey` if needed.
pub fn set_string(subkey: &str, name: &str, value: &str) -> io::Result<()> {
    let subkey = wide(subkey);
    let name = wide(name);
    let data = wide(value);
    let bytes = u32::try_from(data.len() * 2).map_err(|_| os_error(ERROR_MORE_DATA))?;

    let mut key: HKEY = ptr::null_mut();
    // SAFETY: `subkey` is NUL-terminated; `key` receives a handle closed below.
    let err = unsafe {
        RegCreateKeyExW(
            HKEY_CURRENT_USER,
            subkey.as_ptr(),
            0,
            ptr::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE,
            ptr::null(),
            &mut key,
            ptr::null_mut(),
        )
    };
    if err != ERROR_SUCCESS {
        return Err(os_error(err));
    }
    // SAFETY: `key` is open with KEY_SET_VALUE; `data` holds `bytes` bytes,
    // including the terminating NUL that REG_SZ requires.
    let err = unsafe { RegSetValueExW(key, name.as_ptr(), 0, REG_SZ, data.as_ptr().cast(), bytes) };
    // SAFETY: `key` was opened above and is not used afterwards.
    unsafe { RegCloseKey(key) };
    if err != ERROR_SUCCESS {
        return Err(os_error(err));
    }
    Ok(())
}

/// Deletes a value; a missing key or value is not an error.
pub fn delete(subkey: &str, name: &str) -> io::Result<()> {
    let subkey = wide(subkey);
    let name = wide(name);
    // SAFETY: both strings are NUL-terminated and outlive the call.
    let err = unsafe { RegDeleteKeyValueW(HKEY_CURRENT_USER, subkey.as_ptr(), name.as_ptr()) };
    match err {
        ERROR_SUCCESS | ERROR_FILE_NOT_FOUND => Ok(()),
        other => Err(os_error(other)),
    }
}
