use std::{f32::consts::PI, process::exit};

use eframe::{
    egui::{self, FontId, Pos2, Vec2, include_image},
    epaint::TextShape,
};

use crate::Reqwestur;

pub fn window(reqwestur: &mut Reqwestur, ui: &mut egui::Ui) {
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
                    ui.button("History");
                    ui.button("Requests");
                });
                ui.menu_button("Import", |ui| {
                    ui.button("History");
                    ui.button("Requests");
                });
                ui.button("Save Request");
                if ui.button("Exit").clicked() {
                    exit(1);
                };
            });
            egui::menu::menu_button(ui, "Help", |ui| {
                ui.button("Guidance");
                ui.button("About");
            });
        });

        ui.add_space(2.);
    });

    egui::SidePanel::new(egui::panel::Side::Left, "control_panel")
        .min_width(15.)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            if !reqwestur.control_panel_visible {
                // Draw the 90 degree label
                draw_angled_text(ui, "REQUEST OPTIONS");

                ui.set_width(23.);
            } else {
                ui.set_width(default_expanded_size - 58.);
            }

            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let left_chevron_icon = include_image!("../assets/double_left_chevron.svg");
                    let right_chevron_icon = include_image!("../assets/double_right_chevron.svg");

                    let current_icon = egui::ImageButton::new(
                        egui::Image::new(if reqwestur.control_panel_visible {
                            left_chevron_icon
                        } else {
                            right_chevron_icon
                        })
                        .fit_to_exact_size(Vec2 { x: 16., y: 16. })
                        .corner_radius(5.)
                        .alt_text("Show the Request Panel"),
                    )
                    .tint(ui.ctx().theme().default_visuals().text_color());

                    if ui.add(current_icon).clicked() {
                        reqwestur.control_panel_visible = !reqwestur.control_panel_visible;
                    }
                });
            });
        });

    egui::SidePanel::new(egui::panel::Side::Right, "history_panel")
        .min_width(15.)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            if !reqwestur.history_panel_visible {
                // Draw the 90 degree label
                draw_angled_text(ui, "HISTORY");

                ui.set_width(23.);
            } else {
                ui.set_width(default_expanded_size);
            }

            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Max), |ui| {
                    let left_chevron_icon = include_image!("../assets/double_left_chevron.svg");
                    let right_chevron_icon = include_image!("../assets/double_right_chevron.svg");

                    let current_icon = egui::ImageButton::new(
                        egui::Image::new(if reqwestur.history_panel_visible {
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
                        reqwestur.history_panel_visible = !reqwestur.history_panel_visible;
                    }
                });

                if reqwestur.history_panel_visible {
                    ui.heading("Previous Requests");
                }
            });

            if reqwestur.history_panel_visible {
                let mut history_copy = reqwestur.request_history.clone();
                let mut num_rows = history_copy.len();

                if history_copy.len() == 0 {
                    history_copy.push((String::from("200"), String::from("https://test.com")));
                    num_rows += 1;
                }

                ui.vertical(|ui| {
                    ui.add_space(5.);
                    ui.group(|ui| {
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
                                                if let Some((status_code, uri)) =
                                                    history_copy.get(row)
                                                {
                                                    ui.label(status_code);
                                                    ui.label("GET");
                                                    ui.label(uri);
                                                }
                                            });
                                        });
                                    }
                                });
                        });
                    });
                });
            }
        });

    egui::CentralPanel::default().show(ui.ctx(), |ui| {
        ui.group(|ui| {
            ui.heading("CENTER");
            ui.allocate_space(Vec2 {
                x: ui.available_width(),
                y: ui.available_height(),
            });
        });
    });
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
