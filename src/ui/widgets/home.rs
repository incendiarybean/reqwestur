use eframe::egui::{self, AtomExt, IntoAtoms};

use crate::{
    ui::widgets::groups::centered_group,
    utils::reqwestur::{AppView, Reqwestur},
};

/// The view handling the initial user interaction
pub fn panel<'a>(app: &'a mut Reqwestur) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        ui.add(centered_group(|ui| {
            let laptop_icon = egui::include_image!("../../assets/laptop.svg");
            ui.add(
                egui::Image::new(laptop_icon)
                    .fit_to_original_size(2.5)
                    .tint(ui.visuals().text_color()),
            );

            ui.add_space(5.);

            if ui
                .add(
                    egui::Button::new((
                        egui::Image::new(egui::include_image!("../../assets/create.svg"))
                            .atom_size(egui::Vec2::splat(18.)),
                        egui::RichText::new("Create a new request")
                            .size(16.)
                            .into_atoms(),
                    ))
                    .truncate()
                    .shortcut_text(ui.ctx().format_shortcut(&egui::KeyboardShortcut::new(
                        egui::Modifiers::CTRL,
                        egui::Key::N,
                    )))
                    .image_tint_follows_text_color(true)
                    .min_size(egui::vec2(ui.available_width(), 32.)),
                )
                .clicked()
            {
                app.view = AppView::Request;
            }

            if ui
                .add(
                    egui::Button::new((
                        egui::Image::new(egui::include_image!("../../assets/undo_history.svg"))
                            .atom_size(egui::Vec2::splat(18.)),
                        egui::RichText::new("View your recent requests").size(16.),
                    ))
                    .truncate()
                    .shortcut_text(ui.ctx().format_shortcut(&egui::KeyboardShortcut::new(
                        egui::Modifiers::CTRL,
                        egui::Key::H,
                    )))
                    .image_tint_follows_text_color(true)
                    .min_size(egui::vec2(ui.available_width(), 32.)),
                )
                .clicked()
            {
                app.view = AppView::History;
            }

            if ui
                .add(
                    egui::Button::new((
                        egui::Image::new(egui::include_image!("../../assets/floppy.svg"))
                            .atom_size(egui::Vec2::splat(18.)),
                        egui::RichText::new("Open your saved requests").size(16.),
                    ))
                    .truncate()
                    .shortcut_text(ui.ctx().format_shortcut(&egui::KeyboardShortcut::new(
                        egui::Modifiers::CTRL,
                        egui::Key::O,
                    )))
                    .image_tint_follows_text_color(true)
                    .min_size(egui::vec2(ui.available_width(), 32.)),
                )
                .clicked()
            {
                app.view = AppView::Saved;
            }
        }))
    }
}
