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
