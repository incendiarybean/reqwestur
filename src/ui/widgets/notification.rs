use eframe::egui;

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum NotificationKind {
    #[default]
    INFO,
    ERROR,
    WARN,
}

impl NotificationKind {
    pub fn to_colour(&self, dark_mode: impl Into<Option<bool>>) -> egui::Color32 {
        let is_dark_mode = dark_mode.into().unwrap_or(false);
        match self {
            NotificationKind::INFO => {
                if is_dark_mode {
                    egui::Color32::LIGHT_GREEN
                } else {
                    egui::Color32::DARK_GREEN
                }
            }
            NotificationKind::ERROR => egui::Color32::RED,
            NotificationKind::WARN => egui::Color32::ORANGE,
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Notification {
    pub kind: NotificationKind,
    pub message: String,
}

impl Notification {
    pub fn display(&self) -> impl egui::Widget {
        move |ui: &mut egui::Ui| {
            let icon = match self.kind {
                NotificationKind::INFO => {
                    egui::include_image!("../../assets/info_circle.svg")
                }
                NotificationKind::ERROR => {
                    egui::include_image!("../../assets/warning_circle.svg")
                }
                NotificationKind::WARN => {
                    egui::include_image!("../../assets/warning_triangle.svg")
                }
            };

            let is_dark_mode = ui.visuals().dark_mode;

            egui::Frame::new()
                .inner_margin(egui::Margin {
                    left: 5,
                    right: 7,
                    top: 3,
                    bottom: 3,
                })
                .corner_radius(5.)
                .stroke(egui::Stroke::new(1., self.kind.to_colour(is_dark_mode)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Image::new(icon)
                                .fit_to_exact_size(egui::Vec2::splat(16.))
                                .tint(self.kind.to_colour(is_dark_mode)),
                        );
                        ui.with_layout(
                            egui::Layout::left_to_right(egui::Align::Max)
                                .with_main_justify(true)
                                .with_main_align(egui::Align::Min),
                            |ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(self.message.clone())
                                            .color(self.kind.to_colour(is_dark_mode)),
                                    )
                                    .truncate(),
                                )
                            },
                        );
                    });
                })
                .response
        }
    }
}
