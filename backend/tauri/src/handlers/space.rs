use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{App, AppHandle, EventLoopMessage, Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};
use time::{Date, Month, OffsetDateTime, UtcOffset};
use ulid::Ulid;

use crate::dir;

#[tauri::command(rename_all = "camelCase")]
pub fn create_space(
    name: String,
    icon: String,
    color: String,
    order: Option<u8>,
    app_handle: AppHandle,
) {
    let mut store = load_spaces(&app_handle);

    let id = Ulid::from_datetime(OffsetDateTime::now_utc().into());
    let order = order.unwrap_or_else(|| store.length() as u8);

    store.set(
        id.to_string(),
        json!({
            "name": name,
            "icon": icon,
            "color": color,
            "order": order,
        }),
    );

    store.save().unwrap();

    fs::create_dir_all(
        crate::dir::spaces(&app_handle)
            .join(id.to_string())
            .join("daily"),
    )
    .unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    name: String,
    icon: String,
    color: String,
    order: u8,
}

#[tauri::command(rename_all = "camelCase")]
pub fn list_spaces(app_handle: AppHandle) -> Box<[(Ulid, Space)]> {
    let mut store = load_spaces(&app_handle);

    let mut spaces = store
        .entries()
        .into_iter()
        .map(|(id, space)| {
            (
                Ulid::from_string(&id).unwrap(),
                serde_json::from_value::<Space>(space.clone()).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    spaces.sort_by_key(|(_, space)| space.order);

    spaces.into_boxed_slice()
}

fn load_spaces(app_handle: &AppHandle) -> Arc<Store<Wry>> {
    let mut store = StoreBuilder::new(app_handle, "spaces.json")
        .build()
        .unwrap();

    // match store.load() {
    //     Ok(..) => {}
    //     Err(..) => store.save().unwrap(),
    // }

    store
}

// pub fn install_package
