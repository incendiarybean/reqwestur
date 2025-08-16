use eframe::egui;

use crate::{
    ui::widgets::{
        about, certificates, headers, help, history, home, menu, payload, request, response,
        saved_requests, task_bar::task_bar,
    },
    utils::{
        request::Request,
        reqwestur::{AppShortcuts, AppView, Reqwestur},
    },
};

pub const PRIMARY: &'static str = "#1b3c79";
pub const _SECONDARY: &'static str = "#112e65";

/// Main Window controller of the UI
pub fn window(app: &mut Reqwestur, ui: &mut egui::Ui, shortcuts: AppShortcuts) {
    register_keyboard_shortcuts(app, ui, shortcuts);

    let max_width = if ui.available_width() < 500. {
        ui.available_width()
    } else {
        ui.available_width() / 3.
    };

    let app_clone = app.clone();
    let mut request = app_clone.request.lock().unwrap();

    /////////////
    // Main UI //
    /////////////

    ui.add(task_bar(app, &mut request));
    ui.add(menu::panel(app, max_width));

    // A banner to track incoming notifications
    app.notification.banner(ui);

    match app.view {
        AppView::Main => {
            ui.add(home::panel(app));
        }
        AppView::Request => {
            ui.add(request::panel(app, &mut request));
            ui.add(response::panel(&mut request));
        }
        AppView::Saved => {
            ui.add(saved_requests::panel(app, &mut request));
        }
        AppView::History => {
            ui.add(history::panel(app, &mut request));
        }
    }

    //////////////////////
    // Editors / Modals //
    //////////////////////

    if app.header_editor_open {
        headers::editor(app, &mut request, ui);
    }

    if app.payload_editor_open {
        payload::editor(app, &mut request, ui);
    }

    if app.certificate_editor_open {
        certificates::editor(app, ui)
    }

    if app.about_modal_open {
        about::panel(app, ui);
    }

    if app.help_modal_open {
        help::panel(app, ui);
    }
}

/// Register the keyboard shortcuts
fn register_keyboard_shortcuts(app: &mut Reqwestur, ui: &mut egui::Ui, shortcuts: AppShortcuts) {
    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.new)) {
        *app.request.lock().unwrap() = Request::default();
        app.view = AppView::Request;
    }

    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.history)) {
        app.view = AppView::History;
    }

    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.save)) {
        todo!("Create a save option!")
    }

    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.open)) {
        app.view = AppView::Saved;
    }

    if ui.input_mut(|input| input.consume_shortcut(&shortcuts.hide_menu)) {
        app.menu_minimised = !app.menu_minimised;
    }
}
