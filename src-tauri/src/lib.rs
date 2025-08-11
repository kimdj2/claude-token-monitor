#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Arc;
use tauri::{Manager, WindowEvent};

pub mod domain;
pub mod infrastructure;
pub mod application;

use infrastructure::{
  ccusage_repository::CcusageRepository,
  tauri_commands::{self, AppState},
  tray,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      tauri_commands::get_claude_usage,
      tauri_commands::get_usage_summary,
      tauri_commands::hide_main_window,
      tauri_commands::toggle_main_window,
      tauri_commands::request_permissions,
      tauri_commands::emergency_show_window,
      tauri_commands::show_system_notification,
      tauri_commands::play_warning_sound,
    ])
    .setup(|app| {
            let usage_repo = Arc::new(CcusageRepository::new());
      let app_state = AppState { usage_repo };

      app.manage(app_state);

      match tray::create_tray(&app.handle()) {
        Ok(_) => {},
        Err(e) => return Err(e),
      }

      let main_window = app.get_webview_window("main").unwrap();
      let window_clone = main_window.clone();
      main_window.on_window_event(move |event| {
        match event {
          WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let _ = window_clone.hide();
          }
          WindowEvent::Focused(focused) => {
            if !focused {
              println!("🔍 Window lost focus, checking if should hide...");
              let window_for_hide = window_clone.clone();
              std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                // Double check if window is still unfocused and visible
                if let Ok(is_focused) = window_for_hide.is_focused() {
                  if !is_focused {
                    if let Ok(is_visible) = window_for_hide.is_visible() {
                      if is_visible {
                        println!("🔍 Confirmed: Window lost focus, hiding now");
                        let _ = window_for_hide.hide();
                      }
                    }
                  } else {
                    println!("🔍 Window regained focus, not hiding");
                  }
                }
              });
            } else {
              println!("🔍 Window gained focus");
            }
          }
          _ => {}
        }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}