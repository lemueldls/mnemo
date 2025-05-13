use std::{fs, path::PathBuf};

use once_cell::sync::Lazy;
use tauri::{AppHandle, Manager, path::BaseDirectory};

pub fn spaces(app_handle: &AppHandle) -> PathBuf {
    app_handle.path().app_data_dir().unwrap().join("spaces")
}
