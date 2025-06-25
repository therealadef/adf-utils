use eframe::egui;
use rfd::FileDialog;
use std::process::Command;
use crate::settings::AppSettings;

#[derive(Default)]
pub struct MediaConverterTab {
    // Video/audio section
    input_path: Option<String>,
    output_format: String,
    log: String,

    // Image section
    image_input_path: Option<String>,
    image_output_format: String,
}

impl MediaConverterTab {
    pub fn ui(&mut self, ui: &mut egui::Ui, settings: &AppSettings) {
        // === Media Converter ===
        ui.label("Media Converter");

        if ui.button("Select Media File").clicked() {
            if let Some(file) = FileDialog::new().pick_file() {
                self.input_path = Some(file.display().to_string());
            }
        }

        if let Some(ref path) = self.input_path {
            ui.label(format!("Selected: {}", path));
        }

        let formats = ["mp4", "mp3", "wav", "avi", "mkv"];
        egui::ComboBox::from_label("Output Format")
            .selected_text(&self.output_format)
            .show_ui(ui, |ui| {
                for &format in formats.iter() {
                    ui.selectable_value(&mut self.output_format, format.to_string(), format);
                }
            });

        if self.output_format.is_empty() {
            self.output_format = settings.output_format.clone();
        }

        if ui.button("Convert Media").clicked() {
            if let Some(ref input) = self.input_path {
                let output = format!("{}/output.{}", settings.output_directory, self.output_format);
                let ffmpeg_path = if settings.ffmpeg_path.is_empty() {
                    "ffmpeg"
                } else {
                    &settings.ffmpeg_path
                };
                match Command::new(ffmpeg_path)
                    .args(["-i", input, &output])
                    .output()
                {
                    Ok(out) => {
                        self.log = String::from_utf8_lossy(&out.stderr).to_string();
                    }
                    Err(e) => {
                        self.log = format!("Error: {}", e);
                    }
                }
            }
        }

        ui.separator();

        // === Image Converter ===
        ui.label("Image Converter");

        if ui.button("Select Image File").clicked() {
            if let Some(file) = FileDialog::new().pick_file() {
                self.image_input_path = Some(file.display().to_string());
            }
        }

        if let Some(ref path) = self.image_input_path {
            ui.label(format!("Selected: {}", path));
        }

        let image_formats = ["png", "jpg", "jpeg", "ico", "bmp", "tiff", "webp"];
        egui::ComboBox::from_label("Image Output Format")
            .selected_text(&self.image_output_format)
            .show_ui(ui, |ui| {
                for &format in image_formats.iter() {
                    ui.selectable_value(&mut self.image_output_format, format.to_string(), format);
                }
            });

        if ui.button("Convert Image").clicked() {
            if let Some(ref input) = self.image_input_path {
                if self.image_output_format.is_empty() {
                    self.log = "Please select an image output format.".to_string();
                } else {
                    let output = format!("{}/image_output.{}", settings.output_directory, self.image_output_format);
                    let ffmpeg_path = if settings.ffmpeg_path.is_empty() {
                        "ffmpeg"
                    } else {
                        &settings.ffmpeg_path
                    };
                    match Command::new(ffmpeg_path)
                        .args(["-i", input, &output])
                        .output()
                    {
                        Ok(out) => {
                            self.log = String::from_utf8_lossy(&out.stderr).to_string();
                        }
                        Err(e) => {
                            self.log = format!("Error: {}", e);
                        }
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
