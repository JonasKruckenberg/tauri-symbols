#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use cached::proc_macro::cached;
use fst::{IntoStreamer, Map, Streamer};
use regex_automata::dfa::dense;
use serde::{Serialize, Serializer};
use tauri::{Manager, State, TitleBarStyle, WindowBuilder};
use tauri_plugin_sentry::sentry::IntoDsn;

pub static FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fst.bin"));

type Icon = (String, char);

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to create FST index {0}")]
    Fst(#[from] fst::Error),
    #[error("Failed to parse regex {0}")]
    Regex(#[from] regex_automata::dfa::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
#[cached(
    result = true,
    size = 50,
    key = "String",
    convert = r##"{ format!("{}", pattern) }"##
)]
fn search(map: State<'_, Map<&[u8]>>, pattern: &str) -> Result<Vec<Icon>, Error> {
    let dfa = dense::Builder::new().build(pattern)?;
    let mut stream = map.search(&dfa).into_stream();

    let mut entries = vec![];
    while let Some((k, v)) = stream.next() {
        entries.push((
            String::from_utf8_lossy(k).to_string(),
            char::from_u32(v as u32).unwrap(),
        ))
    }

    Ok(entries)
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
    let logger = sentry_log::SentryLogger::with_dest(env_logger::builder().build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let init_sentry = |_:bool| {
        tauri_plugin_sentry::sentry::init(tauri_plugin_sentry::sentry::ClientOptions {
            dsn: "https://cd1169c0f3334d53b97db60d1ca1ac01@o4503930527088640.ingest.sentry.io/4503930528399360".into_dsn().expect("failed to parse DSN"),
            release: tauri_plugin_sentry::sentry::release_name!(),
            ..Default::default()
        })
    };

    tauri_plugin_sentry::init(init_sentry, |sentry_plugin| {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![all, search])
            .plugin(sentry_plugin)
            .setup(|app| {
                app.manage(Map::new(FST)?);
    
                WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
                    .inner_size(1000.0, 600.0)
                    .visible(true)
                    .title("")
                    .hidden_title(true)
                    .title_bar_style(TitleBarStyle::Overlay)
                    .build()?;
    
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });

}
