use std::str::FromStr;

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, window::Color};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init());

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_cli::init())
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let _ = app
                    .get_webview_window("main")
                    .expect("no main window")
                    .set_focus();
            }))
            .plugin(tauri_plugin_updater::Builder::new().build())
    }

    // #[cfg(any(debug_assertions, feature = "devtools"))]
    // let builder = builder
    //     .plugin(tauri_plugin_devtools::init())
    //     .plugin(tauri_plugin_devtools_app::init());

    builder
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Mnemo")
                .background_color(Color::from_str("#4c4d72").unwrap());

            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(tauri::TitleBarStyle::Overlay);
            #[cfg(target_os = "linux")]
            let win_builder = win_builder.decorations(false);

            win_builder.build().unwrap();

            #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register_all()?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
