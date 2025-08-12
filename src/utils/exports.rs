use serde_json::{Value, json};

use crate::{ui::widgets::notification::Notification, utils::request::Request};

#[derive(serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum RequestSourceType {
    HISTORY,
    SAVED,
}

impl ToString for RequestSourceType {
    fn to_string(&self) -> String {
        let str = match self {
            RequestSourceType::HISTORY => "reqwestur_history",
            RequestSourceType::SAVED => "reqwestur_saved_requests",
        };

        str.to_owned()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Eq, PartialEq)]
pub enum ExportType {
    JSON,
    HTTP,
}

impl ToString for ExportType {
    fn to_string(&self) -> String {
        let str = match self {
            ExportType::JSON => "json",
            ExportType::HTTP => "http",
        };

        str.to_owned()
    }
}

impl ExportType {
    const OPTIONS: [Self; 2] = [Self::HTTP, Self::JSON];

    pub fn values() -> Vec<Self> {
        Self::OPTIONS.to_vec()
    }
}

pub struct ReqwesturIO {
    requests: Vec<Request>,
    file_path: std::path::PathBuf,
    export_type: ExportType,
}

impl ReqwesturIO {
    pub fn new(
        requests: Vec<Request>,
        source_type: RequestSourceType,
        export_type: ExportType,
    ) -> Option<Self> {
        if let Some(file_path) = rfd::FileDialog::new()
            .set_file_name(format!(
                "{}.{}",
                source_type.to_string(),
                export_type.to_string()
            ))
            .save_file()
        {
            Some(Self {
                requests,
                file_path,
                export_type,
            })
        } else {
            None
        }
    }

    pub fn export(&self) -> Result<(), Notification> {
        match self.export_type {
            ExportType::JSON => {
                let mut json_requests: Vec<Value> = Vec::default();
                for request in self.requests.clone() {
                    json_requests.push(json!({
                        "method": request.method,
                        "contentType": request.content_type,
                        "uri": request.address.uri,
                        "headers": request.headers,
                        "body": request.body,
                        "params": request.params
                    }));
                }

                let body = serde_json::to_string_pretty(&json!(json_requests)).unwrap();
                let _ = std::fs::write(self.file_path.clone(), body);
            }
            ExportType::HTTP => {
                let http_requests: Vec<String> = Vec::default();
                for request in self.requests.clone() {
                    let mut http_request: Vec<String> = Vec::default();

                    // Method and URI
                    http_request.push(format!(
                        "{} {}",
                        request.method.to_string(),
                        request.address.uri
                    ));

                    // Headers
                    let mut headers: String = String::default();
                    for (name, value) in request.headers {
                        headers += &format!("{name}={value}\n");
                    }
                    http_request.push(headers);

                    // Params
                    if !request.params.is_empty() {
                        let mut params: Vec<String> = Vec::default();
                        for (name, value) in request.params {
                            params.push(format!("{name}={value}"));
                        }

                        http_request.push(params.join("\n&") + "\n");
                    }

                    // Body
                    if let Some(body) = request.body {
                        http_request.push(format!("{body}\n"));
                    }
                }

                let body = http_requests.join("\n###\n\n");
                let _ = std::fs::write(self.file_path.clone(), body);
            }
        }

        Ok(())
    }

    pub fn _import(&self) -> Result<(), Notification> {
        Ok(())
    }
}
