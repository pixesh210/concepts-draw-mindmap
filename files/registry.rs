//! Windows registry read helpers for Concepts Draw Mindmap.

pub const APP_KEY: &str = "Software\\Concepts\\Concepts Draw Mindmap";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Concepts Draw Mindmap";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Concepts Draw Mindmap") },
        RegValue { name: "Version",     value: String::from("v4.7.16") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Concepts Draw Mindmap") },
        RegValue { name: "Publisher",       value: String::from("Concepts") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Concepts Draw Mindmap\\uninstall.exe\"") },
    ]
}
