use eframe::egui::{self, AtomExt, IntoAtoms};

use crate::{
    ui::widgets::chip::Chip,
    utils::{
        request::Request,
        reqwestur::{AppView, Reqwestur},
        traits::ToColour,
    },
};

/// The view handling the user's saved requests
pub fn panel<'a>(app: &'a mut Reqwestur, request: &'a mut Request) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    if app.saved_requests.len() == 0 {
                        ui.label("You haven't saved any requests yet!");
                    } else {
                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .max_height(ui.available_height())
                            .max_width(ui.available_width())
                            .show_rows(ui, 18., app.saved_requests.len(), |ui, row_range| {
                                for row in row_range.rev() {
                                    if let Some(row_data) = app.saved_requests.get(row) {
                                        egui::Frame::new()
                                            .stroke(egui::Stroke::new(
                                                1.,
                                                ui.visuals().noninteractive().bg_stroke.color,
                                            ))
                                            .inner_margin(egui::Vec2::splat(5.))
                                            .corner_radius(5.)
                                            .show(ui, |ui| {
                                                let Request {
                                                    method,
                                                    headers: _,
                                                    address,
                                                    timestamp: _,
                                                    content_type: _,
                                                    body: _,
                                                    params: _,
                                                    response: _,
                                                    notification: _,
                                                    event: _,
                                                } = row_data;

                                                // Create widget to add
                                                let method = method.to_string();

                                                ui.with_layout(
                                                    egui::Layout::left_to_right(egui::Align::Min)
                                                        .with_main_justify(true)
                                                        .with_main_align(egui::Align::LEFT),
                                                    |ui| {
                                                        let open_icon = egui::include_image!(
                                                            "../../assets/folder_open.svg"
                                                        );

                                                        let desired_size = ui.fonts(|f| {
                                                            f.glyph_width(
                                                                &egui::FontId::default(),
                                                                'M',
                                                            )
                                                        }) * (method.len()
                                                            as f32);

                                                        // Create button and allocate rect space
                                                        let custom_button_id =
                                                            egui::Id::new("custom_button");

                                                        // TODO: Readable label
                                                        let saved_request_button =
                                                            egui::Button::new((
                                                                egui::Image::from(open_icon)
                                                                    .atom_size(egui::Vec2::splat(
                                                                        20.,
                                                                    )),
                                                                egui::Atom::custom(
                                                                    custom_button_id,
                                                                    egui::vec2(
                                                                        desired_size + 8.,
                                                                        20.,
                                                                    ),
                                                                ),
                                                                egui::RichText::new(&address.uri)
                                                                    .size(16.)
                                                                    .into_atoms(),
                                                            ))
                                                            .truncate()
                                                            .image_tint_follows_text_color(true)
                                                            .frame(false)
                                                            .atom_ui(ui);

                                                        // Handle adding custom content
                                                        if let Some(rect) = saved_request_button
                                                            .rect(custom_button_id)
                                                        {
                                                            let saved_response_pip = ui.put(
                                                                rect,
                                                                Chip::new(
                                                                    &method,
                                                                    row_data.method.to_colour(
                                                                        ui.visuals().dark_mode,
                                                                    ),
                                                                )
                                                                .create(),
                                                            );

                                                            if saved_request_button
                                                                .response
                                                                .clicked()
                                                                || saved_response_pip.clicked()
                                                            {
                                                                *request = row_data.clone();
                                                                app.view = AppView::Request;
                                                            }
                                                        }
                                                    },
                                                );
                                            });
                                    }
                                }
                            });
                    }
                });
            })
            .response
    }
}
