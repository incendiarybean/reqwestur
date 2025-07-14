use eframe::egui::{self};
use std::process::exit;

use crate::{
    ui::widgets::{self, default_button},
    utils::{
        self,
        reqwestur::{
            BodyType, Certificates, CertificatesStatus, Method, Notification, NotificationKind,
            Reqwestur,
        },
    },
};

pub fn window(app: &mut Reqwestur, ui: &mut egui::Ui) {
    let max_width = if ui.available_width() < 500. {
        ui.available_width()
    } else {
        ui.available_width() / 3.
    };

    /////////////
    // Main UI //
    /////////////
    ui.add(task_bar());
    ui.add(request_panel(app, max_width));
    ui.add(history_panel(app, max_width));
    ui.add(viewer_panel(app));

    //////////////////////
    // Editors / Modals //
    //////////////////////
    if app.headers_editor_open {
        header_editor(app, ui);
    }

    if app.payload_editor_open {
        payload_editor(app, ui);
    }

    if app.certificate_editor_open {
        certificate_editor(app, ui)
    }

    if app.about_modal_open {
        about_modal(app, ui);
    }

    if app.help_modal_open {
        about_modal(app, ui);
    }
}

fn task_bar() -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        egui::TopBottomPanel::top("settings_panel")
            .show(ui.ctx(), |ui| {
                ui.add_space(2.);

                ui.horizontal(|ui| {
                    egui::menu::menu_button(ui, "File", |ui| {
                        ui.menu_button("Export", |ui| {
                            if ui.button("History").clicked() {};
                            if ui.button("Requests").clicked() {};
                        });

                        ui.menu_button("Import", |ui| {
                            if ui.button("History").clicked() {};
                            if ui.button("Requests").clicked() {};
                        });

                        if ui.button("Save Request").clicked() {};

                        if ui.button("Exit").clicked() {
                            exit(1);
                        };
                    });

                    egui::menu::menu_button(ui, "Help", |ui| {
                        if ui.button("Guidance").clicked() {};
                        if ui.button("About").clicked() {};
                    });
                });

                ui.add_space(2.);
            })
            .response
    }
}

