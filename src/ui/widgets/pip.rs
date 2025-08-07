use eframe::egui;

pub struct PipColour {
    pub background: egui::Color32,
    pub foreground: egui::Color32,
}

impl From<egui::Color32> for PipColour {
    fn from(background: egui::Color32) -> Self {
        PipColour {
            background,
            foreground: egui::Color32::BLACK,
        }
    }
}

impl From<(egui::Color32, egui::Color32)> for PipColour {
    fn from(colours: (egui::Color32, egui::Color32)) -> Self {
        let (background, foreground) = colours;
        PipColour {
            background,
            foreground,
        }
    }
}

impl From<Option<PipColour>> for PipColour {
    fn from(pip_colour: Option<PipColour>) -> Self {
        if let Some(value) = pip_colour {
            value
        } else {
            PipColour {
                background: egui::Color32::BLACK,
                foreground: egui::Color32::WHITE,
            }
        }
    }
}

pub fn pip(label: &str, colour: impl Into<PipColour>) -> impl egui::Widget {
    move |ui: &mut egui::Ui| {
        let PipColour {
            foreground,
            background,
        } = colour.into();

        egui::Frame::new()
            .fill(background)
            .corner_radius(15.)
            .inner_margin(egui::vec2(8., 1.))
            .stroke(egui::Stroke::new(1., foreground))
            .show(ui, |ui| {
                ui.label(egui::RichText::new(label).color(foreground).size(12.));
            })
            .response
    }
}
