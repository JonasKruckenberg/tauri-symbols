use fst::Map;
use serde::Deserialize;
use std::{fs, io::BufReader, path::Path, env};

#[derive(Debug, Deserialize, PartialEq)]
struct Icon {
    name: String,
    symbol: char,
}

impl PartialOrd for Icon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl<'a> From<&'a Icon> for (&'a [u8], u64) {
    fn from(entry: &'a Icon) -> Self {
        (entry.name.as_bytes(), entry.symbol as u64)
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../data.json");

    let f = fs::File::open("../data.json").unwrap();
    let reader = BufReader::new(f);

    let mut data: Vec<Icon> = serde_json::from_reader(reader).unwrap();
    data.sort_by_key(|e| e.name.to_string());

    let map = Map::from_iter(data.iter().map(Into::into)).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fst.bin");

    fs::write(&dest_path, map.as_fst().as_bytes()).unwrap();

    tauri_build::build()
}