fn request_panel<'a>(app: &'a mut Reqwestur, max_width: f32) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        egui::SidePanel::new(egui::panel::Side::Left, "request_panel")
            .min_width(15.)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                if app.request_panel_minimised {
                    // Draw the 90 degree label
                    widgets::draw_vertical_text(ui, "REQUEST OPTIONS");

                    ui.set_width(23.);
                } else {
                    ui.set_width(max_width - 58.);
                }

                ui.add_space(5.);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui
                            .add(widgets::minimiser(
                                widgets::MinimiserDirection::LeftToRight,
                                app.request_panel_minimised,
                            ))
                            .clicked()
                        {
                            app.request_panel_minimised = !app.request_panel_minimised;
                        }
                        if !app.request_panel_minimised {
                            ui.vertical(|ui| {
                                ui.add_space(5.);
                                ui.heading(
                                    egui::RichText::new("Request Options").heading().size(16.),
                                );
                            });
                        }
                    })
                });

                if !app.request_panel_minimised {
                    app.check_sendable();

                    ui.with_layout(
                        egui::Layout::bottom_up(egui::Align::Min),
                        |ui: &mut egui::Ui| {
                            ui.add_space(10.);

                            let send_icon = egui::include_image!("../assets/reuse.svg");
                            if ui
                                .add_enabled(
                                    app.request.sendable,
                                    widgets::default_button(
                                        Some(send_icon),
                                        "Send!",
                                        ui.visuals().text_color(),
                                        ui.available_width(),
                                    ),
                                )
                                .clicked()
                            {
                                let _ = app.send();
                            }

                            ui.add(widgets::display_notification(&app.request.notification));

                            ui.vertical(|ui| {
                                ui.separator();
                                ui.add_space(1.);

                                widgets::padded_group(ui, |ui| {
                                    ui.label(egui::RichText::new("Request URL").size(14.));
                                    if ui
                                        .add(
                                            egui::TextEdit::singleline(
                                                &mut app.request.address.uri,
                                            )
                                            .min_size(egui::vec2(ui.available_width(), 10.))
                                            .hint_text("http://test.com")
                                            .margin(5.),
                                        )
                                        .changed()
                                    {
                                        if let Err(error) =
                                            reqwest::Url::parse(&app.request.address.uri)
                                        {
                                            app.request.address.notification = Some(Notification {
                                                kind: NotificationKind::ERROR,
                                                message: format!(
                                                    "URL cannot be parsed: {}!",
                                                    error
                                                ),
                                            })
                                        } else {
                                            app.request.address.notification = None;
                                        }
                                    }

                                    ui.add(widgets::display_notification(
                                        &app.request.address.notification,
                                    ));
                                });

                                widgets::padded_group(ui, |ui| {
                                    ui.label(egui::RichText::new("Request Method").size(14.));

                                    egui::ComboBox::new("request_method", "Select the Method:")
                                        .selected_text(app.request.method.to_string())
                                        .show_ui(ui, |ui| {
                                            for method in Method::values() {
                                                app.request.body_type = BodyType::EMPTY;
                                                app.request.body = None;

                                                ui.selectable_value(
                                                    &mut app.request.method,
                                                    method.clone(),
                                                    method.to_string(),
                                                );
                                            }
                                        });

                                    if [Method::PATCH, Method::POST, Method::PUT]
                                        .contains(&app.request.method)
                                    {
                                        let edit_icon = egui::include_image!("../assets/reuse.svg");
                                        if ui
                                            .add(widgets::default_button(
                                                Some(edit_icon),
                                                "Payload Management",
                                                ui.visuals().text_color(),
                                                ui.available_width(),
                                            ))
                                            .clicked()
                                        {
                                            app.payload_editor_open = true;
                                        }
                                    }
                                });

                                widgets::padded_group(ui, |ui| {
                                    ui.label(egui::RichText::new("Request Headers").size(14.));

                                    let edit_icon = egui::include_image!("../assets/reuse.svg");
                                    if ui
                                        .add(widgets::default_button(
                                            Some(edit_icon),
                                            "Header Management",
                                            ui.visuals().text_color(),
                                            ui.available_width(),
                                        ))
                                        .clicked()
                                    {
                                        app.headers_editor_open = true;
                                    }
                                });

                                widgets::padded_group(ui, |ui| {
                                    ui.label(egui::RichText::new("Certificates").size(14.));

                                    if ui
                                        .checkbox(
                                            &mut app.certificates.required,
                                            "Use Certificates?",
                                        )
                                        .changed()
                                    {
                                        if !app.certificates.required {
                                            app.certificates = Certificates::default();
                                        }
                                    }

                                    if app.certificates.required {
                                        let edit_icon = egui::include_image!("../assets/reuse.svg");
                                        if ui
                                            .add(widgets::default_button(
                                                Some(edit_icon),
                                                "Certificate Management",
                                                ui.visuals().text_color(),
                                                ui.available_width(),
                                            ))
                                            .clicked()
                                        {
                                            app.certificate_editor_open = true;
                                        }
                                    }

                                    ui.add(widgets::display_notification(
                                        &app.certificates.notification,
                                    ));
                                });
                            });
                        },
                    );
                }
            })
            .response
    }
}

