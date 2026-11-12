//! Windows registry read helpers for Waves Renaissance Eq.

pub const APP_KEY: &str = "Software\\Waves\\Waves Renaissance Eq";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Waves Renaissance Eq";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Waves Renaissance Eq") },
        RegValue { name: "Version",     value: String::from("v1.2.13") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Waves Renaissance Eq") },
        RegValue { name: "Publisher",       value: String::from("Waves") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Waves Renaissance Eq\\uninstall.exe\"") },
    ]
}
