#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::sync::Arc;

mod ui;
mod utils;

use crate::utils::reqwestur::Reqwestur;

/// The main application runner
///
/// Spawns a new native eframe application with the provided parameters
fn main() -> Result<(), eframe::Error> {
    let icon: &[u8] = include_bytes!("assets/icon.png");
    let img: image::DynamicImage = image::load_from_memory(icon).unwrap();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_min_inner_size(egui::vec2(600.0, 400.0))
            .with_resizable(true)
            .with_icon(Arc::new(egui::viewport::IconData {
                rgba: img.into_bytes(),
                width: 288,
                height: 288,
            })),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "REQWESTUR",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            if let Some(theme) = cc.egui_ctx.system_theme() {
                cc.egui_ctx.set_theme(theme);
            } else {
                cc.egui_ctx.set_theme(egui::Theme::Light);
            }

            Ok(Box::new(Reqwestur::new(cc)))
        }),
    )
}