fn history_panel<'a>(app: &'a mut Reqwestur, max_width: f32) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        egui::SidePanel::new(egui::panel::Side::Right, "history_panel")
            .min_width(15.)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                if app.history_panel_minimised {
                    // Draw the 90 degree label
                    widgets::draw_vertical_text(ui, "HISTORIC REQUESTS");

                    ui.set_width(23.);
                } else {
                    ui.set_width(max_width);
                }

                ui.add_space(5.);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        if ui
                            .add(widgets::minimiser(
                                widgets::MinimiserDirection::RightToLeft,
                                app.history_panel_minimised,
                            ))
                            .clicked()
                        {
                            app.history_panel_minimised = !app.history_panel_minimised;
                        }
                    });

                    if !app.history_panel_minimised {
                        ui.label(egui::RichText::new("History").heading().size(16.));
                    }
                });

                ui.vertical(|ui| {
                    ui.add_space(1.);

                    if !app.history_panel_minimised {
                        ui.separator();

                        if app.history.len() == 0 {
                            ui.label("You haven't made any requests yet!");
                        } else {
                            let bin_icon = egui::include_image!("../assets/reuse.svg");
                            if ui
                                .add(widgets::default_button(
                                    Some(bin_icon),
                                    "Clear History",
                                    ui.visuals().text_color(),
                                    ui.available_width(),
                                ))
                                .clicked()
                            {
                                app.history = Vec::new();
                            }

                            egui::ScrollArea::vertical()
                                .auto_shrink(false)
                                .max_height(ui.available_height())
                                .show_rows(ui, 18., app.history.len(), |ui, row_range| {
                                    for row in row_range {
                                        if let Some(row_data) = app.history.get(row) {
                                            widgets::padded_group(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    let open_icon =
                                                        egui::include_image!("../assets/reuse.svg");
                                                    if ui
                                                        .add(egui::ImageButton::new(
                                                            egui::Image::new(open_icon)
                                                                .tint(ui.visuals().text_color())
                                                                .fit_to_exact_size(
                                                                    [16., 16.].into(),
                                                                )
                                                                .corner_radius(5.),
                                                        ))
                                                        .clicked()
                                                    {
                                                        app.request = row_data.clone();
                                                    }

                                                    ui.vertical(|ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label(
                                                                egui::RichText::new(
                                                                    row_data.method.to_string(),
                                                                )
                                                                .strong(),
                                                            );

                                                            let status_colour =
                                                                utils::common::status_colour(
                                                                    &row_data.response.status_code,
                                                                );
                                                            ui.label(
                                                                egui::RichText::new(
                                                                    row_data
                                                                        .response
                                                                        .status_code
                                                                        .to_string(),
                                                                )
                                                                .color(status_colour),
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
                    }
                });
            })
            .response
    }
}

fn viewer_panel<'a>(app: &'a mut Reqwestur) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    let enabled_editor = !app.request.response.body.is_empty();
                    let response = app.request.response.clone();

                    if !enabled_editor {
                        ui.label("You haven't made a request yet.");
                    } else {
                        widgets::padded_group(ui, |ui| {
                            ui.horizontal(|ui| {
                                let status_colour =
                                    utils::common::status_colour(&response.status_code);

                                ui.label("Response Status:");
                                ui.label(
                                    egui::RichText::new(response.status_code.to_string())
                                        .color(status_colour),
                                );
                            });
                        });

                        widgets::padded_group(ui, |ui| {
                            egui::CollapsingHeader::new("Response Headers").show_unindented(
                                ui,
                                |ui| {
                                    for (name, value) in &mut app.request.response.headers {
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Min),
                                            |ui| {
                                                ui.add(
                                                    egui::TextEdit::singleline(value)
                                                        .desired_width(ui.available_width() / 2.)
                                                        .margin(5.),
                                                );
                                                ui.add(egui::TextEdit::singleline(name).margin(5.));
                                            },
                                        );
                                    }
                                },
                            );
                        });

                        ui.group(|ui| {
                            egui::ScrollArea::both()
                                .auto_shrink(false)
                                .max_height(ui.available_height())
                                .max_width(ui.available_width())
                                .drag_to_scroll(false)
                                .show(ui, |ui| {
                                    let theme =
                                        egui_extras::syntax_highlighting::CodeTheme::from_memory(
                                            ui.ctx(),
                                            ui.style(),
                                        );
                                    let mut layouter = |ui: &egui::Ui, buf: &str, _| {
                                        let layout_job =
                                            egui_extras::syntax_highlighting::highlight(
                                                ui.ctx(),
                                                ui.style(),
                                                &theme.clone(),
                                                buf,
                                                "json",
                                            );
                                        ui.fonts(|f| f.layout_job(layout_job))
                                    };

                                    ui.add(
                                        egui::TextEdit::multiline(&mut app.request.response.body)
                                            .code_editor()
                                            .layouter(&mut layouter)
                                            .desired_width(ui.available_width()),
                                    );
                                });
                        });
                    }
                });
            })
            .response
    }
}

