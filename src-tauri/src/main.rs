#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use fst::{IntoStreamer, Map, Streamer};
use regex_automata::dfa::dense;
use tauri::{Manager, State, TitleBarStyle, WindowBuilder};

pub static FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fst.bin"));

type Icon = (String, char);

#[tauri::command]
fn search(map: State<'_, Map<&[u8]>>, pattern: &str) -> Vec<Icon> {
    let dfa = dense::Builder::new().build(pattern).unwrap();
    let mut stream = map.search(&dfa).into_stream();

    let mut entries = vec![];
    while let Some((k, v)) = stream.next() {
        entries.push((
            String::from_utf8_lossy(k).to_string(),
            char::from_u32(v as u32).unwrap(),
        ))
    }

    entries
}

#[tauri::command]
fn all(map: State<'_, Map<&[u8]>>) -> Vec<Icon> {
    let mut stream = map.stream();

    let mut entries = vec![];
    while let Some((k, v)) = stream.next() {
        entries.push((
            String::from_utf8_lossy(k).to_string(),
            char::from_u32(v as u32).unwrap(),
        ))
    }

    entries
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![all, search])
        .setup(|app| {
            app.manage(Map::new(FST).unwrap());

            WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
                .inner_size(1000.0, 600.0)
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
