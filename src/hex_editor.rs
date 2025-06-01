use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::io::Write;

#[derive(Default)]
pub struct HexEditorTab {
    bytes: Vec<u8>,
    original_bytes: Vec<u8>,
    search_query: String,
    selected_index: Option<usize>,
    file_path: Option<String>,
    page_offset: usize,
    diffs: Option<Vec<(usize, u8, u8)>>, // (offset, original_byte, modified_byte)
}

impl HexEditorTab {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Open File Button
        if ui.button("Open File").clicked() {
            if let Some(file) = FileDialog::new().pick_file() {
                match fs::read(&file) {
                    Ok(data) => {
                        self.original_bytes = data.clone();
                        self.bytes = data;
                        self.file_path = Some(file.display().to_string());
                        self.page_offset = 0;
                        self.selected_index = None;
                        self.diffs = None;
                    }
                    Err(e) => {
                        self.file_path = None;
                        self.bytes.clear();
                        self.original_bytes.clear();
                        self.diffs = None;
                        ui.label(format!("Error reading file: {}", e));
                    }
                }
            }
        }

        // Show file path
        if let Some(ref path) = self.file_path {
            ui.label(format!("Opened: {}", path));
        } else {
            ui.label("No file loaded.");
        }

        ui.separator();

        // Paging Controls
        ui.horizontal(|ui| {
            if ui.button("Prev Page").clicked() {
                self.page_offset = self.page_offset.saturating_sub(256);
            }
            if ui.button("Next Page").clicked() {
                if self.page_offset + 256 < self.bytes.len() {
                    self.page_offset += 256;
                }
            }
            ui.label(format!(
                "Showing bytes {} - {} of {}",
                self.page_offset,
                (self.page_offset + 256).min(self.bytes.len()),
                self.bytes.len()
            ));
        });

        ui.separator();

        // Search Controls
        ui.horizontal(|ui| {
            ui.label("Search (hex bytes):");
            ui.text_edit_singleline(&mut self.search_query);
            if ui.button("Find").clicked() {
                // Parse search string like "4A 6F 68 6E" into bytes
                let search_bytes = self
                    .search_query
                    .split_whitespace()
                    .filter_map(|b| u8::from_str_radix(b, 16).ok())
                    .collect::<Vec<_>>();

                if !search_bytes.is_empty() {
                    self.selected_index = self.bytes
                        .windows(search_bytes.len())
                        .position(|w| w == search_bytes.as_slice());
                    if let Some(idx) = self.selected_index {
                        self.page_offset = (idx / 256) * 256;
                    }
                } else {
                    self.selected_index = None;
                }
            }
        });

        // Show search result
        if let Some(idx) = self.selected_index {
            ui.label(format!("Found at offset: {:#X}", idx));
        }

        ui.separator();

        // Display hex + ASCII editor in pages of 256 bytes (16 bytes per row)
        egui::ScrollArea::vertical().show(ui, |ui| {
            let end = (self.page_offset + 256).min(self.bytes.len());
            let slice = &mut self.bytes[self.page_offset..end];

            for (i, chunk) in slice.chunks_mut(16).enumerate() {
                let offset = self.page_offset + i * 16;
                ui.horizontal(|ui| {
                    // Offset label
                    ui.monospace(format!("{:08X}:", offset));

                    ui.spacing_mut().item_spacing.x = 4.0;

                    // Hex bytes editor
                    for byte in chunk.iter_mut() {
                        let mut hex = format!("{:02X}", byte);
                        let response = ui
                            .add(egui::TextEdit::singleline(&mut hex).desired_width(22.0));
                        if response.changed() {
                            if let Ok(new_val) = u8::from_str_radix(&hex, 16) {
                                *byte = new_val;
                            }
                        }
                    }

                    ui.add_space(16.0);

                    // ASCII editor
                    for byte in chunk.iter_mut() {
                        let ch = if byte.is_ascii_graphic() || *byte == b' ' {
                            *byte as char
                        } else {
                            '.'
                        };

                        let mut s = ch.to_string();
                        let response = ui
                            .add(egui::TextEdit::singleline(&mut s).desired_width(14.0));
                        if response.changed() {
                            if let Some(c) = s.chars().next() {
                                *byte = c as u8;
                            }
                        }
                    }
                });
            }
        });

        ui.separator();

        // Save button
        if ui.button("Save Changes").clicked() {
            if let Some(ref path) = self.file_path {
                match fs::File::create(path).and_then(|mut f| f.write_all(&self.bytes)) {
                    Ok(_) => {
                        self.original_bytes = self.bytes.clone();
                        self.diffs = None;
                        ui.label("File saved successfully.");
                    }
                    Err(e) => {
                        ui.label(format!("Error saving file: {}", e));
                    }
                }
            } else {
                ui.label("No file loaded to save.");
            }
        }

        ui.separator();

        // Show binary diff button
        if ui.button("Show Binary Diff").clicked() {
            let diffs: Vec<_> = self.bytes.iter()
                .zip(self.original_bytes.iter())
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .map(|(i, (a, b))| (i, *b, *a))
                .collect();

            self.diffs = Some(diffs);
        }

        // Clear diff button
        if self.diffs.is_some() {
            if ui.button("Clear Diff").clicked() {
                self.diffs = None;
            }
        }

        // Show diff info if available
        if let Some(ref diffs) = self.diffs {
            ui.separator();
            ui.label(format!("Modified Bytes: {}", diffs.len()));
            for (i, original, modified) in diffs.iter().take(100) {
                ui.monospace(format!("Offset {:#06X}: {:02X} -> {:02X}", i, original, modified));
            }
            if diffs.len() > 100 {
                ui.label("... (only showing first 100 differences)");
            }
        }
    }
}
