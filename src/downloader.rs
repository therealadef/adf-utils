use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

use eframe::egui::{self, Button, ComboBox, ProgressBar, TextEdit};
use rfd::FileDialog;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Format {
    Mp3,
    Mp4,
    Mkv,
}

impl Format {
    fn to_args(&self) -> Vec<&'static str> {
        match self {
            Format::Mp3 => vec!["-x", "--audio-format", "mp3"],
            Format::Mp4 => vec!["-f", "mp4"],
            Format::Mkv => vec!["-f", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/mp4"],
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Format::Mp3 => "mp3",
            Format::Mp4 => "mp4",
            Format::Mkv => "mkv",
        }
    }
}

#[derive(Default)]
struct DownloadState {
    progress: f32,
    is_downloading: bool,
    status_text: String,
}

pub struct DownloaderTab {
    url: String,
    format: Format,
    save_path: Option<PathBuf>,
    state: Arc<Mutex<DownloadState>>,
}

impl Default for DownloaderTab {
    fn default() -> Self {
        Self {
            url: String::new(),
            format: Format::Mp3,
            save_path: None,
            state: Arc::new(Mutex::new(DownloadState::default())),
        }
    }
}

impl DownloaderTab {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("YouTube URL:");
        ui.add(TextEdit::singleline(&mut self.url).hint_text("https://..."));

        ComboBox::from_label("Format")
            .selected_text(self.format.as_str())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.format, Format::Mp3, "mp3");
                ui.selectable_value(&mut self.format, Format::Mp4, "mp4");
                ui.selectable_value(&mut self.format, Format::Mkv, "mkv");
            });

        if ui.button("Choose Save Location").clicked() {
            if let Some(path) = FileDialog::new()
                .set_file_name("download")
                .save_file()
            {
                self.save_path = Some(path);
            }
        }

        if let Some(path) = &self.save_path {
            ui.label(format!("Save to: {}", path.display()));
        }

        let downloading = self.state.lock().unwrap().is_downloading;

        if ui.add_enabled(!downloading, Button::new("Download")).clicked() {
            if let Some(path) = &self.save_path {
                self.start_download(self.url.clone(), self.format.clone(), path.clone());
            }
        }

        let state = self.state.lock().unwrap();
        if state.is_downloading {
            ui.add(ProgressBar::new(state.progress).show_percentage());
        }
        ui.label(&state.status_text);
    }

    fn start_download(&self, url: String, format: Format, save_path: PathBuf) {
        let state = Arc::clone(&self.state);

        thread::spawn(move || {
            {
                let mut s = state.lock().unwrap();
                s.is_downloading = true;
                s.status_text = "Starting download...".to_string();
            }

            let mut args = format.to_args();
            args.push(&url);
            args.push("-o");
            args.push(save_path.to_str().unwrap());

            let mut cmd = Command::new("yt-dlp")
                .args(args)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start yt-dlp");

            if let Some(stdout) = cmd.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    let mut s = state.lock().unwrap();
                    if let Some(progress) = parse_progress(&line) {
                        s.progress = progress;
                        s.status_text = format!("Downloading... {:.0}%", progress * 100.0);
                    } else if line.contains("[download]") {
                        s.status_text = line;
                    }
                }
            }

            let _ = cmd.wait();

            let mut s = state.lock().unwrap();
            s.is_downloading = false;
            s.progress = 1.0;
            s.status_text = "Download complete!".to_string();
        });
    }
}

fn parse_progress(line: &str) -> Option<f32> {
    let re = Regex::new(r"(\d{1,3}\.\d)%").unwrap();
    re.captures(line)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<f32>().ok())
        .map(|p| p / 100.0)
}