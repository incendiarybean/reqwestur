use std::str::FromStr;

use eframe::egui;
use reqwest::header::IntoHeaderName;

use crate::ui;

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

    pub fn values() -> Vec<Method> {
        Vec::from(Self::OPTIONS)
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Response {
    pub status_code: u16,
    pub body: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum BodyType {
    MULTIPART,
    #[default]
    JSON,
}

impl BodyType {
    const OPTIONS: [Self; 2] = [Self::MULTIPART, Self::JSON];

    pub fn to_string(&self) -> String {
        let str = match self {
            BodyType::MULTIPART => "multipart/form-data",
            BodyType::JSON => "application/json",
        };

        str.to_string()
    }

    pub fn values() -> Vec<BodyType> {
        Vec::from(Self::OPTIONS)
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Request {
    pub uri: String,
    pub method: Method,
    pub headers: Vec<Header>,
    pub response: Response,
    pub body: String,
    pub body_type: BodyType,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Reqwestur {
    // Request
    pub request: Request,

    #[serde(skip)]
    pub client: reqwest::Client,

    // History
    pub history: Vec<Request>,

    // Editors
    pub headers_editor_open: bool,
    pub body_editor_open: bool,

    // Panel
    pub request_panel_minimised: bool,
    pub history_panel_minimised: bool,
}

impl Default for Reqwestur {
    fn default() -> Self {
        Self {
            // Request
            client: reqwest::Client::default(),
            request: Request::default(),

            // History
            history: Vec::new(),

            // Editors
            headers_editor_open: false,
            body_editor_open: false,

            // Panels
            request_panel_minimised: false,
            history_panel_minimised: true,
        }
    }
}

impl Reqwestur {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            // Handle our own state here:
            // The basic state is ok being managed by the app
            // The Proxy state needs adjusting as it contains Mutex state which doesn't reimplement well
            let previous_values: Reqwestur =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            // Create new app to generate mutables
            return Self {
                // request: Request::default(),
                headers_editor_open: false,
                ..previous_values
            };
        }

        Default::default()
    }

    async fn send(&self, request: Request) -> Result<reqwest::Response, reqwest::Error> {
        let client = &self.client.clone();
        let uri = request.uri.clone();
        let method = request.method.clone();

        let mut header_map = reqwest::header::HeaderMap::new();
        let _ = request.headers.clone().iter().map(|header| {
            header_map.append(
                reqwest::header::HeaderName::from_bytes(header.key.clone().as_bytes()).unwrap(),
                header.value.parse().unwrap(),
            )
        });

        let built_request = match method {
            Method::GET => client.get(&uri),
            Method::POST => client.post(&uri),
            Method::PUT => client.put(&uri),
            Method::PATCH => client.patch(&uri),
            Method::DELETE => client.delete(&uri),
        }
        .headers(header_map)
        .build();

        client.execute(built_request.unwrap()).await
    }
}

impl eframe::App for Reqwestur {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            stroke: egui::Stroke::new(0., egui::Color32::LIGHT_GRAY),
            outer_margin: 0.1.into(),
            ..Default::default()
        };

        // Main layout of UI, task_bar top and main_body bottom
        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui::window::window(self, ui);
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
