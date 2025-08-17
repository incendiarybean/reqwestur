use eframe::egui::{self, scroll_area::ScrollSource};

use crate::{
    ui::widgets::{
        chip::Chip,
        headers::{self, StringToVec},
        tabs::tabs,
    },
    utils::{
        request::{Request, ResponseView},
        traits::ToColour,
    },
};

/// The panel showing the request's response
pub fn panel(request: &mut Request) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        let frame = egui::frame::Frame {
            outer_margin: 0.0.into(),
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
                        let response = request.response.clone();
                        match request.event {
                            crate::utils::request::RequestEvent::UNSENT => {
                                ui.label("You haven't made a request yet!");
                            }
                            crate::utils::request::RequestEvent::PENDING => {
                                ui.add_sized(
                                    egui::vec2(ui.available_width(), ui.available_height()),
                                    egui::Spinner::new().size(ui.available_width() / 10.),
                                );
                            }
                            crate::utils::request::RequestEvent::SENT => {
                                egui::Frame::new()
                                    .inner_margin(egui::Margin {
                                        left: 5,
                                        right: 5,
                                        top: 5,
                                        bottom: 2,
                                    })
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            Chip::new(
                                                response.status_code.to_string(),
                                                response
                                                    .status_code
                                                    .to_colour(ui.visuals().dark_mode),
                                            )
                                            .show(ui);
                                            Chip::new(response.body.len().to_string() + "B", None)
                                                .show(ui);
                                        });
                                    });
                                ui.add(tabs(
                                    ResponseView::values(),
                                    response.view,
                                    &mut request.response.view,
                                ));
                                match response.view {
                                    ResponseView::RESPONSE => {
                                        ui.add(self::body_editor(&mut request.response.body));
                                    }
                                    ResponseView::HEADERS => {
                                        ui.add(headers::viewer(response.headers.convert().clone()));
                                    }
                                    ResponseView::COOKIES => {
                                        ui.add(headers::viewer(response.cookies.convert().clone()));
                                    }
                                }
                            }
                        }
                    });
            })
            .response
    }
}

/// The editor that displays the body content
fn body_editor(body: &mut String) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        egui::Frame::new()
            .outer_margin(5.)
            .stroke(egui::Stroke::new(
                1.,
                ui.style().noninteractive().bg_stroke.color,
            ))
            .corner_radius(5.)
            .show(ui, |ui| {
                let theme =
                    egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, _| {
                    let mut layout_job = egui_extras::syntax_highlighting::highlight(
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
                    egui::TextEdit::multiline(body)
                        .code_editor()
                        .layouter(&mut layouter)
                        .desired_width(ui.available_width())
                        .min_size(egui::vec2(ui.available_width(), ui.available_height())),
                );
            })
            .response
    }
}
