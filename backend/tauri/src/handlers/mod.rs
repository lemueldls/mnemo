mod space;
mod sticky;

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};
use tauri::{App, AppHandle, EventLoopMessage, Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};
use time::{Date, Month, OffsetDateTime, UtcOffset};
use ulid::Ulid;

use crate::dir;

pub use space::*;
pub use sticky::*;

// #[tauri::command(rename_all = "camelCase")]
// async pub fn close_splashscreen(window: tauri::Window) {
//     if let Some(splashscreen) = window.get_webview_window("splashscreen") {
//         splashscreen.close().unwrap();
//     }

//     let main = window.get_webview_window("main").unwrap();
//     main.show().unwrap();

//     #[cfg(any(debug_assertions, feature = "devtools"))]
//     main.open_devtools();
// }dir

// #[tauri::command(rename_all = "camelCase")]
// pub fn sync_packages() -> Vec<IndexPackageInfo> {
//     let packages = mnemo_common::fetch_packages().unwrap();

//     fs::write(
//         crate::dir::DATA_DIR.join("packages.json"),
//         serde_json::to_string(&packages).unwrap(),
//     )
//     .unwrap();

//     packages
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NoteKind {
    Daily,
    Sticky,
}

impl NoteKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            NoteKind::Daily => "daily",
            NoteKind::Sticky => "sticky",
        }
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn read_file(kind: NoteKind, space_id: Ulid, path: &str, app_handle: AppHandle) -> String {
    let spaces_dir = crate::dir::spaces(&app_handle)
        .join(space_id.to_string())
        .join(kind.as_str());
    let path = spaces_dir.join(path).with_extension("typ");

    match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(..) => {
            fs::create_dir_all(&spaces_dir).unwrap();
            fs::write(&path, "").unwrap();

            String::new()
        }
    }

    // fs::read_to_string(&path).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyNote {
    id: Ulid,
    datetime: (i32, Month, u8, u8, u8),
}

impl DailyNote {
    pub fn new(id: Ulid) -> Self {
        let datetime = OffsetDateTime::from(id.datetime());
        let (year, month, day) = datetime.to_calendar_date();
        let (hour, minute, _) = datetime.to_hms();

        Self {
            id,
            datetime: (year, month, day, hour, minute),
        }
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_daily_notes(space_id: Ulid, app_handle: AppHandle) -> Vec<DailyNote> {
    let path = dir::spaces(&app_handle)
        .join(space_id.to_string())
        .join("daily");

    let mut add_today = true;
    let today = OffsetDateTime::now_utc().date();

    let mut entries = fs::read_dir(&path)
        .unwrap()
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_stem().unwrap().to_string_lossy().to_string();

            let id = name.parse::<Ulid>().ok().unwrap();

            let note = DailyNote::new(id);

            let date = OffsetDateTime::from(id.datetime()).date();
            let is_today = date == today;

            if is_today {
                add_today = false;

                Some(note)
            } else {
                let has_content = path.metadata().unwrap().len() > 0;
                if has_content {
                    Some(note)
                } else {
                    // fs::remove_file(path).unwrap();

                    None
                }
            }
        })
        .collect::<Vec<_>>();

    if add_today {
        let id = Ulid::from_datetime(OffsetDateTime::now_utc().into());

        let path = path.join(id.to_string()).with_extension("typ");
        fs::write(path, "").unwrap();

        entries.push(DailyNote::new(id));
    }

    entries.sort_by(|a, b| b.id.cmp(&a.id));

    entries
}

#[tauri::command(rename_all = "camelCase")]
pub fn sync_file(kind: NoteKind, space_id: Ulid, path: &str, text: &str, app_handle: AppHandle) {
    let path = dir::spaces(&app_handle)
        .join(space_id.to_string())
        .join(kind.as_str())
        .join(path)
        .with_extension("typ");

    fs::write(&path, text).unwrap();
}
