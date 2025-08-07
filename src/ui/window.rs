use eframe::egui::{
    self,
    gui_zoom::kb_shortcuts::{ZOOM_IN, ZOOM_OUT},
    scroll_area::ScrollSource,
};
use std::process::exit;

const PRIMARY: &'static str = "#1b3c79";
const _SECONDARY: &'static str = "#112e65";

use crate::{
    ui::widgets::{
        buttons::{MinimiserDirection, default_button, minimiser, side_menu_button, toggle_switch},
        frames::padded_group,
        notification::{Notification, NotificationKind},
        pip::pip,
    },
    utils::{
        certificates::{Certificates, CertificatesStatus},
        request::{ContentType, Method, Request},
        reqwestur::{AppShortcuts, AppView, Reqwestur},
        traits::ToColour,
    },
};

/// Main Window controller of the UI
pub fn window(app: &mut Reqwestur, ui: &mut egui::Ui, shortcuts: AppShortcuts) {
    register_keyboard_shortcuts(app, ui, shortcuts);

    let max_width = if ui.available_width() < 500. {
        ui.available_width()
    } else {
        ui.available_width() / 3.
    };

    /////////////
    // Main UI //
    /////////////

    ui.add(task_bar(app));
    ui.add(menu_panel(app, max_width));

    match app.view {
        AppView::Main => {
            ui.add(home_panel(app));
        }
        AppView::Request => {
            ui.add(request_panel(app));
            ui.add(viewer_panel(app));
        }
        AppView::Saved => {
            todo!()
        }
        AppView::History => {
            ui.add(history_panel(app));
        }
    }

    //////////////////////
    // Editors / Modals //
    //////////////////////

    if app.header_editor_open {
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
        help_modal(app, ui);
    }
}

/// Register the keyboard shortcuts
fn register_keyboard_shortcuts(app: &mut Reqwestur, ui: &mut egui::Ui, shortcuts: AppShortcuts) {
    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.new)) {
        app.request = Request::default();
        app.view = AppView::Request;
    }

    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.history)) {
        app.view = AppView::History;
    }

    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.save)) {
        todo!("Create a save option!")
    }

    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.open)) {
        app.view = AppView::Saved;
    }

    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.hide_menu)) {
        app.menu_minimised = !app.menu_minimised;
    }
}

/// The title and toolbar at the top of the screen
fn task_bar(app: &mut Reqwestur) -> impl egui::Widget {
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
                                let logo = egui::include_image!("../assets/reqwestur-lg.png");
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

fn menu_panel<'a>(app: &'a mut Reqwestur, max_width: f32) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        let frame = egui::Frame {
            inner_margin: egui::Margin {
                left: 5,
                right: 5,
                top: 2,
                bottom: 2,
            },
            fill: if ui.style().visuals.dark_mode {
                egui::Color32::BLACK
            } else {
                egui::Color32::WHITE
            },
            ..Default::default()
        };
        egui::SidePanel::new(egui::panel::Side::Left, "menu_panel")
            .frame(frame)
            .min_width(15.)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                if app.menu_minimised {
                    ui.set_width(24.);
                } else {
                    ui.set_width(if max_width >= 300. { 300. } else { max_width });
                }

                let txt_colour = if ui.style().visuals.dark_mode {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::BLACK
                };
                ui.style_mut().visuals.override_text_color = Some(txt_colour);
                ui.style_mut().spacing.button_padding = egui::vec2(8., 5.);

                egui::ScrollArea::vertical()
                    .scroll_source(ScrollSource {
                        scroll_bar: true,
                        drag: false,
                        mouse_wheel: true,
                    })
                    .show(ui, |ui| {
                        ui.add_space(5.);

                        let home_icon = egui::include_image!("../assets/home_with_door.svg");
                        if ui
                            .add(side_menu_button(
                                home_icon,
                                "Home",
                                "Home",
                                app.menu_minimised,
                                app.view == AppView::Main,
                            ))
                            .clicked()
                        {
                            app.view = AppView::Main;
                        };

                        let request_icon = egui::include_image!("../assets/create.svg");
                        if ui
                            .add(side_menu_button(
                                request_icon,
                                "Make a Request",
                                "Make a new Request",
                                app.menu_minimised,
                                app.view == AppView::Request,
                            ))
                            .clicked()
                        {
                            app.view = AppView::Request;
                        };

                        let save_icon = egui::include_image!("../assets/floppy.svg");
                        if ui
                            .add(side_menu_button(
                                save_icon,
                                "Saved Requests",
                                "Saved Requests",
                                app.menu_minimised,
                                app.view == AppView::Saved,
                            ))
                            .clicked()
                        {
                            app.view = AppView::Saved;
                        };

                        let history_icon = egui::include_image!("../assets/undo_history.svg");
                        let history_btn_rect = ui.add(side_menu_button(
                            history_icon,
                            "History",
                            "View Historic Requests",
                            app.menu_minimised,
                            app.view == AppView::History,
                        ));

                        if history_btn_rect.clicked() {
                            app.view = AppView::History;
                        }

                        ui.allocate_ui_with_layout(
                            egui::vec2(
                                ui.available_width(),
                                if ui.available_height() - history_btn_rect.rect.height()
                                    <= history_btn_rect.rect.height()
                                {
                                    ui.available_height() + history_btn_rect.rect.height()
                                } else {
                                    ui.available_height()
                                },
                            ),
                            egui::Layout::bottom_up(egui::Align::Min),
                            |ui| {
                                ui.add_space(5.);
                                if ui
                                    .add(minimiser(
                                        MinimiserDirection::LeftToRight,
                                        app.menu_minimised,
                                    ))
                                    .clicked()
                                {
                                    app.menu_minimised = !app.menu_minimised;
                                }
                            },
                        );
                    });
            })
            .response
    }
}

