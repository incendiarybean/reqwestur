use eframe::egui;

use crate::{ui::widgets::notification::Notification, utils::traits::ToColour};

/// HTTP method mapping
#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum Method {
    #[default]
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

/// Implement the ToString function, using std::fmt::Display causes a stack overflow
impl ToString for Method {
    /// Convert the method type to string
    fn to_string(&self) -> String {
        let str = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        };

        str.to_string()
    }
}

/// Implement the ToColour function
impl ToColour for Method {
    /// Convert the method type to associated colour
    fn to_colour(&self) -> egui::Color32 {
        match self {
            Method::GET => egui::Color32::LIGHT_BLUE,
            Method::POST => egui::Color32::ORANGE,
            Method::PUT => egui::Color32::ORANGE,
            Method::PATCH => egui::Color32::ORANGE,
            Method::DELETE => egui::Color32::RED,
        }
    }
}

impl Method {
    /// A list to offer all method types for iteration
    const OPTIONS: [Self; 5] = [Self::GET, Self::POST, Self::PUT, Self::PATCH, Self::DELETE];

    /// Return an iterable of the available methods
    pub fn values() -> Vec<Method> {
        Vec::from(Self::OPTIONS)
    }
}

/// An alias to annotate the u16 type as a HTTP status code
type StatusCode = u16;

impl ToColour for StatusCode {
    /// Converts the current u16 value to a colour value (based on HTTP status codes)
    fn to_colour(&self) -> egui::Color32 {
        match self {
            200..=399 => egui::Color32::from_rgb(60, 215, 60),
            400..=499 => egui::Color32::ORANGE,
            _ => egui::Color32::RED,
        }
    }
}

/// The struct containing the HTTP response
#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Response {
    pub status_code: StatusCode,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

/// The Content-Type of the request
#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum ContentType {
    #[default]
    EMPTY,
    TEXT,
    MULTIPART,
    XWWWFORMURLENCODED,
    JSON,
}

/// Implement the ToString function, using std::fmt::Display causes a stack overflow
impl ToString for ContentType {
    /// Convert the content type to string
    fn to_string(&self) -> String {
        let str = match self {
            ContentType::XWWWFORMURLENCODED => "application/x-www-form-urlencoded",
            ContentType::MULTIPART => "multipart/form-data",
            ContentType::JSON => "application/json",
            ContentType::TEXT => "text/plain",
            ContentType::EMPTY => "empty",
        };

        str.to_string()
    }
}

/// The struct containing the content type
impl ContentType {
    const OPTIONS: [Self; 5] = [
        Self::XWWWFORMURLENCODED,
        Self::JSON,
        Self::EMPTY,
        Self::TEXT,
        Self::MULTIPART,
    ];

    pub fn values() -> Vec<Self> {
        Vec::from(Self::OPTIONS)
    }
}

/// The struct containing the request address details
#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Address {
    pub uri: String,
    pub notification: Option<Notification>,
}

/// The request event types
#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum RequestEvent {
    #[default]
    UNSENT,
    PENDING,
    SENT,
}

/// The struct containing the HTTP request details
#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Request {
    /// Contains whether the request is valid and sendable
    pub sendable: bool,

    /// Contains the request Method
    pub method: Method,

    /// Contains the request Headers
    pub headers: Vec<(String, String)>,

    /// Contains the request URI
    pub address: Address,

    /// Contains the request's timestamp
    pub timestamp: String,

    /// Contains the request's content-type
    pub content_type: ContentType,

    /// Contains the request's body
    pub body: Option<String>,

    /// Contains the request's formdata/params
    pub params: Vec<(String, String)>,

    /// Contains the request's response
    pub response: Response,

    /// Contains any notification related to the overall request
    pub notification: Option<Notification>,

    pub event: RequestEvent,
}

impl Request {
    /// A setter to set the notification
    pub fn notification(&mut self, notification: &Notification) {
        self.notification = Some(notification.to_owned());
    }
}
