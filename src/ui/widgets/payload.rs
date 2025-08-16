use eframe::egui::{self};

use crate::{
    ui::widgets::buttons::default_button,
    utils::{
        request::{ContentType, Request},
        reqwestur::Reqwestur,
    },
};

/// The payload editor window
pub fn editor(app: &mut Reqwestur, request: &mut Request, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("payload_editor"),
        egui::ViewportBuilder::default()
            .with_title("Payload Editor")
            .with_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                egui::ComboBox::new("body_type_dropdown", "Payload Type")
                    .selected_text(&request.content_type.to_string())
                    .show_ui(ui, |ui| {
                        for body_type in ContentType::values() {
                            ui.selectable_value(
                                &mut request.content_type,
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
                            match request.content_type {
                                ContentType::EMPTY => {
                                    request.body = None;
                                }
                                ContentType::XWWWFORMURLENCODED => {
                                    let add_icon = egui::include_image!("../../assets/plus.svg");
                                    if ui
                                        .add(default_button(
                                            Some(add_icon),
                                            "New Field",
                                            ui.available_width(),
                                            ui.visuals().text_color(),
                                        ))
                                        .clicked()
                                    {
                                        request.params.push((String::default(), String::default()));
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
                                            request.params.len(),
                                            |ui, row_range| {
                                                for row in row_range {
                                                    ui.group(|ui| {
                                                        ui.horizontal(|ui| {
                                                            if let Some((name, value)) =
                                                                request.params.get_mut(row)
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
                                    let content_type = request.content_type.clone();
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
                                                    match content_type {
                                                        ContentType::JSON => "json",
                                                        _ => "text",
                                                    },
                                                );
                                            ui.fonts(|f| f.layout_job(layout_job))
                                        };

                                    match &mut request.body {
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
                                        _ => request.body = Some(String::default()),
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
