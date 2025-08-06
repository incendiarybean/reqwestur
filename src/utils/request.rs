use eframe::egui;

use crate::utils::notifications::Notification;

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum Method {
    #[default]
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Method {
    const OPTIONS: [Self; 5] = [Self::GET, Self::POST, Self::PUT, Self::PATCH, Self::DELETE];

    pub fn to_string(&self) -> String {
        let str = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        };

        str.to_string()
    }

    pub fn to_colour(&self) -> egui::Color32 {
        match self {
            Method::GET => egui::Color32::LIGHT_BLUE,
            Method::POST => egui::Color32::ORANGE,
            Method::PUT => egui::Color32::ORANGE,
            Method::PATCH => egui::Color32::ORANGE,
            Method::DELETE => egui::Color32::RED,
        }
    }

    pub fn values() -> Vec<Method> {
        Vec::from(Self::OPTIONS)
    }
}

/// Create a colour trait for u16
pub trait ToColour {
    /// Converts the current u16 value to a colour value (based on HTTP status codes)
    fn to_colour(&self) -> egui::Color32;
}

impl ToColour for u16 {
    /// Converts the current u16 value to a colour value (based on HTTP status codes)
    fn to_colour(&self) -> egui::Color32 {
        match self {
            200..=399 => egui::Color32::from_rgb(60, 215, 60),
            400..=499 => egui::Color32::ORANGE,
            _ => egui::Color32::RED,
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Response {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum BodyType {
    #[default]
    EMPTY,
    TEXT,
    MULTIPART,
    XWWWFORMURLENCODED,
    JSON,
}

impl BodyType {
    const OPTIONS: [Self; 3] = [Self::XWWWFORMURLENCODED, Self::JSON, Self::EMPTY];

    pub fn to_string(&self) -> String {
        let str = match self {
            BodyType::XWWWFORMURLENCODED => "application/x-www-form-urlencoded",
            BodyType::MULTIPART => "multipart/form-data",
            BodyType::JSON => "application/json",
            BodyType::TEXT => "text/plain",
            BodyType::EMPTY => "empty",
        };

        str.to_string()
    }

    pub fn values() -> Vec<BodyType> {
        Vec::from(Self::OPTIONS)
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Address {
    pub uri: String,
    pub notification: Option<Notification>,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Request {
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub address: Address,
    pub timestamp: String,

    pub body: Option<String>,
    pub form_data: Vec<(String, String)>,
    pub body_type: BodyType,

    pub sendable: bool,

    pub response: Response,

    pub notification: Option<Notification>,
}