/// The view handling the user request & associated settings
fn request_panel<'a>(app: &'a mut Reqwestur) -> impl egui::Widget + 'a {
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

                app.check_sendable();

                egui::ScrollArea::vertical()
                    .scroll_source(ScrollSource {
                        scroll_bar: true,
                        drag: false,
                        mouse_wheel: true,
                    })
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                            ui.add_space(1.);

                            ui.add(padded_group(|ui| {
                                ui.label(egui::RichText::new("Request URL").size(14.));
                                if ui
                                    .add(
                                        egui::TextEdit::singleline(&mut app.request.address.uri)
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
                                            message: format!("URL cannot be parsed: {}!", error),
                                        })
                                    } else {
                                        app.request.address.notification = None;
                                    }
                                }

                                if let Some(notification) = &app.request.address.notification {
                                    ui.add(notification.display());
                                }
                            }));

                            ui.add(padded_group(|ui| {
                                ui.label(egui::RichText::new("Request Method").size(14.));

                                egui::ComboBox::new("request_method", "Select the Method")
                                    .selected_text(app.request.method.to_string())
                                    .show_ui(ui, |ui| {
                                        for method in Method::values() {
                                            app.request.content_type = ContentType::EMPTY;
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
                                    let edit_icon = egui::include_image!("../assets/pen.svg");
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
                                }
                            }));

                            ui.add(padded_group(|ui| {
                                ui.label(egui::RichText::new("Request Headers").size(14.));

                                let edit_icon = egui::include_image!("../assets/pen.svg");
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
                            }));

                            let size = ui
                                .add(padded_group(|ui| {
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
                                        let edit_icon = egui::include_image!("../assets/pen.svg");
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
                                    }

                                    if let Some(notification) = &app.certificates.notification {
                                        ui.add(notification.display());
                                    }
                                }))
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
                                        egui::include_image!("../assets/paper_plane.svg");
                                    if ui
                                        .add_enabled(
                                            app.request.sendable,
                                            default_button(
                                                Some(send_icon),
                                                "Send!",
                                                ui.available_width(),
                                                ui.visuals().text_color(),
                                            ),
                                        )
                                        .clicked()
                                    {
                                        let _ = app.send();
                                    }

                                    if let Some(notification) = &app.request.notification {
                                        ui.add(notification.display());
                                    }
                                },
                            );
                        });
                    });
            })
            .response
    }
}

