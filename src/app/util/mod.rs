use std::io::Read;
use cached::proc_macro::cached;
use crate::VersionData;

#[cached]
pub fn fetch_versions() -> VersionData {
    println!("fetch");
    let mut response = reqwest::blocking::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json");
    let mut body = String::new();
    response.unwrap().read_to_string(&mut body).unwrap();

    serde_json::from_str(&body).unwrap()
}