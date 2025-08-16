use eframe::egui::{self, AtomExt, IntoAtoms};

use crate::utils::reqwestur::{AppView, Reqwestur};

/// The view handling the initial user interaction
pub fn panel<'a>(app: &'a mut Reqwestur) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical()
                    .max_width(ui.available_width())
                    .max_height(ui.available_height())
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() / 4.);
                            let laptop_icon = egui::include_image!("../../assets/laptop.svg");
                            ui.add(
                                egui::Image::new(laptop_icon)
                                    .fit_to_original_size(2.5)
                                    .tint(ui.visuals().text_color()),
                            );

                            ui.add_space(5.);

                            let half_available_width = ui.available_width() / 3.;
                            let button_width = if half_available_width <= 250. {
                                250.
                            } else {
                                half_available_width
                            };
                            ui.scope(|ui| {
                                ui.visuals_mut().button_frame = false;

                                if ui
                                    .add(
                                        egui::Button::new((
                                            egui::Image::new(egui::include_image!(
                                                "../../assets/create.svg"
                                            ))
                                            .atom_size(egui::Vec2::splat(18.)),
                                            egui::RichText::new("Create a new request")
                                                .size(16.)
                                                .into_atoms(),
                                        ))
                                        .shortcut_text(ui.ctx().format_shortcut(
                                            &egui::KeyboardShortcut::new(
                                                egui::Modifiers::CTRL,
                                                egui::Key::N,
                                            ),
                                        ))
                                        .image_tint_follows_text_color(true)
                                        .min_size(egui::vec2(button_width, 32.)),
                                    )
                                    .clicked()
                                {
                                    app.view = AppView::Request;
                                }

                                if ui
                                    .add(
                                        egui::Button::new((
                                            egui::Image::new(egui::include_image!(
                                                "../../assets/undo_history.svg"
                                            ))
                                            .atom_size(egui::Vec2::splat(18.)),
                                            egui::RichText::new("View your recent requests")
                                                .size(16.),
                                        ))
                                        .shortcut_text(ui.ctx().format_shortcut(
                                            &egui::KeyboardShortcut::new(
                                                egui::Modifiers::CTRL,
                                                egui::Key::H,
                                            ),
                                        ))
                                        .image_tint_follows_text_color(true)
                                        .min_size(egui::vec2(button_width, 32.)),
                                    )
                                    .clicked()
                                {
                                    app.view = AppView::History;
                                }

                                if ui
                                    .add(
                                        egui::Button::new((
                                            egui::Image::new(egui::include_image!(
                                                "../../assets/floppy.svg"
                                            ))
                                            .atom_size(egui::Vec2::splat(18.)),
                                            egui::RichText::new("Open your saved requests")
                                                .size(16.),
                                        ))
                                        .shortcut_text(ui.ctx().format_shortcut(
                                            &egui::KeyboardShortcut::new(
                                                egui::Modifiers::CTRL,
                                                egui::Key::O,
                                            ),
                                        ))
                                        .image_tint_follows_text_color(true)
                                        .min_size(egui::vec2(button_width, 32.)),
                                    )
                                    .clicked()
                                {
                                    app.view = AppView::Saved;
                                }
                            });
                        });
                    });
            })
            .response
    }
}
