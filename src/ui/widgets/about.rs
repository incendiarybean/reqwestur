use crate::utils::reqwestur::Reqwestur;
use eframe::egui;

/// The information modal
pub fn panel(app: &mut Reqwestur, ui: &mut egui::Ui) {
    egui::Modal::new("AboutModal".into()).show(ui.ctx(), |ui| {
        ui.set_min_width(200.);

        ui.heading("About");

        ui.label("This is some text");

        if ui.button("Close").clicked() {
            app.about_modal_open = false;
        }
    });
}