/// The view handling the user's previous requests
fn history_panel<'a>(app: &'a mut Reqwestur) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    if app.history.len() == 0 {
                        ui.label("You haven't made any requests yet!");
                    } else {
                        let bin_icon = egui::include_image!("../assets/trash.svg");
                        if ui
                            .add(default_button(
                                Some(bin_icon),
                                "Clear History",
                                ui.available_width(),
                                ui.visuals().text_color(),
                            ))
                            .clicked()
                        {
                            app.history = Vec::new();
                        }

                        ui.add_space(2.);
                        ui.separator();
                        ui.add_space(2.);

                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .max_height(ui.available_height())
                            .max_width(ui.available_width())
                            .show_rows(ui, 18., app.history.len(), |ui, row_range| {
                                for row in row_range.rev() {
                                    if let Some(row_data) = app.history.get(row) {
                                        egui::Frame::new()
                                            .stroke(egui::Stroke::new(
                                                1.,
                                                ui.visuals().noninteractive().bg_stroke.color,
                                            ))
                                            .inner_margin(egui::Vec2::splat(5.))
                                            .corner_radius(5.)
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    let open_icon = egui::include_image!(
                                                        "../assets/folder_open.svg"
                                                    );
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
                                                        app.view = AppView::Request;
                                                    }

                                                    ui.vertical(|ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.add(pip(
                                                                &row_data.method.to_string(),
                                                                row_data.method.to_colour(),
                                                            ));
                                                            ui.add(pip(
                                                                &row_data
                                                                    .response
                                                                    .status_code
                                                                    .to_string(),
                                                                row_data
                                                                    .response
                                                                    .status_code
                                                                    .to_colour(),
                                                            ));

                                                            ui.with_layout(
                                                                egui::Layout::right_to_left(
                                                                    egui::Align::RIGHT,
                                                                ),
                                                                |ui| {
                                                                    ui.add(pip(
                                                                        &row_data.timestamp.clone(),
                                                                        egui::Color32::ORANGE,
                                                                    ));
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

/// The view handling the initial user interaction
fn home_panel<'a>(app: &'a mut Reqwestur) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical()
                    .max_width(ui.available_width())
                    .max_height(ui.available_height())
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() / 4.);
                            let laptop_icon = egui::include_image!("../assets/laptop.svg");
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
                                        default_button(
                                            egui::include_image!("../assets/create.svg"),
                                            "Create a new request",
                                            button_width,
                                            ui.visuals().text_color(),
                                        )
                                        .shortcut_text(
                                            ui.ctx().format_shortcut(&egui::KeyboardShortcut::new(
                                                egui::Modifiers::CTRL,
                                                egui::Key::N,
                                            )),
                                        ),
                                    )
                                    .clicked()
                                {
                                    app.view = AppView::Request;
                                }

                                if ui
                                    .add(
                                        default_button(
                                            egui::include_image!("../assets/undo_history.svg"),
                                            "View your recent requests",
                                            button_width,
                                            ui.visuals().text_color(),
                                        )
                                        .shortcut_text(
                                            ui.ctx().format_shortcut(&egui::KeyboardShortcut::new(
                                                egui::Modifiers::CTRL,
                                                egui::Key::H,
                                            )),
                                        ),
                                    )
                                    .clicked()
                                {
                                    app.view = AppView::History;
                                }

                                if ui
                                    .add(
                                        default_button(
                                            egui::include_image!("../assets/floppy.svg"),
                                            "Open your saved requests",
                                            button_width,
                                            ui.visuals().text_color(),
                                        )
                                        .shortcut_text(
                                            ui.ctx().format_shortcut(&egui::KeyboardShortcut::new(
                                                egui::Modifiers::CTRL,
                                                egui::Key::O,
                                            )),
                                        ),
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

/// The panel showing the request's response
fn viewer_panel<'a>(app: &'a mut Reqwestur) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| {
        let frame = egui::frame::Frame {
            outer_margin: 10.0.into(),
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(frame)
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical()
                    .scroll_source(ScrollSource {
                        scroll_bar: true,
                        drag: false,
                        mouse_wheel: true,
                    })
                    .show(ui, |ui| {
                        let enabled_editor = !app.request.response.body.is_empty();
                        let response = app.request.response.clone();

                        if !enabled_editor {
                            ui.label("You haven't made a request yet!");
                        } else {
                            ui.add(padded_group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Response Status:");
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::Max)
                                            .with_main_justify(true)
                                            .with_main_align(egui::Align::LEFT),
                                        |ui| {
                                            ui.label(
                                                egui::RichText::new(
                                                    response.status_code.to_string(),
                                                )
                                                .color(response.status_code.to_colour()),
                                            );
                                        },
                                    );
                                });
                            }));

                            ui.add(padded_group(|ui| {
                                egui::CollapsingHeader::new("Response Headers").show_unindented(
                                    ui,
                                    |ui| {
                                        egui::ScrollArea::vertical()
                                            .scroll_source(ScrollSource {
                                                scroll_bar: true,
                                                drag: false,
                                                mouse_wheel: true,
                                            })
                                            .show(ui, |ui| {
                                                for (name, value) in
                                                    &mut app.request.response.headers
                                                {
                                                    ui.with_layout(
                                                        egui::Layout::right_to_left(
                                                            egui::Align::Min,
                                                        ),
                                                        |ui| {
                                                            ui.add(
                                                                egui::TextEdit::singleline(value)
                                                                    .desired_width(
                                                                        ui.available_width() / 2.,
                                                                    )
                                                                    .margin(5.),
                                                            );
                                                            ui.add(
                                                                egui::TextEdit::singleline(name)
                                                                    .desired_width(
                                                                        ui.available_width(),
                                                                    )
                                                                    .margin(5.),
                                                            );
                                                        },
                                                    );
                                                }
                                            });
                                    },
                                );
                            }));

                            egui::Frame::new()
                                .stroke(egui::Stroke::new(
                                    1.,
                                    ui.style().noninteractive().bg_stroke.color,
                                ))
                                .corner_radius(5.)
                                .show(ui, |ui| {
                                    let theme =
                                        egui_extras::syntax_highlighting::CodeTheme::from_memory(
                                            ui.ctx(),
                                            ui.style(),
                                        );
                                    let mut layouter =
                                        |ui: &egui::Ui, buf: &dyn egui::TextBuffer, _| {
                                            let mut layout_job =
                                                egui_extras::syntax_highlighting::highlight(
                                                    ui.ctx(),
                                                    ui.style(),
                                                    &theme.clone(),
                                                    buf.as_str(),
                                                    "json",
                                                );

                                            // Don't allow the wrap to reach the end of the TextEdit
                                            layout_job.wrap.max_width = ui.available_width() - 20.;

                                            ui.fonts(|f| f.layout_job(layout_job))
                                        };

                                    ui.add(
                                        egui::TextEdit::multiline(&mut app.request.response.body)
                                            .code_editor()
                                            .layouter(&mut layouter)
                                            .desired_width(ui.available_width()),
                                    );
                                });
                        }
                    });
            })
            .response
    }
}

