use eframe::egui;

pub fn default_button<'image>(
    image: impl Into<Option<egui::ImageSource<'image>>>,
    text: &str,
    width: f32,
) -> egui::Button<'image> {
    let image = image.into();
    let txt = egui::RichText::new(text).size(14.);
    let btn = if let Some(image) = image {
        egui::Button::image_and_text(
            egui::Image::new(image).fit_to_exact_size(egui::vec2(16., 16.)),
            txt,
        )
    } else {
        egui::Button::new(txt)
    }
    .min_size(egui::vec2(width, 32.))
    .corner_radius(5.);

    btn
}

pub fn side_menu_button<'a>(
    icon: egui::ImageSource<'a>,
    text: &'static str,
    alt_text: &'static str,
    small: bool,
    active: bool,
) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        if small {
            ui.add(
                egui::ImageButton::new(
                    egui::Image::new(icon)
                        .fit_to_exact_size(egui::vec2(16., 16.))
                        .alt_text(alt_text)
                        .corner_radius(5.),
                )
                .tint(ui.visuals().text_color())
                .selected(active),
            )
        } else {
            ui.add(
                egui::Button::image_and_text(
                    egui::Image::new(icon)
                        .alt_text(alt_text)
                        .fit_to_exact_size(egui::vec2(16., 16.))
                        .tint(ui.visuals().text_color()),
                    egui::RichText::new(text).size(16.),
                )
                .min_size(egui::vec2(ui.available_width(), 32.))
                .corner_radius(5.)
                .selected(active),
            )
        }
    }
}

pub fn toggle_switch(on: &mut bool, label: &str) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
        let label = ui
            .scope(|ui| {
                ui.visuals_mut().button_frame = false;
                ui.button(label)
            })
            .inner;

        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if label.clicked() || response.clicked() {
            *on = !*on;
            response.mark_changed();
        }

        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, false, "Enable dark mode?")
        });

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();

            ui.painter().rect(
                rect,
                radius,
                visuals.bg_fill,
                visuals.bg_stroke,
                egui::StrokeKind::Inside,
            );

            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);

            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }
}

pub enum MinimiserDirection {
    LeftToRight,
    _RightToLeft,
}

pub fn minimiser(direction: MinimiserDirection, current_value: bool) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        let chevron_left = egui::include_image!("../../assets/chevron_left_double.svg");
        let chevron_right = egui::include_image!("../../assets/chevron_right_double.svg");

        let (minimise, expand) = match direction {
            MinimiserDirection::LeftToRight => (chevron_right, chevron_left),
            MinimiserDirection::_RightToLeft => (chevron_left, chevron_right),
        };

        let current_icon = if current_value.clone() {
            minimise
        } else {
            expand
        };

        ui.ctx()
            .style_mut(|style| style.spacing.button_padding = egui::vec2(5., 5.));

        if current_value {
            ui.add(
                egui::ImageButton::new(
                    egui::Image::new(current_icon)
                        .fit_to_exact_size(egui::vec2(16., 16.))
                        .corner_radius(5.)
                        .alt_text("Minimise/Maximise the Panel"),
                )
                .tint(ui.visuals().text_color()),
            )
        } else {
            ui.add(
                egui::Button::image_and_text(
                    egui::Image::new(current_icon)
                        .alt_text("Minimise/Maximise the Panel")
                        .tint(ui.visuals().text_color()),
                    egui::RichText::new("Hide Menu").size(16.),
                )
                .min_size(egui::vec2(ui.available_width(), 32.))
                .corner_radius(5.),
            )
        }
    }
}
