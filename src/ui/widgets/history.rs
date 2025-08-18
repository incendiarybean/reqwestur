use eframe::egui::{self};

use crate::{
    ui::widgets::{buttons::default_button, chip::Chip},
    utils::{
        request::Request,
        reqwestur::{AppView, Reqwestur},
        traits::{ToColour, ToStringForeign},
    },
};

/// The view handling the user's previous requests
pub fn panel<'a>(app: &'a mut Reqwestur, request: &'a mut Request) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        let mut history = app.history.lock().unwrap();
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    if history.len() == 0 {
                        ui.label("You haven't made any requests yet!");
                    } else {
                        let bin_icon = egui::include_image!("../../assets/trash.svg");
                        if ui
                            .add(default_button(
                                Some(bin_icon),
                                "Clear History",
                                ui.available_width(),
                                ui.visuals().text_color(),
                            ))
                            .clicked()
                        {
                            *history = Vec::new();
                        }

                        ui.add_space(2.);
                        ui.separator();
                        ui.add_space(2.);

                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .max_height(ui.available_height())
                            .max_width(ui.available_width())
                            .show_rows(ui, 18., history.len(), |ui, row_range| {
                                for row in row_range.rev() {
                                    if let Some(row_data) = history.get(row) {
                                        egui::Frame::new()
                                            .stroke(egui::Stroke::new(
                                                1.,
                                                ui.visuals().noninteractive().bg_stroke.color,
                                            ))
                                            .inner_margin(egui::Vec2::splat(5.))
                                            .corner_radius(5.)
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.vertical(|ui| {
                                                        let open_icon = egui::include_image!(
                                                            "../../assets/folder_open.svg"
                                                        );
                                                        if ui
                                                            .add(egui::ImageButton::new(
                                                                egui::Image::new(open_icon)
                                                                    .tint(ui.visuals().text_color())
                                                                    .fit_to_exact_size(
                                                                        [16., 16.].into(),
                                                                    )
                                                                    .corner_radius(5.)
                                                                    .alt_text("View Request"),
                                                            ))
                                                            .clicked()
                                                        {
                                                            *request = row_data.clone();
                                                            app.view = AppView::Request;
                                                        }

                                                        let save_icon = egui::include_image!(
                                                            "../../assets/floppy.svg"
                                                        );
                                                        if ui
                                                            .add(egui::ImageButton::new(
                                                                egui::Image::new(save_icon)
                                                                    .tint(ui.visuals().text_color())
                                                                    .fit_to_exact_size(
                                                                        [16., 16.].into(),
                                                                    )
                                                                    .corner_radius(5.)
                                                                    .alt_text("Save Request"),
                                                            ))
                                                            .clicked()
                                                        {
                                                            let Request {
                                                                method,
                                                                headers,
                                                                address,
                                                                timestamp: _,
                                                                content_type,
                                                                body,
                                                                params,
                                                                response: _,
                                                                notification: _,
                                                                event: _,
                                                            } = row_data.clone();

                                                            app.saved_requests.push(Request {
                                                                method,
                                                                headers,
                                                                address,
                                                                content_type,
                                                                body,
                                                                params,
                                                                ..Default::default()
                                                            });
                                                        }
                                                    });

                                                    ui.vertical(|ui| {
                                                        ui.horizontal(|ui| {
                                                            Chip::new(
                                                                &row_data.method.to_string(),
                                                                row_data.method.to_colour(
                                                                    ui.visuals().dark_mode,
                                                                ),
                                                            )
                                                            .show(ui);

                                                            Chip::new(
                                                                &row_data
                                                                    .response
                                                                    .status
                                                                    .to_string(),
                                                                row_data.response.status.to_colour(
                                                                    ui.visuals().dark_mode,
                                                                ),
                                                            )
                                                            .show(ui);

                                                            ui.with_layout(
                                                                egui::Layout::right_to_left(
                                                                    egui::Align::RIGHT,
                                                                ),
                                                                |ui| {
                                                                    Chip::new(
                                                                        &row_data.timestamp.clone(),
                                                                        egui::Color32::ORANGE,
                                                                    )
                                                                    .show(ui);
                                                                },
                                                            );
                                                        });

                                                        ui.add(
                                                            egui::Label::new(
                                                                egui::RichText::new(
                                                                    &row_data.address.uri,
                                                                )
                                                                .size(14.),
                                                            )
                                                            .truncate(),
                                                        );
                                                    });
                                                });
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
