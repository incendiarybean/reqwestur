use eframe::egui::{self, scroll_area::ScrollSource};

use crate::{
    ui::widgets::buttons::{MinimiserDirection, minimiser, side_menu_button},
    utils::reqwestur::{AppView, Reqwestur},
};

pub fn panel<'a>(app: &'a mut Reqwestur, max_width: f32) -> impl egui::Widget + 'a {
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

                        let home_icon = egui::include_image!("../../assets/home_with_door.svg");
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

                        let request_icon = egui::include_image!("../../assets/create.svg");
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

                        let save_icon = egui::include_image!("../../assets/floppy.svg");
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

                        let history_icon = egui::include_image!("../../assets/undo_history.svg");
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
