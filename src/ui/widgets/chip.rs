use eframe::egui::{self};
use std::f32;

/// A struct containing the anatomy of the PIP
pub struct Chip {
    width: Option<f32>,
    height: Option<f32>,
    padding: Option<egui::Vec2>,
    text: String,
    foreground: egui::Color32,
    background: egui::Color32,
}

/// A custom trait to describe the conversion of a value to a tuple of egui's Color32
pub trait ToChipColours {
    /// Convert from the inherited value to a tuple of egui::Color32
    fn from(&self) -> (egui::Color32, egui::Color32);
}

/// Implement the ToColours trait, allowing multiple choices in creating background/foreground colours
impl ToChipColours for egui::Color32 {
    fn from(&self) -> (egui::Color32, egui::Color32) {
        (*self, egui::Color32::BLACK)
    }
}

/// Implement the ToColours trait, allowing multiple choices in creating background/foreground colours
impl ToChipColours for (egui::Color32, egui::Color32) {
    fn from(&self) -> (egui::Color32, egui::Color32) {
        self.to_owned()
    }
}

/// Implement the ToColours trait, allowing multiple choices in creating background/foreground colours
impl ToChipColours for Option<(egui::Color32, egui::Color32)> {
    fn from(&self) -> (egui::Color32, egui::Color32) {
        self.unwrap_or((egui::Color32::BLACK, egui::Color32::WHITE))
    }
}

/// Implement the PIP functionality
impl Chip {
    /// Create a new PIP from a label and colour
    /// The PIP by default inherits the width and height of the label provided, with slight padding.
    pub fn new<C: ToChipColours>(label: impl Into<String>, colour: C) -> Self {
        let (background, foreground) = colour.from();

        Self {
            width: None,
            height: None,
            padding: None,
            text: label.into(),
            foreground,
            background,
        }
    }

    /// Get the PIP's width
    #[allow(dead_code, reason = "May be used in the future")]
    pub fn width(&self) -> Option<f32> {
        self.width
    }

    /// Set the PIP's width
    #[allow(dead_code, reason = "May be used in the future")]
    pub fn set_width(&mut self, width: f32) -> &mut Self {
        self.width = Some(width);

        self
    }

    /// Get the PIP's height
    #[allow(dead_code, reason = "May be used in the future")]
    pub fn height(&self) -> Option<f32> {
        self.height
    }

    /// Set the PIP's height
    #[allow(dead_code, reason = "May be used in the future")]
    pub fn set_height(&mut self, height: f32) -> &mut Self {
        self.height = Some(height);

        self
    }

    /// Get the PIP's padding
    #[allow(dead_code, reason = "May be used in the future")]
    pub fn padding(&self) -> Option<egui::Vec2> {
        self.padding
    }

    /// Set the PIP's height
    #[allow(dead_code, reason = "May be used in the future")]
    pub fn set_padding(&mut self, padding: egui::Vec2) -> &mut Self {
        self.padding = Some(padding);

        self
    }

    /// Show the PIP
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.add(self.create());
    }

    /// Create the PIP widget
    pub fn create(&self) -> impl egui::Widget {
        move |ui: &mut egui::Ui| {
            // Check if values already exist in memory
            let id = egui::Id::new("chip_text_sizes".to_owned() + &self.text);
            let (mut text_width, mut text_height) =
                ui.data(|data| data.get_temp(id).unwrap_or((f32::NAN, f32::NAN)));

            // Cannot pass UI to the data reader, check if the values exist or create them here
            if text_width.is_nan() || text_height.is_nan() {
                text_width = self
                    .text
                    .chars()
                    .map(|char| ui.fonts(|f| f.glyph_width(&egui::FontId::default(), char).ceil()))
                    .sum();
                text_height = ui.fonts(|f| f.row_height(&egui::FontId::default()));
            }

            // Size calculations
            let padding = self.padding.unwrap_or(egui::vec2(16., 2.));
            let desired_width = self.width.unwrap_or(padding.x + text_width);
            let desired_height = self.height.unwrap_or(padding.y + text_height);

            // Allocate the rect
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(desired_width, desired_height),
                egui::Sense::CLICK,
            );

            // Create painter
            let painter: &egui::Painter = ui.painter();

            // Draw rounded rect
            painter.rect(
                rect,
                15.,
                self.background,
                egui::Stroke::new(1., self.foreground),
                egui::StrokeKind::Inside,
            );

            // Draw text in center of PIP
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &self.text,
                egui::FontId::default(),
                self.foreground,
            );

            // Additional widget context
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &self.text)
            });

            // Memoize the width values so we don't calculate on ever render
            ui.data_mut(|data| data.insert_temp(id, (text_width, text_height)));

            response
        }
    }
}
