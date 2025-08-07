use eframe::egui;

/// A custom trait to describe the conversion of a value to egui's Color32
pub trait ToColour {
    fn to_colour(&self) -> egui::Color32;
}
