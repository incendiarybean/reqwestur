use eframe::egui;

/// A custom trait to describe the conversion of a value to egui's Color32
pub trait ToColour {
    /// Convert from the inherited value of egui::Color32
    fn to_colour(&self) -> egui::Color32;
}

/// A custom trait to describe the conversion of a value to a tuple of egui's Color32
pub trait ToColours {
    /// Convert from the inherited value to a tuple of egui::Color32
    fn from(&self) -> (egui::Color32, egui::Color32);
}
