//! Windows registry read helpers for Jetbrains Aqua.

pub const APP_KEY: &str = "Software\\Jetbrains\\Jetbrains Aqua";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Jetbrains Aqua";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Jetbrains Aqua") },
        RegValue { name: "Version",     value: String::from("v1.0.4") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Jetbrains Aqua") },
        RegValue { name: "Publisher",       value: String::from("Jetbrains") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Jetbrains Aqua\\uninstall.exe\"") },
    ]
}
