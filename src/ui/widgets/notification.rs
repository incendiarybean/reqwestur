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
struct NotificationInner {
    kind: NotificationKind,
    message: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Notification {
    inner: Option<NotificationInner>,
}

impl Notification {
    pub fn new(message: impl Into<String>, kind: NotificationKind) -> Self {
        Self {
            inner: Some(NotificationInner {
                kind,
                message: message.into(),
            }),
        }
    }

    /// Clear the current Notification
    pub fn clear(&mut self) {
        self.inner = None;
    }

    /// Display a rounded notification
    pub fn display(&mut self, ui: &mut egui::Ui) {
        if self.inner.is_some() {
            ui.add(self.display_widget());
        }
    }

    /// Private display function
    fn display_widget(&self) -> impl egui::Widget {
        move |ui: &mut egui::Ui| {
            // Grab the current notification if it exists
            let notification = self.inner.clone().unwrap_or(NotificationInner::default());

            let icon = match notification.kind {
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
                .stroke(egui::Stroke::new(
                    1.,
                    notification.kind.to_colour(is_dark_mode),
                ))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Image::new(icon)
                                .fit_to_exact_size(egui::Vec2::splat(16.))
                                .tint(notification.kind.to_colour(is_dark_mode)),
                        );
                        ui.with_layout(
                            egui::Layout::left_to_right(egui::Align::Max)
                                .with_main_justify(true)
                                .with_main_align(egui::Align::Min),
                            |ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(notification.message.clone())
                                            .color(notification.kind.to_colour(is_dark_mode)),
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

    /// Display a banner notification
    pub fn banner(&mut self, ui: &mut egui::Ui) {
        if self.inner.is_some() {
            ui.add(self.banner_widget());
        }
    }

    fn banner_widget(&mut self) -> impl egui::Widget {
        move |ui: &mut egui::Ui| {
            // Grab the current notification if it exists
            let notification = self.inner.clone().unwrap_or(NotificationInner::default());

            let icon = match notification.kind {
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

            let frame = egui::Frame {
                inner_margin: 10.into(),
                fill: notification.kind.to_colour(is_dark_mode),
                shadow: egui::Shadow {
                    color: egui::Color32::BLACK,
                    spread: 1.into(),
                    offset: [0, 2],
                    blur: 30.into(),
                    ..Default::default()
                },
                ..Default::default()
            };

            egui::TopBottomPanel::top("notification_banner")
                .frame(frame)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        let text_colour = if ui.visuals().dark_mode {
                            egui::Color32::BLACK
                        } else {
                            egui::Color32::WHITE
                        };

                        ui.add(
                            egui::Image::new(icon)
                                .fit_to_exact_size(egui::Vec2::splat(16.))
                                .tint(text_colour),
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(notification.message.clone())
                                    .color(text_colour),
                            )
                            .truncate(),
                        );

                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::RIGHT)
                                .with_main_align(egui::Align::RIGHT),
                            |ui| {
                                let close_icon =
                                    egui::include_image!("../../assets/cross_circle.svg");
                                if ui
                                    .add(egui::ImageButton::new(
                                        egui::Image::new(close_icon)
                                            .tint(ui.visuals().text_color())
                                            .fit_to_exact_size(egui::Vec2::splat(12.))
                                            .corner_radius(5.)
                                            .alt_text("Close Notification"),
                                    ))
                                    .clicked()
                                {
                                    self.inner = None;
                                }
                            },
                        );
                    });
                })
                .response
        }
    }
}
