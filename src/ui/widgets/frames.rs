use eframe::egui;

pub fn padded_group<F: FnOnce(&mut egui::Ui)>(content: F) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        ui.group(|ui| {
            ui.add_space(2.);

            ui.allocate_space(egui::vec2(ui.available_width(), 0.));

            content(ui);

            ui.add_space(2.);
        })
        .response
    }
}
