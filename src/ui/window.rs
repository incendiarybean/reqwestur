use std::{f32::consts::PI, process::exit};

use eframe::{
    egui::{self, Color32, FontId, Pos2, Vec2, include_image},
    epaint::TextShape,
};
use egui_extras::syntax_highlighting::{self, code_view_ui};

use crate::utils::reqwestur::{BodyType, Header, Method, Reqwestur};

pub fn window(app: &mut Reqwestur, ui: &mut egui::Ui) {
    let default_expanded_size = if ui.available_width() < 500. {
        ui.available_width()
    } else {
        ui.available_width() / 3.
    };

    egui::TopBottomPanel::top("settings_panel").show(ui.ctx(), |ui| {
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
    });

    egui::SidePanel::new(egui::panel::Side::Left, "request_panel")
        .min_width(15.)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            if app.request_panel_minimised {
                // Draw the 90 degree label
                draw_angled_text(ui, "REQUEST OPTIONS");

                ui.set_width(23.);
            } else {
                ui.set_width(default_expanded_size - 58.);
            }

            ui.add_space(5.);

            ui.horizontal(|ui| {
                if !app.request_panel_minimised {
                    ui.heading("Request Settings");
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    let left_chevron_icon = include_image!("../assets/double_left_chevron.svg");
                    let right_chevron_icon = include_image!("../assets/double_right_chevron.svg");

                    let current_icon = egui::ImageButton::new(
                        egui::Image::new(if app.request_panel_minimised {
                            right_chevron_icon
                        } else {
                            left_chevron_icon
                        })
                        .fit_to_exact_size(Vec2 { x: 16., y: 16. })
                        .corner_radius(5.)
                        .alt_text("Show the Request Panel"),
                    )
                    .tint(ui.ctx().theme().default_visuals().text_color());

                    if ui.add(current_icon).clicked() {
                        app.request_panel_minimised = !app.request_panel_minimised;
                    }
                });
            });

            if !app.request_panel_minimised {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.add_space(2.);
                    let send_icon = include_image!("../assets/paper_plane.svg");
                    if ui
                        .add(
                            egui::Button::image_and_text(
                                egui::Image::new(send_icon)
                                    .fit_to_exact_size(Vec2 { x: 16., y: 16. })
                                    .tint(Color32::LIGHT_BLUE),
                                "Send Request",
                            )
                            .min_size(Vec2 {
                                x: ui.available_width(),
                                y: 0.,
                            }),
                        )
                        .clicked()
                    {
                        app.history.push(app.request.clone());
                    };

                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        // URL
                        padded_group(ui, |ui| {
                            ui.allocate_ui(
                                Vec2 {
                                    x: ui.available_width(),
                                    y: 0.,
                                },
                                |ui| {
                                    ui.label("Request URL:");
                                    ui.add_sized(
                                        Vec2 {
                                            x: ui.available_width(),
                                            y: 20.,
                                        },
                                        egui::TextEdit::singleline(&mut app.request.uri)
                                            .margin(5.)
                                            .hint_text("http://test.com"),
                                    );
                                },
                            );
                        });

                        // Method
                        padded_group(ui, |ui| {
                            ui.label("Request Method:");
                            egui::ComboBox::new("method_list", "Method")
                                .selected_text(app.request.method.to_string())
                                .show_ui(ui, |ui| {
                                    for method in Method::values() {
                                        ui.selectable_value(
                                            &mut app.request.method,
                                            method.clone(),
                                            method.to_string(),
                                        );
                                    }
                                });
                        });

                        // Request Body
                        if [Method::PATCH, Method::POST, Method::PUT].contains(&app.request.method)
                        {
                            padded_group(ui, |ui| {
                                ui.label("Request Body:");
                                let edit_icon = include_image!("../assets/pen.svg");
                                if ui
                                    .add(
                                        egui::Button::image_and_text(
                                            egui::Image::new(edit_icon)
                                                .fit_to_exact_size(Vec2 { x: 16., y: 16. })
                                                .tint(ui.ctx().style().visuals.text_color()),
                                            "Edit Body",
                                        )
                                        .min_size(Vec2 {
                                            x: ui.available_width(),
                                            y: 0.,
                                        }),
                                    )
                                    .clicked()
                                {
                                    app.body_editor_open = !app.body_editor_open
                                };
                            });
                        }

                        // Headers
                        padded_group(ui, |ui| {
                            ui.label("Request Headers:");
                            let edit_icon = include_image!("../assets/pen.svg");
                            if ui
                                .add(
                                    egui::Button::image_and_text(
                                        egui::Image::new(edit_icon)
                                            .fit_to_exact_size(Vec2 { x: 16., y: 16. })
                                            .tint(ui.ctx().style().visuals.text_color()),
                                        "Edit Headers",
                                    )
                                    .min_size(Vec2 {
                                        x: ui.available_width(),
                                        y: 0.,
                                    }),
                                )
                                .clicked()
                            {
                                app.headers_editor_open = !app.headers_editor_open
                            };
                        });
                    });
                });
            }
        });

    egui::SidePanel::new(egui::panel::Side::Right, "history_panel")
        .min_width(15.)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            if !app.history_panel_minimised {
                // Draw the 90 degree label
                draw_angled_text(ui, "HISTORY");

                ui.set_width(23.);
            } else {
                ui.set_width(default_expanded_size);
            }

            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    let left_chevron_icon = include_image!("../assets/double_left_chevron.svg");
                    let right_chevron_icon = include_image!("../assets/double_right_chevron.svg");

                    let current_icon = egui::ImageButton::new(
                        egui::Image::new(if app.history_panel_minimised {
                            right_chevron_icon
                        } else {
                            left_chevron_icon
                        })
                        .fit_to_exact_size(Vec2 { x: 16., y: 16. })
                        .corner_radius(5.)
                        .alt_text("Show the History Panel"),
                    )
                    .tint(ui.ctx().theme().default_visuals().text_color());

                    if ui.add(current_icon).clicked() {
                        app.history_panel_minimised = !app.history_panel_minimised;
                    }
                });

                if app.history_panel_minimised {
                    ui.heading("Previous Requests");
                }
            });

            if app.history_panel_minimised {
                let history_copy = app.history.clone();
                let num_rows = history_copy.len();

                ui.vertical(|ui| {
                    ui.add_space(5.);
                    if history_copy.len() == 0 {
                        padded_group(ui, |ui| {
                            ui.label("You haven't made any requests yet...");
                        });
                    } else {
                        ui.push_id("history_panel_list", |ui| {
                            egui::ScrollArea::new([false, false])
                                .max_height(ui.available_height())
                                .max_width(ui.available_width())
                                .auto_shrink(false)
                                .show_rows(ui, 18.0, num_rows, |ui, row_range| {
                                    for row in row_range {
                                        ui.group(|ui| {
                                            ui.allocate_space(Vec2 {
                                                x: ui.available_width(),
                                                y: 0.,
                                            });
                                            ui.horizontal(|ui| {
                                                if let Some(request) = history_copy.get(row) {
                                                    ui.label(
                                                        request.response.status_code.to_string(),
                                                    );
                                                    ui.label(request.method.to_string());
                                                    ui.label(request.uri.clone());
                                                }
                                            });
                                        });
                                    }
                                });
                        });
                    }
                });
            }
        });

    egui::CentralPanel::default().show(ui.ctx(), |ui| {
        ui.add_enabled_ui(!app.request.response.body.is_empty(), |ui| {
            ui.group(|ui| {
                ui.add_sized(
                    Vec2 {
                        x: ui.available_width(),
                        y: ui.available_height(),
                    },
                    egui::TextEdit::multiline(&mut app.request.response.body)
                        .code_editor()
                        .hint_text("You haven't made a request yet..."),
                )
            });
        });
    });

    if app.headers_editor_open {
        ui.ctx().show_viewport_immediate(
            egui::ViewportId::from_hash_of("headers_editor"),
            egui::ViewportBuilder::default()
                .with_title("Header Editor")
                .with_inner_size([500.0, 500.0]),
            |context, _class| {
                egui::CentralPanel::default().show(ui.ctx(), |ui| {
                    let add_icon = include_image!("../assets/add_button.svg");
                    if ui
                        .add(
                            egui::Button::image_and_text(
                                egui::Image::new(add_icon)
                                    .fit_to_exact_size(Vec2 { x: 16., y: 16. })
                                    .corner_radius(5.)
                                    .alt_text("Add a New Header")
                                    .tint(ui.ctx().theme().default_visuals().text_color()),
                                "Add a New Header",
                            )
                            .min_size(Vec2 {
                                x: ui.available_width(),
                                y: 0.,
                            }),
                        )
                        .clicked()
                    {
                        app.request.headers.push(Header::default());
                    }

                    ui.add_space(2.);
                    ui.separator();
                    ui.add_space(2.);

                    let row_len = app.request.headers.clone().len();
                    egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                        ui,
                        18.,
                        row_len,
                        |ui, num_rows| {
                            if num_rows.is_empty() {
                                padded_group(ui, |ui| {
                                    ui.label("You haven't added any headers yet.");
                                });
                            } else {
                                for row in num_rows {
                                    ui.group(|ui| {
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Min),
                                            |ui| {
                                                let add_icon =
                                                    include_image!("../assets/trash.svg");
                                                if ui
                                                    .add(egui::ImageButton::new(
                                                        egui::Image::new(add_icon)
                                                            .fit_to_exact_size(Vec2 {
                                                                x: 14.,
                                                                y: 14.,
                                                            })
                                                            .corner_radius(5.)
                                                            .alt_text("Delete this header")
                                                            .tint(Color32::RED),
                                                    ))
                                                    .clicked()
                                                {
                                                    app.request.headers.remove(row);
                                                };

                                                if let Some(header) =
                                                    app.request.headers.get_mut(row)
                                                {
                                                    ui.add(
                                                        egui::TextEdit::singleline(
                                                            &mut header.value,
                                                        )
                                                        .desired_width(ui.available_width() / 2.)
                                                        .margin(5.)
                                                        .hint_text("Header Name"),
                                                    );

                                                    ui.add(
                                                        egui::TextEdit::singleline(&mut header.key)
                                                            .desired_width(ui.available_width())
                                                            .margin(5.)
                                                            .hint_text("Header Value"),
                                                    );
                                                }
                                            },
                                        )
                                    });
                                }
                            }
                        },
                    );
                });

                if context.input(|i| i.viewport().close_requested()) {
                    app.headers_editor_open = false;
                }
            },
        );
    }

    if app.body_editor_open {
        ui.ctx().show_viewport_immediate(
            egui::ViewportId::from_hash_of("body_editor"),
            egui::ViewportBuilder::default()
                .with_title("Body Editor")
                .with_inner_size([500.0, 500.0]),
            |context, _class| {
                egui::CentralPanel::default().show(ui.ctx(), |ui| {
                    egui::ComboBox::new("body_type_dropdown", "Request Body Type")
                        .selected_text(app.request.body_type.to_string())
                        .show_ui(ui, |ui| {
                            for body_type in BodyType::values() {
                                ui.selectable_value(
                                    &mut app.request.body_type,
                                    body_type.clone(),
                                    body_type.to_string(),
                                );
                            }
                        });

                    ui.add_space(2.);
                    ui.separator();
                    ui.add_space(2.);

                    if app.request.body_type == BodyType::JSON {
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                            let theme =
                                syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());

                            // ui.collapsing("Theme", |ui| {
                            //     ui.group(|ui| {
                            //         theme.ui(ui);
                            //         theme.clone().store_in_memory(ui.ctx());
                            //     });
                            // });

                            let mut layouter = |ui: &egui::Ui, buf: &str, _| {
                                let layout_job = syntax_highlighting::highlight(
                                    ui.ctx(),
                                    ui.style(),
                                    &theme.clone(),
                                    buf,
                                    "JSON",
                                );
                                ui.fonts(|f| f.layout_job(layout_job))
                            };

                            // code_view_ui(ui, &theme, &mut app.request.body, "js");
                            ui.add_sized(
                                Vec2 {
                                    x: ui.available_width(),
                                    y: ui.available_height(),
                                },
                                egui::TextEdit::multiline(&mut app.request.body)
                                    .code_editor()
                                    .layouter(&mut layouter),
                            );
                        });
                    }

                    if context.input(|i| i.viewport().close_requested()) {
                        app.body_editor_open = false;
                    }
                });
            },
        );
    }
}

fn draw_angled_text(ui: &egui::Ui, text: &'static str) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter_at(rect);
    let txt_colour = ui.ctx().style().visuals.text_color();

    let label = TextShape::new(
        Pos2 {
            x: rect.center_top().x + 11.,
            y: rect.center_top().y + 35.,
        },
        ui.fonts(|f| {
            f.layout_no_wrap(
                text.to_string(),
                FontId {
                    size: 20.,
                    ..Default::default()
                },
                txt_colour,
            )
        }),
        txt_colour,
    )
    .with_angle(PI / 2.);

    painter.add(label);
}

fn padded_group(ui: &mut egui::Ui, add_content: impl FnOnce(&mut egui::Ui)) {
    ui.group(|ui| {
        ui.allocate_space(Vec2 {
            x: ui.available_width(),
            y: 0.,
        });

        ui.add_space(2.);

        add_content(ui);

        ui.add_space(2.);
    });
}
