use std::{fs, path::PathBuf};

use once_cell::sync::Lazy;

pub static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = dirs::data_dir().unwrap().join("mnemo");
    fs::create_dir_all(&path).unwrap();

    path
});

pub static SPACES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = DATA_DIR.join("spaces");
    fs::create_dir_all(&path).unwrap();

    path
});

pub fn spaces() -> &'static PathBuf {
    &SPACES_DIR
}
