mod dir;

use std::{collections::BTreeMap, fs, time::SystemTime};

use mnemo_common::bundler::model::IndexPackageInfo;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            sync_packages,
            read_dir,
            read_file,
            sync_file,
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    if let Some(splashscreen) = window.get_webview_window("splashscreen") {
        splashscreen.close().unwrap();
    }

    let main = window.get_webview_window("main").unwrap();
    main.show().unwrap();

    #[cfg(any(debug_assertions, feature = "devtools"))]
    main.open_devtools();
}

#[tauri::command]
fn sync_packages() -> Vec<IndexPackageInfo> {
    let packages = mnemo_common::fetch_packages().unwrap();

    fs::write(
        crate::dir::DATA_DIR.join("packages.json"),
        serde_json::to_string(&packages).unwrap(),
    )
    .unwrap();

    packages
}

#[tauri::command]
fn read_file(space: &str, path: &str) -> String {
    let path = crate::dir::spaces().join(space).join(path);

    match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(..) => {
            fs::write(&path, "").unwrap();

            String::new()
        }
    }
}

#[tauri::command]
fn read_dir(space: &str) -> Vec<(u128, String)> {
    let path = dir::spaces().join(space);

    match fs::read_dir(&path) {
        Ok(dir) => {
            let mut entries = dir
                .filter_map(|result| {
                    let result = result.unwrap();

                    let has_content = match fs::File::open(result.path()) {
                        Ok(file) => file.metadata().unwrap().len() > 0,
                        Err(..) => false,
                    };

                    if !has_content {
                        fs::remove_file(result.path()).unwrap();

                        return None;
                    }

                    let file_name = result.file_name().to_string_lossy().to_string();

                    let date = result.metadata().unwrap().created().unwrap();
                    let date = date
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    Some((date, file_name))
                })
                .collect::<Vec<_>>();

            entries.sort();

            entries
        }
        Err(..) => {
            fs::create_dir_all(path).unwrap();

            Vec::new()
        }
    }
}

#[tauri::command]
fn sync_file(space: &str, path: &str, text: &str) {
    let path = crate::dir::spaces().join(space).join(path);

    fs::write(path, text).unwrap();
}
