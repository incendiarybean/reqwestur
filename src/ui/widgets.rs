use eframe::egui::{self};
use std::f32::consts::PI;

use crate::utils::reqwestur::Notification;

pub fn display_notification(notification: &Option<Notification>) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        ui.vertical(|ui| {
            if let Some(notification) = notification {
                let Notification { kind, message } = notification;
                ui.label(egui::RichText::new(message).color(kind.to_colour()));
            }
        })
        .response
    }
}

pub fn default_button<'a>(
    image: Option<egui::ImageSource<'a>>,
    text: &str,
    colour: egui::Color32,
    width: f32,
) -> egui::Button<'a> {
    let txt = egui::RichText::new(text).size(14.);
    let btn = if let Some(image) = image {
        egui::Button::image_and_text(
            egui::Image::new(image)
                .tint(colour)
                .fit_to_exact_size(egui::vec2(16., 16.)),
            txt,
        )
    } else {
        egui::Button::new(txt)
    }
    .min_size(egui::vec2(width, 10.));

    btn
}

pub fn draw_vertical_text(ui: &mut egui::Ui, text: &str) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter_at(rect);
    let txt_colour = ui.ctx().theme().default_visuals().text_color();

    let label = eframe::epaint::TextShape::new(
        egui::pos2(rect.center_top().x + 11., rect.center_top().y + 35.),
        ui.fonts(|f| {
            f.layout_no_wrap(
                text.to_string(),
                egui::FontId {
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

pub fn padded_group<F: FnOnce(&mut egui::Ui)>(ui: &mut egui::Ui, content: F) {
    ui.group(|ui| {
        ui.add_space(2.);

        ui.allocate_space(egui::vec2(ui.available_width(), 0.));

        content(ui);

        ui.add_space(2.);
    });
}

pub enum MinimiserDirection {
    LeftToRight,
    RightToLeft,
}

pub fn minimiser(direction: MinimiserDirection, current_value: bool) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        let chevron_left = egui::include_image!("../assets/chevron_left_double.svg");
        let chevron_right = egui::include_image!("../assets/chevron_right_double.svg");

        let (minimise, expand) = match direction {
            MinimiserDirection::LeftToRight => (chevron_right, chevron_left),
            MinimiserDirection::RightToLeft => (chevron_left, chevron_right),
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

pub fn toggle_switch(on: &mut bool) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }

        response
            .widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, false, ""));

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

pub fn side_menu_button<'a>(
    icon: egui::ImageSource<'a>,
    text: &'static str,
    alt_text: &'static str,
    small: bool,
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
                .tint(ui.visuals().text_color()),
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
                .corner_radius(5.),
            )
        }
    }
}
