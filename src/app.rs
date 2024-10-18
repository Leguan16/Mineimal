mod util;
mod auth;

use crate::app::auth::auth::{Auth, Session};
use crate::{Profile, ProfileData, Version, VersionType, Widgets};
use eframe::epaint::{Color32, FontFamily, FontId};
use egui::{Align, TextFormat, Ui, Vec2, WidgetText};
use std::fmt::Debug;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Launcher {
    profile_data: ProfileData,
    version: Version,
    
}

impl Default for Launcher {
    fn default() -> Self {
        let mut prof: Vec<Profile> = Vec::new();
        prof.insert(0, Profile { name: "Select".to_owned() });

        Self {
            profile_data: ProfileData { selected: 0, profiles: prof },
            version: Version { id: "Select".to_owned(), r#type: VersionType::release, sha1: "default".to_owned(), url: "default".to_owned() },
        }
    }
}

impl Launcher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Launcher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.profile_combo(ui);
                self.version_combo(ui);
                ui.add_space(25.0);
                start_button(ui);
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl Widgets for Launcher {
    fn profile_combo(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Profile")
            .selected_text(format!("{}", self.profile_data.profiles[self.profile_data.selected].name))
            .show_ui(ui, |ui| {
                for index in 0..self.profile_data.profiles.len() {
                    if self.profile_data.profiles[self.profile_data.selected].name.eq("Select") {
                        continue;
                    }
                    ui.selectable_value(&mut self.profile_data.selected, index, self.profile_data.profiles[index].name.clone());
                }
                if ui.button("+ Add Profile").clicked() {
                    println!("Add account");
                    let _ = open_add_account_window(ui);
                }
            });
    }

    fn version_combo(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Version")
            .selected_text(format!("{}", self.version.id))
            .show_ui(ui, |ui| {
                for version in util::fetch_versions().versions {
                    ui.selectable_value(&mut self.version, version.clone(), version.id.clone());
                }
            });
    }
}

fn open_add_account_window(ui: &mut Ui) {
    println!("Add account");
    Session::poll_device_code(ui);
}

fn start_button(ui: &mut egui::Ui) {
    let mut job = egui::text::LayoutJob::default();
    job.append("Start", 0.0, TextFormat {
        font_id: FontId::new(16f32, FontFamily::Monospace),
        extra_letter_spacing: 0.0,
        line_height: None,
        color: Color32::WHITE,
        background: Default::default(),
        italics: false,
        underline: Default::default(),
        strikethrough: Default::default(),
        valign: Align::Center,
    });
    if ui.add(egui::Button::new(WidgetText::LayoutJob(job)).min_size(Vec2::from([64f32, 32f32]))).clicked() {}
}