/// The header editor window
fn header_editor(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("header_editor"),
        egui::ViewportBuilder::default()
            .with_title("Header Editor")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                let add_icon = egui::include_image!("../assets/plus.svg");
                if ui
                    .add(default_button(
                        Some(add_icon),
                        "New Header",
                        ui.available_width(),
                        ui.visuals().text_color(),
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
                    .max_height(ui.available_height() - 34.)
                    .max_width(ui.available_width())
                    .show_rows(ui, 18., app.request.headers.len(), |ui, row_range| {
                        for row in row_range {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    if let Some((name, value)) = app.request.headers.get_mut(row) {
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

/// The payload editor window
fn payload_editor(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("payload_editor"),
        egui::ViewportBuilder::default()
            .with_title("Payload Editor")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                egui::ComboBox::new("body_type_dropdown", "Payload Type")
                    .selected_text(&app.request.content_type.to_string())
                    .show_ui(ui, |ui| {
                        for body_type in ContentType::values() {
                            ui.selectable_value(
                                &mut app.request.content_type,
                                body_type.clone(),
                                body_type.to_string(),
                            );
                        }
                    });

                ui.add_space(2.);
                ui.separator();
                ui.add_space(2.);

                egui::ScrollArea::vertical()
                    .max_height(ui.available_height())
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                            match app.request.content_type {
                                ContentType::EMPTY => {
                                    app.request.body = None;
                                }
                                ContentType::XWWWFORMURLENCODED => {
                                    let add_icon = egui::include_image!("../assets/plus.svg");
                                    if ui
                                        .add(default_button(
                                            Some(add_icon),
                                            "New Field",
                                            ui.available_width(),
                                            ui.visuals().text_color(),
                                        ))
                                        .clicked()
                                    {
                                        app.request
                                            .form_data
                                            .push((String::default(), String::default()));
                                    }

                                    ui.add_space(2.);
                                    ui.separator();
                                    ui.add_space(2.);

                                    egui::ScrollArea::vertical()
                                        .auto_shrink(false)
                                        .max_width(ui.available_width())
                                        .max_height(ui.available_height() - 34.)
                                        .show_rows(
                                            ui,
                                            18.,
                                            app.request.form_data.len(),
                                            |ui, row_range| {
                                                for row in row_range {
                                                    ui.group(|ui| {
                                                        ui.horizontal(|ui| {
                                                            if let Some((name, value)) =
                                                                app.request.form_data.get_mut(row)
                                                            {
                                                                let name_editor =
                                                                    egui::TextEdit::singleline(
                                                                        name,
                                                                    )
                                                                    .hint_text("Field Name")
                                                                    .margin(5.)
                                                                    .vertical_align(
                                                                        egui::Align::Center,
                                                                    )
                                                                    .desired_width(
                                                                        ui.available_width() / 2.
                                                                            - 50.,
                                                                    );

                                                                let value_editor =
                                                                    egui::TextEdit::singleline(
                                                                        value,
                                                                    )
                                                                    .hint_text("Field Value")
                                                                    .margin(5.)
                                                                    .vertical_align(
                                                                        egui::Align::Center,
                                                                    )
                                                                    .desired_width(
                                                                        ui.available_width(),
                                                                    );

                                                                ui.add(name_editor);
                                                                ui.add(value_editor);
                                                            }
                                                        });
                                                    });
                                                }
                                            },
                                        );
                                }
                                ContentType::MULTIPART => todo!(),
                                ContentType::JSON | ContentType::TEXT => {
                                    let theme =
                                        egui_extras::syntax_highlighting::CodeTheme::from_memory(
                                            ui.ctx(),
                                            ui.style(),
                                        );
                                    let mut layouter =
                                        |ui: &egui::Ui, buf: &dyn egui::TextBuffer, _| {
                                            let layout_job =
                                                egui_extras::syntax_highlighting::highlight(
                                                    ui.ctx(),
                                                    ui.style(),
                                                    &theme.clone(),
                                                    buf.as_str(),
                                                    match app.request.content_type {
                                                        ContentType::JSON => "json",
                                                        _ => "text",
                                                    },
                                                );
                                            ui.fonts(|f| f.layout_job(layout_job))
                                        };

                                    match &mut app.request.body {
                                        Some(body) => {
                                            ui.add_sized(
                                                egui::vec2(
                                                    ui.available_width(),
                                                    ui.available_height() - 34.,
                                                ),
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
                                    app.payload_editor_open = false;
                                }
                            });
                        });
                    });
            });

            if context.input(|i| i.viewport().close_requested()) {
                app.payload_editor_open = false;
            }
        },
    );
}

/// The certificate editor window
fn certificate_editor(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("certificate_editor"),
        egui::ViewportBuilder::default()
            .with_title("Certificate Editor")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    let save_icon = egui::include_image!("../assets/floppy.svg");
                    if ui
                        .add_enabled(
                            app.certificates.status == CertificatesStatus::OK,
                            default_button(
                                Some(save_icon.clone()),
                                "Confirm & Close!",
                                ui.available_width(),
                                ui.visuals().text_color(),
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
                                ui.available_width(),
                                ui.visuals().text_color(),
                            ),
                        )
                        .clicked()
                    {
                        match app.certificates.import() {
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

                    if let Some(notification) = &app.certificates.notification {
                        ui.add(notification.display());
                    }

                    if app.certificates.status == CertificatesStatus::UNCONFIRMED {
                        ui.label("No certificates have been loaded.");
                    }

                    ui.vertical(|ui| {
                        ui.add(padded_group(|ui| {
                            let file_name = match app.certificates.file_path.file_name() {
                                Some(file_name) => {
                                    file_name.to_str().unwrap_or("Error reading file name.")
                                }
                                None => "No file selected.",
                            };
                            ui.label(format!("Selected File: {}", file_name));

                            let upload_icon = egui::include_image!("../assets/upload.svg");
                            if ui
                                .add(default_button(
                                    Some(upload_icon),
                                    "Select a PFX.",
                                    ui.available_width(),
                                    ui.visuals().text_color(),
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
                                let bin_icon = egui::include_image!("../assets/trash.svg");
                                if ui
                                    .add(default_button(
                                        Some(bin_icon),
                                        "Remove PFX.",
                                        ui.available_width(),
                                        ui.visuals().text_color(),
                                    ))
                                    .clicked()
                                {
                                    app.certificates = Certificates {
                                        required: app.certificates.required,
                                        ..Default::default()
                                    };
                                }
                            }
                        }));

                        if app.certificates.file_path.exists() {
                            ui.add(padded_group(|ui| {
                                ui.label("Certificate Passphrase:");
                                ui.add_sized(
                                    egui::vec2(ui.available_width(), 20.),
                                    egui::TextEdit::singleline(&mut app.certificates.passphrase)
                                        .margin(5.)
                                        .password(true),
                                );
                            }));
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

/// The help modal
fn help_modal(app: &mut Reqwestur, ui: &mut egui::Ui) {
    egui::Modal::new("HelpModal".into()).show(ui.ctx(), |ui| {
        ui.set_min_width(200.);

        ui.heading("Help");

        ui.label("This is some text");

        if ui.button("Close").clicked() {
            app.help_modal_open = false;
        }
    });
}

/// The information modal
fn about_modal(app: &mut Reqwestur, ui: &mut egui::Ui) {
    egui::Modal::new("AboutModal".into()).show(ui.ctx(), |ui| {
        ui.set_min_width(200.);

        ui.heading("About");

        ui.label("This is some text");

        if ui.button("Close").clicked() {
            app.about_modal_open = false;
        }
    });
}
