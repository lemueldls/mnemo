use std::{
    fs,
    path::{self, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, EventLoopMessage, Wry};
use tauri_plugin_store::{Store, StoreBuilder};
use time::{Date, OffsetDateTime};
use ulid::Ulid;

pub use super::DailyNote;

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyNote {
    id: Ulid,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    // datetime: (i32, Month, u8, u8, u8),
}

impl StickyNote {
    pub fn new(id: Ulid, name: String, x: f64, y: f64, width: f64, height: f64) -> Self {
        let datetime = OffsetDateTime::from(id.datetime());
        let (year, month, day) = datetime.to_calendar_date();
        let (hour, minute, _) = datetime.to_hms();

        Self {
            id,
            name,
            x,
            y,
            width,
            height,
            // datetime: (year, month, day, hour, minute),
        }
    }
}

#[tauri::command(rename_all = "camelCase")]
pub fn new_sticky_note(space_id: Ulid, app_handle: AppHandle) -> Ulid {
    let mut store = load_sticky(space_id, &app_handle);
    let id = Ulid::new();

    let note = StickyNote::new(id, String::new(), 40.0, 40.0, 500.0, 500.0);

    store.set(id.to_string(), serde_json::to_value(&note).unwrap());
    store.save().unwrap();

    id
}

#[tauri::command(rename_all = "camelCase")]
pub fn rename_sticky_note(space_id: Ulid, note_id: Ulid, name: String, app_handle: AppHandle) {
    let mut store = load_sticky(space_id, &app_handle);

    let note = store.get(note_id.to_string()).unwrap();
    let mut note = serde_json::from_value::<StickyNote>(note.clone()).unwrap();

    note.name = name;

    store.set(note_id.to_string(), serde_json::to_value(note).unwrap());
    store.save().unwrap();
}

#[tauri::command(rename_all = "camelCase")]
pub fn update_sticky_note(
    space_id: Ulid,
    note_id: Ulid,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    app_handle: AppHandle,
) {
    let mut store = load_sticky(space_id, &app_handle);

    let note = store.get(note_id.to_string()).unwrap();
    let mut note = serde_json::from_value::<StickyNote>(note.clone()).unwrap();

    note.x = x;
    note.y = y;
    note.width = width;
    note.height = height;

    store.set(note_id.to_string(), serde_json::to_value(note).unwrap());
    store.save().unwrap();
}

#[tauri::command(rename_all = "camelCase")]
pub fn delete_sticky_note(space_id: Ulid, note_id: Ulid, app_handle: AppHandle) {
    let path = crate::dir::spaces(&app_handle)
        .join(space_id.to_string())
        .join("sticky")
        .join(note_id.to_string())
        .with_extension("typ");
    // fs::remove_file(path).unwrap();

    let mut store = load_sticky(space_id, &app_handle);

    let note = store.get(note_id.to_string()).unwrap();
    let mut note = serde_json::from_value::<StickyNote>(note.clone()).unwrap();

    store.delete(note_id.to_string());
    store.save().unwrap();
}

#[tauri::command(rename_all = "camelCase")]
pub fn list_sticky_notes(space_id: Ulid, app_handle: AppHandle) -> Vec<StickyNote> {
    let store = load_sticky(space_id, &app_handle);

    let entries = store.entries();

    let mut notes = entries
        .into_iter()
        .map(|(id, note)| {
            let id = Ulid::from_string(&id).unwrap();
            let note = serde_json::from_value::<StickyNote>(note.clone()).unwrap();

            note
        })
        .collect::<Vec<_>>();

    // notes.sort_by_key(|note| note.datetime);

    notes
}

fn load_sticky(space_id: Ulid, app_handle: &AppHandle) -> Arc<Store<Wry>> {
    let path = PathBuf::from(space_id.to_string()).join("sticky.json");

    let mut store = StoreBuilder::new(app_handle, path).build().unwrap();

    // match store.load() {
    //     Ok(..) => {}
    //     Err(..) => store.save().unwrap(),
    // }

    store
}
