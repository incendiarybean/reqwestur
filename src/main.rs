#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::{clone, sync::Arc};

mod ui;

fn main() -> Result<(), eframe::Error> {
    let icon: &[u8] = include_bytes!("assets/icon.png");
    let img: image::DynamicImage = image::load_from_memory(icon).unwrap();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_min_inner_size(egui::vec2(600.0, 400.0))
            .with_resizable(true)
            .with_icon(Arc::new(egui::viewport::IconData {
                rgba: img.into_bytes(),
                width: 288,
                height: 288,
            })),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "REQWESTUR",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            if let Some(theme) = cc.egui_ctx.system_theme() {
                cc.egui_ctx.set_theme(theme);
            } else {
                cc.egui_ctx.set_theme(egui::Theme::Light);
            }

            cc.egui_ctx.style_mut(|style| {
                // style.spacing.item_spacing = egui::vec2(5.0, 5.0);
                style.spacing.button_padding = egui::vec2(5.0, 5.0);
            });

            Ok(Box::new(Reqwestur::new(cc)))
        }),
    )
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
enum Method {
    #[default]
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Method {
    const OPTIONS: [Self; 5] = [Self::GET, Self::POST, Self::PUT, Self::PATCH, Self::DELETE];

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

    fn to_list() -> Vec<Method> {
        Self::OPTIONS.to_vec()
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
struct Response {
    status_code: u16,
    body: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
struct Headers {
    key: String,
    value: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
struct Request {
    uri: String,
    method: Method,
    headers: Vec<Headers>,
    response: Response,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Reqwestur {
    // Request
    request: Request,

    // History
    history: Vec<Request>,

    // Editors
    headers_editor_open: bool,

    // Panel
    request_panel_minimised: bool,
    history_panel_minimised: bool,
}

impl Default for Reqwestur {
    fn default() -> Self {
        Self {
            // Request
            request: Request::default(),

            // History
            history: Vec::new(),

            // Editors
            headers_editor_open: false,

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
                request: Request::default(),
                headers_editor_open: false,
                ..previous_values
            };
        }

        Default::default()
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
