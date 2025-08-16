use eframe::egui::{self, scroll_area::ScrollSource};

use crate::{
    ui::widgets::{
        buttons::default_button,
        notification::{Notification, NotificationKind},
    },
    utils::{
        request::{ContentType, Method, Request},
        reqwestur::Reqwestur,
    },
};

/// The view handling the user request & associated settings
pub fn panel<'a>(app: &'a mut Reqwestur, request: &'a mut Request) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        // Adjust the ensure the menu is always visible, but so is the content
        let size_adjust = 250.;

        // Create a Menu Panel
        egui::SidePanel::new(egui::panel::Side::Left, "request_panel")
            .resizable(true)
            .min_width(size_adjust)
            .max_width(ui.available_width() - 250.)
            .default_width(ui.available_width() / 2.)
            .show(ui.ctx(), |ui| {
                ui.add_space(5.);

                egui::ScrollArea::vertical()
                    .scroll_source(ScrollSource {
                        scroll_bar: true,
                        drag: false,
                        mouse_wheel: true,
                    })
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                            ui.add_space(1.);

                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    egui::ComboBox::from_id_salt("request_method")
                                        .selected_text(request.method.to_string())
                                        .show_ui(ui, |ui| {
                                            for method in Method::values() {
                                                request.content_type = ContentType::EMPTY;
                                                request.body = None;

                                                ui.selectable_value(
                                                    &mut request.method,
                                                    method.clone(),
                                                    method.to_string(),
                                                );
                                            }
                                        });

                                    if ui
                                        .add(
                                            egui::TextEdit::singleline(&mut request.address.uri)
                                                .min_size(egui::vec2(ui.available_width(), 10.))
                                                .hint_text("Request URL, e.g. http://test.com")
                                                .margin(5.),
                                        )
                                        .changed()
                                    {
                                        if let Err(error) =
                                            reqwest::Url::parse(&request.address.uri)
                                        {
                                            request.address.notification = Notification::new(
                                                format!("URL cannot be parsed: {}!", error),
                                                NotificationKind::ERROR,
                                            );
                                        } else {
                                            request.address.notification.clear();
                                        }
                                    }
                                });

                                request.address.notification.display(ui);
                            });

                            if [Method::PATCH, Method::POST, Method::PUT].contains(&request.method)
                            {
                                ui.group(|ui| {
                                    let edit_icon = egui::include_image!("../../assets/pen.svg");
                                    if ui
                                        .add(default_button(
                                            Some(edit_icon),
                                            "Payload Management",
                                            ui.available_width(),
                                            ui.visuals().text_color(),
                                        ))
                                        .clicked()
                                    {
                                        app.payload_editor_open = true;
                                    }
                                });
                            }

                            ui.group(|ui| {
                                ui.label(egui::RichText::new("Request Headers").size(14.));

                                let edit_icon = egui::include_image!("../../assets/pen.svg");
                                if ui
                                    .add(default_button(
                                        Some(edit_icon),
                                        "Header Management",
                                        ui.available_width(),
                                        ui.visuals().text_color(),
                                    ))
                                    .clicked()
                                {
                                    app.header_editor_open = true;
                                }
                            });

                            let size = ui
                                .group(|ui| {
                                    ui.label(egui::RichText::new("Certificates").size(14.));

                                    ui.checkbox(
                                        &mut app.use_certificate_authentication,
                                        "Use Certificates?",
                                    );

                                    if app.use_certificate_authentication {
                                        let edit_icon =
                                            egui::include_image!("../../assets/pen.svg");
                                        if ui
                                            .add(default_button(
                                                Some(edit_icon),
                                                "Certificate Management",
                                                ui.available_width(),
                                                ui.visuals().text_color(),
                                            ))
                                            .clicked()
                                        {
                                            app.certificate_editor_open = true;
                                        }
                                    } else {
                                        app.certificate = None;
                                    }
                                })
                                .response
                                .rect;

                            ui.allocate_ui_with_layout(
                                egui::vec2(
                                    ui.available_width(),
                                    if ui.available_height() - size.height() <= size.height() {
                                        ui.available_height() + size.height()
                                    } else {
                                        ui.available_height()
                                    },
                                ),
                                egui::Layout::bottom_up(egui::Align::Min),
                                |ui: &mut egui::Ui| {
                                    ui.add_space(5.);

                                    let send_icon =
                                        egui::include_image!("../../assets/paper_plane.svg");
                                    if ui
                                        .add_enabled(
                                            true,
                                            default_button(
                                                Some(send_icon),
                                                "Send!",
                                                ui.available_width(),
                                                ui.visuals().text_color(),
                                            ),
                                        )
                                        .clicked()
                                    {
                                        let mut app_clone = app.clone();
                                        std::thread::spawn(move || {
                                            match app_clone.send() {
                                                Ok(_) => {
                                                    app_clone.history.lock().unwrap().push(
                                                        app_clone.request.lock().unwrap().clone(),
                                                    );
                                                }
                                                Err(_) => {
                                                    todo!("Send the error to notification mutex.")
                                                }
                                            };
                                        });
                                    }

                                    request.notification.display(ui);
                                },
                            );
                        });
                    });
            })
            .response
    }
}
