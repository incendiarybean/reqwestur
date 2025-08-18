use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use eframe::egui::{self};

use crate::{
    ui::{
        self,
        widgets::notification::{Notification, NotificationKind},
    },
    utils::{
        certificates::{Certificate, CertificateStatus},
        request::{ContentType, Method, Request, RequestEvent, Response},
    },
};

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum AppView {
    #[default]
    Main,
    Request,
    Saved,
    History,
}

/// A struct containing application shortcut keybindings
pub struct AppShortcuts {
    pub save: egui::KeyboardShortcut,
    pub new: egui::KeyboardShortcut,
    pub history: egui::KeyboardShortcut,
    pub open: egui::KeyboardShortcut,
    pub hide_menu: egui::KeyboardShortcut,
}

impl Default for AppShortcuts {
    fn default() -> Self {
        Self {
            save: egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::S),
            new: egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::N),
            history: egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::H),
            open: egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::O),
            hide_menu: egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::B),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Reqwestur {
    // Display & Navigation
    pub is_dark_mode: bool,
    pub menu_minimised: bool,
    pub view: AppView,

    // Request Panel
    pub request: Arc<Mutex<Request>>,
    pub saved_requests: Vec<Request>,
    pub certificate: Option<Certificate>,
    pub use_certificate_authentication: bool,

    // History Panel
    pub history: Arc<Mutex<Vec<Request>>>,

    // Editors
    pub header_editor_open: bool,
    pub payload_editor_open: bool,
    pub certificate_editor_open: bool,

    // Modals
    pub help_modal_open: bool,
    pub about_modal_open: bool,

    // Alerts
    pub notification: Notification,
}

impl Default for Reqwestur {
    fn default() -> Self {
        Self {
            // Display & Navigation
            is_dark_mode: false,
            menu_minimised: false,
            view: AppView::default(),

            // Request
            request: Arc::new(Mutex::new(Request::default())),
            saved_requests: Vec::new(),
            certificate: None,
            use_certificate_authentication: false,

            // History
            history: Arc::new(Mutex::new(Vec::new())),

            // Editors
            header_editor_open: false,
            payload_editor_open: false,
            certificate_editor_open: false,

            // Modals
            help_modal_open: false,
            about_modal_open: false,

            // Alerts
            notification: Notification::default(),
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

            cc.egui_ctx.add_font(egui::epaint::text::FontInsert::new(
                "OpenSans",
                egui::FontData::from_static(include_bytes!("../assets/OpenSans-Medium.ttf")),
                vec![
                    egui::epaint::text::InsertFontFamily {
                        family: egui::FontFamily::Monospace,
                        priority: egui::epaint::text::FontPriority::Highest,
                    },
                    egui::epaint::text::InsertFontFamily {
                        family: egui::FontFamily::Proportional,
                        priority: egui::epaint::text::FontPriority::Highest,
                    },
                ],
            ));

            cc.egui_ctx.set_theme(if previous_values.is_dark_mode {
                egui::Theme::Dark
            } else {
                egui::Theme::Light
            });

            // Create new app to generate mutables
            return Self {
                // Create fresh request
                request: Arc::new(Mutex::new(Request::default())),

                // Reset window values
                header_editor_open: false,
                payload_editor_open: false,
                certificate_editor_open: false,
                help_modal_open: false,
                about_modal_open: false,

                // Restore old values
                ..previous_values
            };
        }

        Default::default()
    }

    /// A function to send the built request
    pub fn send(&mut self) -> Result<Response, Notification> {
        self.request.lock().unwrap().event = RequestEvent::PENDING;
        let mut request = self.request.lock().unwrap().clone();

        request.response = Response::default();

        let Request {
            method,
            headers,
            address,
            timestamp: _,
            body,
            content_type,
            params,
            response: _,
            notification: _,
            event: _,
        } = request.clone();

        let mut client_builder = reqwest::blocking::ClientBuilder::new();

        if let Some(certificate) = &mut self.certificate {
            if certificate.file_path.exists() && !certificate.passphrase.is_empty() {
                let (kind, message) = match certificate.import() {
                    Ok(identity) => {
                        certificate.status = CertificateStatus::OK;
                        certificate.identity = Some(identity);
                        (
                            NotificationKind::INFO,
                            "Certificate loaded successfully!".to_string(),
                        )
                    }
                    Err(error) => {
                        certificate.status = CertificateStatus::OK;
                        (NotificationKind::ERROR, error)
                    }
                };
                certificate.notification = Notification::new(message, kind);
            }

            if let Some(identity) = certificate.identity.clone() {
                client_builder = client_builder.identity(identity);
            } else {
                let notification = Notification::new(
                    "Cannot find certificates, have you added them?",
                    NotificationKind::WARN,
                );

                request.notification(&notification);
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

        if request.content_type != ContentType::EMPTY {
            built_request = match content_type {
                ContentType::MULTIPART => {
                    let mut form = reqwest::blocking::multipart::Form::new();
                    for (name, value) in params {
                        form = form.text(name, value);
                    }
                    built_request.multipart(form)
                }
                ContentType::XWWWFORMURLENCODED => built_request.form(&params),
                _ => {
                    if let Some(body) = body {
                        built_request.body(body)
                    } else {
                        built_request
                    }
                }
            };
        }

        let http_request = built_request.build().unwrap();

        let (status, headers, cookies, text) = match client.execute(http_request) {
            Ok(response) => (
                (
                    response.status().as_u16(),
                    response
                        .status()
                        .canonical_reason()
                        .unwrap_or("UNKNOWN")
                        .to_string(),
                ),
                response.headers().clone(),
                response
                    .cookies()
                    .map(|cookie| (cookie.value().to_string()))
                    .collect::<Vec<String>>(),
                match response.text() {
                    Ok(str) => str,
                    Err(_) => todo!(),
                },
            ),
            Err(response) => (
                (
                    response
                        .status()
                        .unwrap_or(reqwest::StatusCode::BAD_REQUEST)
                        .as_u16(),
                    response
                        .status()
                        .unwrap_or(reqwest::StatusCode::BAD_REQUEST)
                        .canonical_reason()
                        .unwrap_or("UNKNOWN")
                        .to_string(),
                ),
                reqwest::header::HeaderMap::new(),
                Vec::new(),
                response.to_string(),
            ),
        };

        let pretty_string = match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(json) => serde_json::to_string_pretty(&json),
            Err(_) => Ok(text),
        };

        if let Err(error) = pretty_string {
            let notification = Notification::new(
                format!("Could not prettify JSON - {:?}", error),
                NotificationKind::ERROR,
            );
            request.notification(&notification);
            return Err(notification);
        }

        let simple_headers: Vec<(String, String)> = headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_str().unwrap().to_string()))
            .collect();

        let response = Response {
            status: status,
            headers: simple_headers,
            body: pretty_string.unwrap(),
            cookies,
            ..Default::default()
        };

        request.notification(&Notification::new(
            "Sent successfully.",
            NotificationKind::INFO,
        ));
        request.response = response.clone();
        request.timestamp = chrono::Utc::now().format("%d/%m/%Y %H:%M").to_string();
        request.event = RequestEvent::SENT;

        *self.request.lock().unwrap() = request;

        Ok(response)
    }
}

impl eframe::App for Reqwestur {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let shortcuts = AppShortcuts::default();
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
                ui::window::window(self, ui, shortcuts);
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
