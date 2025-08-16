use eframe::egui::{self};

use crate::utils::reqwestur::Reqwestur;

/// The help modal
pub fn panel(app: &mut Reqwestur, ui: &mut egui::Ui) {
    egui::Modal::new("HelpModal".into()).show(ui.ctx(), |ui| {
        ui.set_min_width(200.);

        ui.heading("Help");

        ui.label("This is some text");

        if ui.button("Close").clicked() {
            app.help_modal_open = false;
        }
    });
}
