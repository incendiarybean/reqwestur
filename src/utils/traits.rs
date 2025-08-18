use eframe::egui;

/// A custom trait to describe the conversion of a value to egui's Color32
pub trait ToColour {
    /// Convert from the inherited value to the value of egui::Color32
    fn to_colour(&self, dark_mode: bool) -> egui::Color32;
}

/// Implement a ToString function for foreign types e.g. tuples
pub trait ToStringForeign {
    /// Convert from the inherited value to the value of String
    fn to_string(&self) -> String;
}
