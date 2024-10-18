#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::Launcher;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Version {
    id: String,
    r#type: VersionType,
    url: String,
    sha1: String,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct VersionData {
    versions: Vec<Version>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub enum VersionType {
    release,
    snapshot,
    old_beta,
    old_alpha
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Profile {
    name: String,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProfileData {
    selected: usize,
    profiles: Vec<Profile>,
}

pub trait Widgets {
    fn profile_combo(&mut self, ui: &mut egui::Ui);
    fn version_combo(&mut self, ui: &mut egui::Ui);
}