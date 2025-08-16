use eframe::egui;

/// Create a tabbed UI bar, to switch between a supplied value - e.g. changing a view
pub fn tabs<C: ToString + std::cmp::PartialEq + Clone>(
    values: Vec<C>,
    current_value: C,
    set_value: &mut C,
) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        ui.scope(|ui| {
            // Custom colours and sizings
            ui.visuals_mut().widgets.hovered.expansion = 0.;
            ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
            ui.visuals_mut().button_frame = false;
            ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
            ui.style_mut().spacing.button_padding = egui::vec2(10., 5.);

            // Create the surrounding tab bar
            ui.add(egui::Separator::default().horizontal().spacing(0.));
            let response = ui
                .horizontal(|ui| {
                    let response = egui::ScrollArea::horizontal()
                        .auto_shrink(false)
                        .max_width(ui.available_width())
                        .show(ui, |ui| {
                            for view in values {
                                if ui
                                    .add(
                                        egui::Button::new(view.to_string())
                                            .selected(current_value == view)
                                            .corner_radius(0.)
                                            .stroke(egui::Stroke::NONE),
                                    )
                                    .clicked()
                                {
                                    *set_value = view;
                                }
                            }
                        });

                    // Return the response from the tab buttons
                    response
                })
                .response;
            ui.add(egui::Separator::default().horizontal().spacing(0.));

            response
        })
        .response
    }
}
