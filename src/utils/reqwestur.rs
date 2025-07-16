use std::str::FromStr;

use eframe::egui::{self, Color32};

use crate::{ui, utils::common};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Reqwestur {
    // Display
    pub is_dark_mode: bool,
    pub menu_minimised: bool,

    // Request Panel
    pub request_panel_minimised: bool,
    pub request: Request,
    pub certificates: Certificates,

    // History Panel
    pub history_panel_minimised: bool,
    pub history: Vec<Request>,

    // Editors
    pub headers_editor_open: bool,
    pub payload_editor_open: bool,
    pub certificate_editor_open: bool,

    // Modals
    pub help_modal_open: bool,
    pub about_modal_open: bool,
}

impl Default for Reqwestur {
    fn default() -> Self {
        Self {
            // Display
            is_dark_mode: false,
            menu_minimised: false,

            // Request
            request_panel_minimised: false,
            request: Request::default(),
            certificates: Certificates::default(),

            // History
            history_panel_minimised: true,
            history: Vec::new(),

            // Editors
            headers_editor_open: false,
            payload_editor_open: false,
            certificate_editor_open: false,

            // Modals
            help_modal_open: false,
            about_modal_open: false,
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

            cc.egui_ctx.all_styles_mut(|style| {
                style.spacing.button_padding = egui::vec2(5.0, 5.0);
            });

            cc.egui_ctx.set_theme(if previous_values.is_dark_mode {
                egui::Theme::Dark
            } else {
                egui::Theme::Light
            });

            // Create new app to generate mutables
            return Self {
                // Create fresh request
                request: Request::default(),

                // Reset window values
                headers_editor_open: false,
                certificate_editor_open: false,
                help_modal_open: false,
                about_modal_open: false,

                // Restore old values
                ..previous_values
            };
        }

        Default::default()
    }

    pub fn send(&mut self) -> Result<Response, Notification> {
        self.request.response = Response::default();

        let Request {
            method,
            headers,
            address,
            body,
            body_type,
            sendable: _,
            response: _,
            notification: _,
        } = self.request.clone();

        let mut client_builder = reqwest::blocking::ClientBuilder::new();

        if self.certificates.required {
            if self.certificates.file_path.exists() && !self.certificates.passphrase.is_empty() {
                let (kind, message) = match common::load_certificates(
                    &self.certificates.file_path,
                    &self.certificates.passphrase,
                ) {
                    Ok(identity) => {
                        self.certificates.status = CertificatesStatus::OK;
                        self.certificates.identity = Some(identity);
                        (
                            NotificationKind::INFO,
                            "Certificate loaded successfully!".to_string(),
                        )
                    }
                    Err(error) => {
                        self.certificates.status = CertificatesStatus::OK;
                        (NotificationKind::ERROR, error)
                    }
                };
                self.certificates.notification = Some(Notification { kind, message });
            }

            if let Some(identity) = self.certificates.identity.clone() {
                client_builder = client_builder.identity(identity);
            } else {
                let notification = Notification {
                    kind: NotificationKind::WARN,
                    message: "Cannot find certificates, have you added them?".to_string(),
                };
                self.request.notification = Some(notification.clone());

                return Err(notification);
            }
        }

        let client = client_builder
            .default_headers(reqwest::header::HeaderMap::from_iter([(
                reqwest::header::USER_AGENT,
                reqwest::header::HeaderValue::from_str("REQWESTUR").unwrap(),
            )]))
            .build()
            .unwrap();

        let mut built_request = match method {
            Method::GET => client.get(&address.uri),
            Method::POST => client.post(&address.uri),
            Method::PUT => client.put(&address.uri),
            Method::PATCH => client.patch(&address.uri),
            Method::DELETE => client.delete(&address.uri),
        };

        if !headers.is_empty() {
            let mut header_list = reqwest::header::HeaderMap::new();
            headers.iter().for_each(|(name, value)| {
                header_list.append(
                    reqwest::header::HeaderName::from_str(&name).unwrap(),
                    reqwest::header::HeaderValue::from_bytes(&value.as_bytes()).unwrap(),
                );
            });

            built_request = built_request.headers(header_list)
        }

        if [BodyType::JSON, BodyType::MULTIPART].contains(&self.request.body_type) {
            built_request = match body_type {
                BodyType::MULTIPART => built_request.multipart(todo!()),
                BodyType::JSON => {
                    if let Some(body) = body {
                        built_request.body(body)
                    } else {
                        built_request
                    }
                }
                _ => built_request,
            };
        }

        let request = built_request.build().unwrap();

        let (status, headers, text) = match client.execute(request) {
            Ok(response) => (
                response.status().as_u16(),
                response.headers().clone(),
                match response.text() {
                    Ok(str) => str,
                    Err(_) => todo!(),
                },
            ),
            Err(response) => (
                response
                    .status()
                    .unwrap_or(reqwest::StatusCode::BAD_REQUEST)
                    .as_u16(),
                reqwest::header::HeaderMap::new(),
                response.to_string(),
            ),
        };

        let pretty_string = match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(json) => serde_json::to_string_pretty(&json),
            Err(_) => Ok(text),
        };

