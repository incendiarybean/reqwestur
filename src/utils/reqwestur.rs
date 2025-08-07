use std::str::FromStr;

use eframe::egui::{self};

use crate::{
    ui::{
        self,
        widgets::notification::{Notification, NotificationKind},
    },
    utils::{
        certificates::{Certificates, CertificatesStatus},
        request::{ContentType, Method, Request, Response},
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
    pub request: Request,
    pub certificates: Certificates,

    // History Panel
    pub history: Vec<Request>,

    // Editors
    pub header_editor_open: bool,
    pub payload_editor_open: bool,
    pub certificate_editor_open: bool,

    // Modals
    pub help_modal_open: bool,
    pub about_modal_open: bool,
}

impl Default for Reqwestur {
    fn default() -> Self {
        Self {
            // Display & Navigation
            is_dark_mode: false,
            menu_minimised: false,
            view: AppView::default(),

            // Request
            request: Request::default(),
            certificates: Certificates::default(),

            // History
            history: Vec::new(),

            // Editors
            header_editor_open: false,
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
                request: Request::default(),

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
        if !self.check_sendable() {
            let notification = Notification {
                kind: NotificationKind::ERROR,
                message: format!("The request is not in a sendable state"),
            };
            self.request.notification(&notification);
            return Err(notification);
        }

        self.request.response = Response::default();

        let Request {
            method,
            headers,
            address,
            timestamp: _,
            body,
            content_type,
            form_data,
            sendable: _,
            response: _,
            notification: _,
        } = self.request.clone();

        let mut client_builder = reqwest::blocking::ClientBuilder::new();

        if self.certificates.required {
            if self.certificates.file_path.exists() && !self.certificates.passphrase.is_empty() {
                let (kind, message) = match self.certificates.import() {
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
                self.request.notification(&notification);
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

        if self.request.content_type != ContentType::EMPTY {
            built_request = match content_type {
                ContentType::MULTIPART => {
                    let mut form = reqwest::blocking::multipart::Form::new();
                    for (name, value) in form_data {
                        form = form.text(name, value);
                    }
                    built_request.multipart(form)
                }
                ContentType::XWWWFORMURLENCODED => built_request.form(&form_data),
                _ => {
                    if let Some(body) = body {
                        built_request.body(body)
                    } else {
                        built_request
                    }
                }
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
            self.request.notification(&notification);
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

        self.request.notification(&Notification {
            kind: NotificationKind::INFO,
            message: "Sent successfully.".to_string(),
        });
        self.request.response = response.clone();
        self.request.timestamp = chrono::Utc::now().format("%d/%m/%Y %H:%M").to_string();
        self.history.push(self.request.clone());

        Ok(response)
    }

    /// A function to check if the request is in a sendable state
    pub fn check_sendable(&mut self) -> bool {
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

        self.request.sendable
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
