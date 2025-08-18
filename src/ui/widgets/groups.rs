use eframe::egui;

use crate::utils::breakpoints::{Breakpoint, BreakpointSize};

/// A group that centers itself, if the window is too small it will fill the space instead
pub fn centered_group<C: FnOnce(&mut egui::Ui)>(add_contents: C) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("centered_widget_scroll_area")
                    .max_width(ui.available_width())
                    .max_height(ui.available_height())
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                let breakpoints =
                                    Breakpoint::from((ui.available_width(), ui.available_height()));

                                let (full_size, width, height_padding) = match breakpoints.size {
                                    BreakpointSize::SMALL => (true, ui.available_width(), 0.),
                                    BreakpointSize::MEDIUM => (
                                        false,
                                        ui.available_width() / 1.5,
                                        ui.available_height() / 4.,
                                    ),
                                    BreakpointSize::LARGE => (
                                        false,
                                        ui.available_width() / 2.,
                                        ui.available_height() / 4.,
                                    ),
                                    BreakpointSize::EXTREME => (
                                        false,
                                        ui.available_width() / 3.,
                                        ui.available_height() / 4.,
                                    ),
                                };

                                let child_ui_builder = egui::UiBuilder::new();
                                let mut child_ui = ui.new_child(child_ui_builder);
                                let mut child_ui_response =
                                    child_ui.vertical_centered_justified(|ui| {
                                        ui.set_width(width);
                                        ui.visuals_mut().button_frame = false;
                                        ui.add_space(height_padding);

                                        egui::Frame::new()
                                            .inner_margin(egui::vec2(10., 25.))
                                            .stroke(egui::Stroke::new(
                                                1.,
                                                ui.visuals().noninteractive().bg_stroke.color,
                                            ))
                                            .corner_radius(10.)
                                            .show(ui, |ui| {
                                                add_contents(ui);

                                                if full_size {
                                                    ui.allocate_space(ui.available_size());
                                                };
                                            });
                                    });

                                child_ui_response.response.rect.max.x = ui.available_width();
                                child_ui_response.response.rect.min.x = 0.0;
                                ui.advance_cursor_after_rect(child_ui_response.response.rect);
                            },
                        );
                    });
            })
            .response
    }
}
