use eframe::egui;
use rfd::FileDialog;
use std::process::Command;
use crate::settings::AppSettings;

#[derive(Default)]
pub struct MediaConverterTab {
    input_path: Option<String>,
    output_format: String, // <-- store chosen output format here
    log: String,
}

impl MediaConverterTab {
    pub fn ui(&mut self, ui: &mut egui::Ui, settings: &AppSettings) {
        ui.label("Media Converter");

        if ui.button("Select File").clicked() {
            if let Some(file) = FileDialog::new().pick_file() {
                self.input_path = Some(file.display().to_string());
            }
        }

        if let Some(ref path) = self.input_path {
            ui.label(format!("Selected: {}", path));
        }

        // Dropdown to choose output format
        let formats = ["mp4", "mp3", "wav", "avi", "mkv"];
        egui::ComboBox::from_label("Output Format")
            .selected_text(&self.output_format)
            .show_ui(ui, |ui| {
                for &format in formats.iter() {
                    ui.selectable_value(&mut self.output_format, format.to_string(), format);
                }
            });

        // Use default if output_format is empty
        if self.output_format.is_empty() {
            self.output_format = settings.output_format.clone();
        }

        if ui.button("Convert").clicked() {
            if let Some(ref input) = self.input_path {
                let output = format!("{}/output.{}", settings.output_directory, self.output_format);
                let ffmpeg_path = if settings.ffmpeg_path.is_empty() {
                    "ffmpeg"
                } else {
                    &settings.ffmpeg_path
                };
                match Command::new(ffmpeg_path)
                    .args(["-i", input, &output])
                    .output() {
                    Ok(out) => {
                        self.log = String::from_utf8_lossy(&out.stderr).to_string();
                    },
                    Err(e) => {
                        self.log = format!("Error: {}", e);
                    }
                }
            }
        }

        ui.separator();
        ui.label("Console Output:");
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.monospace(&self.log);
        });
    }
}
