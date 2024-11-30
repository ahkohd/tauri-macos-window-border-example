use border::{BorderConfig, WebviewWindowExt};
use tauri::{window::Color, WebviewWindow};

#[tauri::command]
fn add_border(window: WebviewWindow) {
    if window.border().is_none() {
        window.add_border(Some(BorderConfig {
            line_color: Color(0, 255, 0, 255),
            ..Default::default()
        }));
    }
}

#[tauri::command]
fn remove_border(window: WebviewWindow) {
    if let Some(border) = window.border() {
        border.remove();
    }
}

#[tauri::command]
fn change_border_color(window: WebviewWindow, color: Color) {
    if let Some(border) = window.border() {
        border.set_line_color(color);
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            add_border,
            remove_border,
            change_border_color
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
