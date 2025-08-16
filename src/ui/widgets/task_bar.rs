use std::process::exit;

use eframe::egui::{
    self,
    gui_zoom::kb_shortcuts::{ZOOM_IN, ZOOM_OUT},
};

use crate::{
    ui::{
        widgets::{
            buttons::toggle_switch,
            notification::{Notification, NotificationKind},
        },
        window::PRIMARY,
    },
    utils::{
        exports::{ExportType, RequestSourceType, ReqwesturIO},
        request::Request,
        reqwestur::Reqwestur,
    },
};

/// The title and toolbar at the top of the screen
pub fn task_bar(app: &mut Reqwestur, request: &mut Request) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        let frame = egui::Frame {
            fill: egui::Color32::from_hex(PRIMARY).unwrap(),
            ..Default::default()
        };
        egui::TopBottomPanel::top("settings_panel")
            .frame(frame)
            .show(ui.ctx(), |ui| {
                ui.scope(|ui| {
                    ui.style_mut().spacing.button_padding = egui::vec2(8., 5.);

                    ui.visuals_mut().widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
                    ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
                    ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    ui.visuals_mut().widgets.open.weak_bg_fill = egui::Color32::TRANSPARENT;

                    ui.visuals_mut().widgets.open.bg_stroke =
                        egui::Stroke::new(1., egui::Color32::WHITE);
                    ui.visuals_mut().widgets.hovered.bg_stroke =
                        egui::Stroke::new(1., egui::Color32::WHITE);
                    ui.visuals_mut().widgets.active.bg_stroke =
                        egui::Stroke::new(1., egui::Color32::WHITE);
                    ui.visuals_mut().widgets.inactive.bg_stroke =
                        egui::Stroke::new(1., egui::Color32::TRANSPARENT);

                    ui.visuals_mut().widgets.inactive.fg_stroke.color = egui::Color32::WHITE;
                    ui.visuals_mut().widgets.active.fg_stroke.color = egui::Color32::WHITE;
                    ui.visuals_mut().widgets.hovered.fg_stroke.color = egui::Color32::WHITE;

                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: 5,
                            right: 5,
                            top: 5,
                            bottom: 2,
                        })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let logo = egui::include_image!("../../assets/reqwestur-lg.png");
                                ui.add(
                                    egui::Image::from(logo)
                                        .alt_text("Reqwestur Logo")
                                        .fit_to_original_size(0.12),
                                );

                                egui::Frame::new()
                                    .stroke(egui::Stroke::new(1., egui::Color32::WHITE))
                                    .corner_radius(5.)
                                    .outer_margin(egui::vec2(4., 10.))
                                    .inner_margin(egui::vec2(4., 2.))
                                    .show(ui, |ui| {
                                        ui.label(
                                            egui::RichText::new("0.0.1")
                                                .color(egui::Color32::WHITE),
                                        )
                                    });
                            });
                        });

                    ui.scope(|ui| {
                        ui.visuals_mut().widgets.noninteractive.bg_stroke = egui::Stroke {
                            width: 0.5,
                            color: egui::Color32::LIGHT_BLUE,
                        };
                        ui.add(egui::Separator::default().horizontal().spacing(0.));
                    });

                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: 5,
                            right: 5,
                            top: 5,
                            bottom: 5,
                        })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                egui::containers::menu::MenuButton::new("File")
                                    .config(
                                        egui::containers::menu::MenuConfig::default()
                                            .close_behavior(
                                                egui::PopupCloseBehavior::CloseOnClickOutside,
                                            ),
                                    )
                                    .ui(ui, |ui| {
                                        ui.menu_button("Export", |ui| {
                                            ui.menu_button("History", |ui| {
                                                for option in ExportType::values() {
                                                    if ui
                                                        .button(option.to_string().to_uppercase())
                                                        .clicked()
                                                    {
                                                        let history = app.history.lock().unwrap();
                                                        if let Some(output) = ReqwesturIO::new(
                                                            history.clone(),
                                                            RequestSourceType::HISTORY,
                                                            option,
                                                        ) {
                                                            let _ = output.export();
                                                        }
                                                    }
                                                }
                                            });
                                            ui.menu_button("Requests", |ui| {
                                                for option in ExportType::values() {
                                                    if ui
                                                        .button(option.to_string().to_uppercase())
                                                        .clicked()
                                                    {
                                                        if let Some(output) = ReqwesturIO::new(
                                                            app.saved_requests.clone(),
                                                            RequestSourceType::SAVED,
                                                            option,
                                                        ) {
                                                            let _ = output.export();
                                                        }
                                                    }
                                                }
                                            });
                                        });

                                        ui.menu_button("Import", |ui| {
                                            if ui.button("History").clicked() {};
                                            if ui.button("Requests").clicked() {};
                                        });

                                        if ui.button("Save Request").clicked() {
                                            if request.address.uri.is_empty() {
                                                app.notification = Notification::new(
                                                    "Failed to save request!",
                                                    NotificationKind::WARN,
                                                );
                                            } else {
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
                                                } = request.clone();

                                                app.saved_requests.push(Request {
                                                    method,
                                                    headers,
                                                    address,
                                                    content_type,
                                                    body,
                                                    params,
                                                    ..Default::default()
                                                });
                                                app.notification = Notification::new(
                                                    "Saved request successfully!",
                                                    NotificationKind::INFO,
                                                );
                                            }
                                        };

                                        if ui.button("Exit").clicked() {
                                            exit(1);
                                        };
                                    });

                                egui::containers::menu::MenuButton::new("Accessibility")
                                    .config(
                                        egui::containers::menu::MenuConfig::default()
                                            .close_behavior(
                                                egui::PopupCloseBehavior::CloseOnClickOutside,
                                            ),
                                    )
                                    .ui(ui, |ui| {
                                        ui.add_space(2.);

                                        ui.horizontal(|ui| {
                                            if ui
                                                .add(toggle_switch(
                                                    &mut app.is_dark_mode,
                                                    "Dark Mode",
                                                ))
                                                .changed()
                                            {
                                                let (text_colour, theme) = if app.is_dark_mode {
                                                    (egui::Color32::WHITE, egui::Theme::Dark)
                                                } else {
                                                    (egui::Color32::BLACK, egui::Theme::Light)
                                                };

                                                ui.ctx().set_theme(theme);
                                                ui.visuals_mut().override_text_color =
                                                    Some(text_colour);
                                            }
                                        });

                                        ui.add_space(2.);
                                        ui.separator();
                                        ui.add_space(2.);

                                        let current_scale_percentage =
                                            (ui.ctx().pixels_per_point() / 1. * 100.).floor();
                                        ui.label(format!(
                                            "Current Scale: {current_scale_percentage}%"
                                        ));

                                        let zoom_in_btn = egui::Button::new("Zoom In")
                                            .shortcut_text(ui.ctx().format_shortcut(&ZOOM_IN));
                                        if ui.add(zoom_in_btn).clicked() {
                                            ui.ctx().set_pixels_per_point(
                                                ui.ctx().pixels_per_point() + 0.1,
                                            );
                                        }

                                        let zoom_out_btn = egui::Button::new("Zoom Out")
                                            .shortcut_text(ui.ctx().format_shortcut(&ZOOM_OUT));
                                        if ui.add(zoom_out_btn).clicked() {
                                            ui.ctx().set_pixels_per_point(
                                                ui.ctx().pixels_per_point() - 0.1,
                                            );
                                        }
                                    });

                                egui::containers::menu::MenuButton::new("Help")
                                    .config(
                                        egui::containers::menu::MenuConfig::default()
                                            .close_behavior(
                                                egui::PopupCloseBehavior::CloseOnClickOutside,
                                            ),
                                    )
                                    .ui(ui, |ui| {
                                        if ui.button("Guidance").clicked() {};
                                        if ui.button("About").clicked() {
                                            app.about_modal_open = true;
                                        };
                                    });
                            });
                        });
                });
            })
            .response
    }
}
