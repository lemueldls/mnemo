// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    if let Some(backend_note) = configure_display_backend() {
        eprintln!("{backend_note}");
    }

    mnemo::run();
}

#[cfg(target_os = "linux")]
fn configure_display_backend() -> Option<String> {
    use std::env;

    let set_env_if_absent = |key: &str, value: &str| {
        if env::var_os(key).is_none() {
            // Safety: called during startup before any threads are spawned, so mutating the
            // process environment is safe.
            unsafe { env::set_var(key, value) };
        }
    };

    let on_wayland = env::var_os("WAYLAND_DISPLAY").is_some()
        || matches!(
            env::var("XDG_SESSION_TYPE"),
            Ok(v) if v.eq_ignore_ascii_case("wayland")
        );

    if !on_wayland {
        return None;
    }

    let mut disable_wayland = false;

    for arg in env::args().skip(1) {
        if arg == "--disable-wayland" {
            disable_wayland = true;
            break;
        }
    }

    // Allow users to explicitly disable Wayland if their setup is unstable.
    let disable_wayland = disable_wayland
        || matches!(
            env::var("MNEMO_DISABLE_WAYLAND"),
            Ok(v) if matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes")
        );

    if disable_wayland {
        // Prefer XWayland when available.
        if env::var_os("DISPLAY").is_some() {
            set_env_if_absent("WINIT_UNIX_BACKEND", "x11");
            set_env_if_absent("GDK_BACKEND", "x11");
            set_env_if_absent("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            return Some("Wayland session detected; forcing X11 backend.".into());
        }

        set_env_if_absent("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

        Some(
            "Wayland session detected without X11; leaving Wayland enabled (set WINIT_UNIX_BACKEND/GDK_BACKEND manually if needed)."
                .into(),
        )
    } else {
        Some(
            "Wayland session detected; Set MNEMO_DISABLE_WAYLAND=1 to disable native Wayland."
                .into(),
        )
    }
}