fn header_editor(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("header_editor"),
        egui::ViewportBuilder::default()
            .with_title("Header Editor")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                let add_icon = egui::include_image!("../assets/reuse.svg");
                if ui
                    .add(widgets::default_button(
                        Some(add_icon),
                        "New Header",
                        ui.visuals().text_color(),
                        ui.available_width(),
                    ))
                    .clicked()
                {
                    app.request
                        .headers
                        .push((String::default(), String::default()));
                }

                ui.add_space(2.);
                ui.separator();
                ui.add_space(2.);

                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .max_height(ui.available_height())
                    .max_width(ui.available_width())
                    .show_rows(ui, 18., app.request.headers.len(), |ui, row_range| {
                        for row in row_range {
                            ui.group(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Min),
                                    |ui| {
                                        if let Some((name, value)) =
                                            app.request.headers.get_mut(row)
                                        {
                                            let name_editor = egui::TextEdit::singleline(name)
                                                .hint_text("Header Name")
                                                .margin(5.)
                                                .vertical_align(egui::Align::Center);

                                            let value_editor = egui::TextEdit::singleline(value)
                                                .hint_text("Header Value")
                                                .margin(5.)
                                                .min_size(egui::vec2(
                                                    ui.available_width() / 2. - 5.,
                                                    25.,
                                                ))
                                                .vertical_align(egui::Align::Center);

                                            ui.add(value_editor);
                                            ui.add(name_editor);
                                        }
                                    },
                                );
                            });
                        }
                    });
            });

            if context.input(|i| i.viewport().close_requested()) {
                app.headers_editor_open = false;
            }
        },
    );
}

fn payload_editor(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("payload_editor"),
        egui::ViewportBuilder::default()
            .with_title("Payload Editor")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                egui::ComboBox::new("body_type_dropdown", "Payload Type")
                    .selected_text(&app.request.body_type.to_string())
                    .show_ui(ui, |ui| {
                        for body_type in BodyType::values() {
                            ui.selectable_value(
                                &mut app.request.body_type,
                                body_type.clone(),
                                body_type.to_string(),
                            );
                        }
                    });

                ui.separator();
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    if ui
                        .add(default_button(
                            None,
                            "Done!",
                            ui.visuals().text_color(),
                            ui.available_width(),
                        ))
                        .clicked()
                    {
                        app.payload_editor_open = false;
                    }

                    match app.request.body_type {
                        BodyType::EMPTY => {
                            app.request.body = None;
                        }
                        BodyType::MULTIPART => todo!(),
                        BodyType::JSON => {
                            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(
                                ui.ctx(),
                                ui.style(),
                            );
                            let mut layouter = |ui: &egui::Ui, buf: &str, _| {
                                let layout_job = egui_extras::syntax_highlighting::highlight(
                                    ui.ctx(),
                                    ui.style(),
                                    &theme.clone(),
                                    buf,
                                    "json",
                                );
                                ui.fonts(|f| f.layout_job(layout_job))
                            };

                            match &mut app.request.body {
                                Some(body) => {
                                    ui.add_sized(
                                        egui::vec2(ui.available_width(), ui.available_height()),
                                        egui::TextEdit::multiline(body)
                                            .code_editor()
                                            .layouter(&mut layouter)
                                            .desired_width(ui.available_width()),
                                    );
                                }
                                _ => app.request.body = Some(String::default()),
                            }
                        }
                    }
                });
            });

            if context.input(|i| i.viewport().close_requested()) {
                app.payload_editor_open = false;
            }
        },
    );
}

