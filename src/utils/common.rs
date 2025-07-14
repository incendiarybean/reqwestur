use eframe::egui;

pub fn status_colour(status_code: &u16) -> egui::Color32 {
    match status_code {
        200..=399 => egui::Color32::GREEN,
        400..=499 => egui::Color32::ORANGE,
        _ => egui::Color32::RED,
    }
}

pub fn load_certificates(
    file_path: &std::path::PathBuf,
    passphrase: &String,
) -> Result<reqwest::Identity, String> {
    match std::fs::read(file_path) {
        Ok(file_bytes) => match reqwest::Identity::from_pkcs12_der(&file_bytes, &passphrase) {
            Ok(identity) => Ok(identity),
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error.to_string()),
    }
}
