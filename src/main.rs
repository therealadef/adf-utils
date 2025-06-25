mod media_converter;
mod hex_editor;
mod settings;
mod downloader;

use eframe::egui::{self, CentralPanel, TopBottomPanel};
use egui::style::{WidgetVisuals, Widgets};
use egui::CornerRadius;
use eframe::{run_native, NativeOptions};
use egui::viewport::ViewportBuilder; // ✅ From egui
use media_converter::MediaConverterTab;
use hex_editor::HexEditorTab;
use settings::SettingsTab;
use downloader::DownloaderTab;

#[derive(PartialEq)]
enum Tab {
    MediaConverter,
    HexEditor,
    Downloader,
    Settings,
}

struct AdfUtilsApp {
    selected_tab: Tab,
    media_converter: MediaConverterTab,
    hex_editor: HexEditorTab,
    settings: SettingsTab,
    downloader: DownloaderTab,
    icon_set: bool,
}

impl Default for AdfUtilsApp {
    fn default() -> Self {
        Self {
            selected_tab: Tab::MediaConverter,
            media_converter: MediaConverterTab::default(),
            hex_editor: HexEditorTab::default(),
            downloader: DownloaderTab::default(),
            settings: SettingsTab::load(),
            icon_set: false,
        }
    }
}

impl eframe::App for AdfUtilsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Rounded corners styling
        let mut style = (*ctx.style()).clone();

        fn with_corner_radius(w: &WidgetVisuals, radius: u8) -> WidgetVisuals {
            WidgetVisuals {
                bg_fill: w.bg_fill,
                bg_stroke: w.bg_stroke,
                fg_stroke: w.fg_stroke,
                corner_radius: CornerRadius::same(radius),
                expansion: w.expansion,
                ..*w
            }
        }

        style.visuals.widgets = Widgets {
            noninteractive: with_corner_radius(&style.visuals.widgets.noninteractive, 6),
            inactive: with_corner_radius(&style.visuals.widgets.inactive, 6),
            hovered: with_corner_radius(&style.visuals.widgets.hovered, 6),
            active: with_corner_radius(&style.visuals.widgets.active, 6),
            open: with_corner_radius(&style.visuals.widgets.open, 6),
        };

        ctx.set_style(style);

        TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, Tab::MediaConverter, "Media Converter");
                ui.selectable_value(&mut self.selected_tab, Tab::HexEditor, "Hex Editor");
                ui.selectable_value(&mut self.selected_tab, Tab::Downloader, "Downloader");
                ui.selectable_value(&mut self.selected_tab, Tab::Settings, "Settings");
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            match self.selected_tab {
                Tab::MediaConverter => self.media_converter.ui(ui, &self.settings.config),
                Tab::HexEditor => self.hex_editor.ui(ui),
                Tab::Downloader => self.downloader.ui(ui),
                Tab::Settings => self.settings.ui(ui),
            }
        });
    }
}

fn main() {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1100.0, 550.0]), // ✅ No `Some(...)` — it's not an Option anymore
        ..Default::default()
    };

    run_native(
        "adf Utils",
        native_options,
        Box::new(|_cc| Ok(Box::new(AdfUtilsApp::default()))), // ✅ Wrap in Ok
    )
    .unwrap();
}
