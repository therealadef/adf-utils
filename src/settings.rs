use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppSettings {
    pub ffmpeg_path: String,
    pub output_format: String,
    pub output_directory: String,
}

#[derive(Default)]
pub struct SettingsTab {
    pub config: AppSettings,
}

impl SettingsTab {
    pub fn load() -> Self {
        let config: AppSettings = confy::load("adfUtils", None).unwrap_or_default();
        Self { config }
    }

    pub fn save(&self) {
        let _ = confy::store("adfUtils", None, &self.config);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Settings");
        ui.horizontal(|ui| {
            ui.label("FFmpeg Path:");
            ui.text_edit_singleline(&mut self.config.ffmpeg_path);
        });
        ui.horizontal(|ui| {
            ui.label("Default Output Format:");
            ui.text_edit_singleline(&mut self.config.output_format);
        });
        ui.horizontal(|ui| {
            ui.label("Output Directory:");
            ui.text_edit_singleline(&mut self.config.output_directory);
        });

        if ui.button("Save Settings").clicked() {
            self.save();
        }
    }
}
