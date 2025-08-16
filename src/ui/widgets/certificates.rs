use eframe::egui::{self};

use crate::{
    ui::widgets::{
        buttons::default_button,
        notification::{Notification, NotificationKind},
    },
    utils::{
        certificates::{Certificate, CertificateStatus},
        reqwestur::Reqwestur,
    },
};

/// The certificate editor window
pub fn editor(app: &mut Reqwestur, ui: &mut egui::Ui) {
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of("certificate_editor"),
        egui::ViewportBuilder::default()
            .with_title("Certificate Editor")
            .with_inner_size([500.0, 500.0])
            .with_min_inner_size([500.0, 500.0]),
        |context, _class| {
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        let file_name = match &app.certificate {
                            Some(certificate) => {
                                if let Some(file_name) = certificate.file_path.file_name() {
                                    file_name.to_str().unwrap_or("Error reading file name.")
                                } else {
                                    "No file selected."
                                }
                            }
                            None => "No file selected.",
                        };
                        ui.label(format!("Selected File: {}", file_name));

                        let upload_icon = egui::include_image!("../../assets/upload.svg");
                        if ui
                            .add(default_button(
                                Some(upload_icon),
                                "Select a PFX.",
                                ui.available_width(),
                                ui.visuals().text_color(),
                            ))
                            .clicked()
                        {
                            let mut certificate = Certificate::default();
                            if let Some(file) = rfd::FileDialog::new()
                                .add_filter("PFX", &["pfx"])
                                .set_directory("/")
                                .pick_file()
                            {
                                certificate.file_path = file;
                            }

                            app.certificate = Some(certificate);
                        }

                        if let Some(certificate) = &mut app.certificate {
                            if certificate.file_path.exists() {
                                ui.group(|ui| {
                                    ui.label("Certificate Passphrase:");
                                    ui.add_sized(
                                        egui::vec2(ui.available_width(), 20.),
                                        egui::TextEdit::singleline(&mut certificate.passphrase)
                                            .margin(5.)
                                            .password(true),
                                    );
                                });
                            }

                            let bin_icon = egui::include_image!("../../assets/trash.svg");
                            if ui
                                .add(default_button(
                                    Some(bin_icon),
                                    "Remove PFX.",
                                    ui.available_width(),
                                    ui.visuals().text_color(),
                                ))
                                .clicked()
                            {
                                app.certificate = None;
                            }
                        }
                    });
                });

                if let Some(certificate) = &mut app.certificate {
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        let save_icon = egui::include_image!("../../assets/floppy.svg");
                        if ui
                            .add_enabled(
                                certificate.status == CertificateStatus::OK,
                                default_button(
                                    Some(save_icon.clone()),
                                    "Confirm & Close!",
                                    ui.available_width(),
                                    ui.visuals().text_color(),
                                ),
                            )
                            .clicked()
                        {
                            app.certificate_editor_open = false;
                            certificate.notification.clear();
                        }

                        if ui
                            .add_enabled(
                                !certificate.passphrase.is_empty()
                                    && certificate.file_path.file_name().is_some(),
                                default_button(
                                    Some(save_icon),
                                    "Validate Certificates",
                                    ui.available_width(),
                                    ui.visuals().text_color(),
                                ),
                            )
                            .clicked()
                        {
                            match certificate.import() {
                                Ok(identity) => {
                                    certificate.status = CertificateStatus::OK;
                                    certificate.notification = Notification::new(
                                        "Certificate loaded successfully!",
                                        NotificationKind::INFO,
                                    );
                                    certificate.identity = Some(identity);
                                }
                                Err(error) => {
                                    certificate.status = CertificateStatus::ERROR;
                                    certificate.notification =
                                        Notification::new(error, NotificationKind::ERROR);
                                }
                            }
                        }

                        certificate.notification.display(ui);

                        if certificate.status == CertificateStatus::UNCONFIRMED {
                            ui.label("No certificates have been loaded.");
                        }
                    });
                }

                ui.add_space(5.);
            });

            if context.input(|i| i.viewport().close_requested()) {
                app.certificate_editor_open = false;
            }
        },
    );
}
