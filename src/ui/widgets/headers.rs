use eframe::egui::{self};

use crate::{
    ui::widgets::buttons::default_button,
    utils::{request::Request, reqwestur::Reqwestur},
};

/// The header editor window
pub fn editor(app: &mut Reqwestur, request: &mut Request, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("header_editor"),
        egui::ViewportBuilder::default()
            .with_title("Header Editor")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                let add_icon = egui::include_image!("../../assets/plus.svg");
                if ui
                    .add(default_button(
                        Some(add_icon),
                        "New Header",
                        ui.available_width(),
                        ui.visuals().text_color(),
                    ))
                    .clicked()
                {
                    request.headers.push((String::default(), String::default()));
                }

                ui.add_space(2.);
                ui.separator();
                ui.add_space(2.);

                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .max_height(ui.available_height() - 34.)
                    .max_width(ui.available_width())
                    .horizontal_scroll_offset(5.)
                    .show_rows(ui, 18., request.headers.len(), |ui, row_range| {
                        for row in row_range {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    if let Some((name, value)) = request.headers.get_mut(row) {
                                        let name_editor = egui::TextEdit::singleline(name)
                                            .hint_text("Header Name")
                                            .margin(5.)
                                            .vertical_align(egui::Align::Center)
                                            .desired_width(ui.available_width() / 2. - 50.);

                                        let value_editor = egui::TextEdit::singleline(value)
                                            .hint_text("Header Value")
                                            .margin(5.)
                                            .vertical_align(egui::Align::Center)
                                            .desired_width(ui.available_width());

                                        ui.add(name_editor);
                                        ui.add(value_editor);
                                    }
                                });
                            });
                        }
                    });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui
                        .add(default_button(
                            None,
                            "Done!",
                            ui.available_width(),
                            ui.visuals().text_color(),
                        ))
                        .clicked()
                    {
                        app.header_editor_open = false;
                    }
                });
            });

            if context.input(|i| i.viewport().close_requested()) {
                app.header_editor_open = false;
            }
        },
    );
}
