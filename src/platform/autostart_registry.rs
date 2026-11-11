//! [`AutostartStore`] backed by a value under `HKEY_CURRENT_USER`.

use std::io;

use super::registry;
use crate::autostart::AutostartStore;

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "caps-widget";

/// One named value under an `HKEY_CURRENT_USER` subkey.
#[derive(Debug, Clone)]
pub struct RegistryRunKey {
    subkey: String,
    value_name: String,
}

impl RegistryRunKey {
    /// The per-user `Run` key, which Windows reads at logon (no admin needed).
    pub fn current_user_run() -> Self {
        Self::new(RUN_KEY, VALUE_NAME)
    }

    /// Any `HKCU` subkey — lets tests use a throwaway key.
    pub fn new(subkey: &str, value_name: &str) -> Self {
        Self {
            subkey: subkey.to_owned(),
            value_name: value_name.to_owned(),
        }
    }

    pub fn subkey(&self) -> &str {
        &self.subkey
    }

    pub fn value_name(&self) -> &str {
        &self.value_name
    }
}

impl AutostartStore for RegistryRunKey {
    fn read(&self) -> io::Result<Option<String>> {
        registry::get_string(&self.subkey, &self.value_name)
    }

    fn write(&mut self, command: &str) -> io::Result<()> {
        registry::set_string(&self.subkey, &self.value_name, command)
    }

    fn delete(&mut self) -> io::Result<()> {
        registry::delete(&self.subkey, &self.value_name)
    }
}