        if let Err(error) = pretty_string {
            let notification = Notification {
                kind: NotificationKind::ERROR,
                message: format!("Could not prettify JSON - {:?}", error),
            };
            self.request.notification = Some(notification.clone());
            return Err(notification);
        }

        let simple_headers: Vec<(String, String)> = headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_str().unwrap().to_string()))
            .collect();

        let response = Response {
            status_code: status,
            headers: simple_headers,
            body: pretty_string.unwrap(),
        };

        let notification = Notification {
            kind: NotificationKind::INFO,
            message: "Sent successfully.".to_string(),
        };

        self.request.notification = Some(notification);
        self.request.response = response.clone();
        self.history.push(self.request.clone());

        Ok(response)
    }

    pub fn check_sendable(&mut self) {
        let uri_ok: bool = reqwest::Url::parse(&self.request.address.uri).is_ok();

        let certificate_ok = if self.certificates.required {
            if self.certificates.status == CertificatesStatus::OK {
                true
            } else {
                false
            }
        } else {
            true
        };

        let no_errors = [
            &self.certificates.notification,
            &self.request.address.notification,
        ]
        .iter()
        .all(|notification| {
            if let Some(notification) = notification {
                notification.kind != NotificationKind::ERROR
            } else {
                true
            }
        });

        self.request.sendable = uri_ok && certificate_ok && no_errors;
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
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum BodyType {
    #[default]
    EMPTY,
    MULTIPART,
    JSON,
}

impl BodyType {
    const OPTIONS: [Self; 3] = [Self::MULTIPART, Self::JSON, Self::EMPTY];

    pub fn to_string(&self) -> String {
        let str = match self {
            BodyType::MULTIPART => "multipart/form-data",
            BodyType::JSON => "application/json",
            BodyType::EMPTY => "text/plain",
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

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum NotificationKind {
    #[default]
    INFO,
    ERROR,
    WARN,
}

impl NotificationKind {
    pub fn to_colour(&self) -> Color32 {
        match self {
            NotificationKind::INFO => Color32::GREEN,
            NotificationKind::ERROR => Color32::RED,
            NotificationKind::WARN => Color32::ORANGE,
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Notification {
    pub kind: NotificationKind,
    pub message: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum CertificatesStatus {
    #[default]
    UNCONFIRMED,
    OK,
    ERROR,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Certificates {
    pub required: bool,
    pub file_path: std::path::PathBuf,
    pub passphrase: String,
    pub status: CertificatesStatus,
    pub notification: Option<Notification>,

    #[serde(skip)]
    pub identity: Option<reqwest::Identity>,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Request {
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub address: Address,

    pub body: Option<String>,
    pub body_type: BodyType,

    pub sendable: bool,

    pub response: Response,

    pub notification: Option<Notification>,
}