fn certificate_editor(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("certificate_editor"),
        egui::ViewportBuilder::default()
            .with_title("Certificate Editor")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    let save_icon = egui::include_image!("../assets/reuse.svg");
                    if ui
                        .add_enabled(
                            app.certificates.status == CertificatesStatus::OK,
                            default_button(
                                Some(save_icon.clone()),
                                "Confirm & Close!",
                                ui.visuals().text_color(),
                                ui.available_width(),
                            ),
                        )
                        .clicked()
                    {
                        app.certificate_editor_open = false;
                        app.certificates.notification = None;
                    }

                    if ui
                        .add_enabled(
                            !app.certificates.passphrase.is_empty()
                                && app.certificates.file_path.file_name().is_some(),
                            default_button(
                                Some(save_icon),
                                "Validate Certificates",
                                ui.visuals().text_color(),
                                ui.available_width(),
                            ),
                        )
                        .clicked()
                    {
                        match utils::common::load_certificates(
                            &app.certificates.file_path,
                            &app.certificates.passphrase,
                        ) {
                            Ok(identity) => {
                                app.certificates.status = CertificatesStatus::OK;
                                app.certificates.notification = Some(Notification {
                                    kind: NotificationKind::INFO,
                                    message: "Certificate loaded successfully!".to_string(),
                                });
                                app.certificates.identity = Some(identity);
                            }
                            Err(error) => {
                                app.certificates.status = CertificatesStatus::ERROR;
                                app.certificates.notification = Some(Notification {
                                    kind: NotificationKind::ERROR,
                                    message: error,
                                });
                            }
                        }
                    }

                    ui.add(widgets::display_notification(
                        &app.certificates.notification,
                    ));

                    if app.certificates.status == CertificatesStatus::UNCONFIRMED {
                        ui.label("No certificates have been loaded.");
                    }

                    ui.vertical(|ui| {
                        widgets::padded_group(ui, |ui| {
                            let file_name = match app.certificates.file_path.file_name() {
                                Some(file_name) => {
                                    file_name.to_str().unwrap_or("Error reading file name.")
                                }
                                None => "No file selected.",
                            };
                            ui.label(format!("Selected File: {}", file_name));

                            let upload_icon = egui::include_image!("../assets/reuse.svg");
                            if ui
                                .add(default_button(
                                    Some(upload_icon),
                                    "Select a PFX.",
                                    ui.visuals().text_color(),
                                    ui.available_width(),
                                ))
                                .clicked()
                            {
                                if let Some(file) = rfd::FileDialog::new()
                                    .add_filter("PFX", &["pfx"])
                                    .set_directory("/")
                                    .pick_file()
                                {
                                    app.certificates.file_path = file;
                                }
                            }

                            if app.certificates.file_path.exists() {
                                let bin_icon = egui::include_image!("../assets/reuse.svg");
                                if ui
                                    .add_enabled(
                                        app.certificates.status == CertificatesStatus::OK,
                                        default_button(
                                            Some(bin_icon),
                                            "Remove PFX.",
                                            ui.visuals().text_color(),
                                            ui.available_width(),
                                        ),
                                    )
                                    .clicked()
                                {
                                    app.certificates = Certificates {
                                        required: app.certificates.required,
                                        ..Default::default()
                                    };
                                }
                            }
                        });

                        if app.certificates.file_path.exists() {
                            widgets::padded_group(ui, |ui| {
                                ui.label("Certificate Passphrase:");
                                ui.add_sized(
                                    egui::vec2(ui.available_width(), 20.),
                                    egui::TextEdit::singleline(&mut app.certificates.passphrase)
                                        .margin(5.)
                                        .password(true),
                                );
                            });
                        }
                    });
                });

                ui.add_space(5.);
            });

            if context.input(|i| i.viewport().close_requested()) {
                app.certificate_editor_open = false;
            }
        },
    );
}

fn help_modal(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("help_modal"),
        egui::ViewportBuilder::default()
            .with_title("Help")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                ui.heading("Help");
            });

            if context.input(|i| i.viewport().close_requested()) {
                app.help_modal_open = false;
            }
        },
    );
}

fn about_modal(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("about_modal"),
        egui::ViewportBuilder::default()
            .with_title("About")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                ui.heading("About");
            });

            if context.input(|i| i.viewport().close_requested()) {
                app.about_modal_open = false;
            }
        },
    );
}
