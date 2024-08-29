mod dir;
mod handlers;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // handlers::close_splashscreen,
            // handlers::sync_packages,
            handlers::create_space,
            handlers::list_spaces,
            // handlers::get_recent_notes,
            handlers::get_daily_notes,
            handlers::new_sticky_note,
            handlers::rename_sticky_note,
            handlers::update_sticky_note,
            handlers::delete_sticky_note,
            handlers::list_sticky_notes,
            handlers::read_file,
            handlers::sync_file,
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        // .plugin(tauri_plugin_updater::Builder::new().build())
        // .plugin(tauri_plugin_dialog::init())
        // .setup(|app| {
        //     let mut store = StoreBuilder::new("spaces.json").build(app.handle().clone());

        //     match store.load() {
        //         Ok(..) => {}
        //         Err(..) => store.save()?,
        //     }

        //     // store.clear()?;
        //     // store.save()?;

        //     Ok(())
        // })
        ;

    // #[cfg(any(debug_assertions, feature = "devtools"))]
    // let builder = builder
    //     .plugin(tauri_plugin_devtools::init())
    //     .plugin(tauri_plugin_devtools_app::init());

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
