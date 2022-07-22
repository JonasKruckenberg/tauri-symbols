#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{TitleBarStyle, WindowBuilder};

fn main() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
                .inner_size(800.0, 600.0)
                .visible(false)
                .title("")
                .hidden_title(true)
                .title_bar_style(TitleBarStyle::Overlay)
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
