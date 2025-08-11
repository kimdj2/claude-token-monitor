use std::sync::Arc;
use tauri::{State, Manager};
use crate::domain::{
  entities::{UsageStats, UsagePeriodSummary},
  repository::UsageRepository,
};
use crate::application::use_cases;

pub struct AppState {
  pub usage_repo: Arc<dyn UsageRepository + Send + Sync>,
}

#[tauri::command]
pub async fn get_claude_usage(state: State<'_, AppState>) -> Result<UsageStats, String> {
  use_cases::get_claude_usage(state.usage_repo.clone()).await
}

#[tauri::command]
pub async fn get_usage_summary(state: State<'_, AppState>, period: String) -> Result<UsagePeriodSummary, String> {
  use_cases::get_usage_summary(state.usage_repo.clone(), period).await
}

#[tauri::command]
pub fn hide_main_window(app: tauri::AppHandle) -> Result<(), String> {
  if let Some(window) = app.get_webview_window("main") {
  window.hide().map_err(|e| e.to_string())?;
  }
  Ok(())
}

#[tauri::command]
pub fn request_permissions() -> Result<(), String> {
  println!("🔍 Permission request called");
  #[cfg(target_os = "macos")]
  {
    println!("⚠️  macOS may require the following permissions:");
  println!("   1. System Settings → Privacy & Security → Accessibility");
  println!("   2. Find 'cc-widgetauri' in the app list and enable it");
  println!("   3. Terminal app may also need accessibility permissions");
  println!("   4. Check screen recording permissions if needed");
  }
  Ok(())
}

#[tauri::command]
pub fn emergency_show_window(app: tauri::AppHandle) -> Result<String, String> {
  println!("🚨 Emergency show window called!");

  if let Some(window) = app.get_webview_window("main") {
  println!("🔍 Window found, attempting emergency show");

  // Force apply all settings
  let _ = window.unminimize();
  let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: 400.0, height: 500.0 }));
  let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x: 100.0, y: 100.0 }));
  let _ = window.set_always_on_top(true);
  let _ = window.set_focus();

  match window.show() {
    Ok(_) => {
      println!("✅ Emergency show successful!");
      Ok("Window displayed successfully!".to_string())
    }
    Err(e) => {
      println!("❌ Emergency show failed: {}", e);
      Err(format!("Failed to show window: {}", e))
    }
  }
  } else {
  println!("❌ Window not found!");
  Err("Window not found".to_string())
  }
}

#[tauri::command]
pub fn toggle_main_window(app: tauri::AppHandle) -> Result<(), String> {
  println!("🔍 toggle_main_window command called");
  if let Some(window) = app.get_webview_window("main") {
  let is_visible = window.is_visible().unwrap_or(false);
  println!("🔍 Window is visible: {}", is_visible);
  match is_visible {
    true => {
      println!("🔍 Hiding window via command");
      window.hide().map_err(|e| e.to_string())?;
    }
    false => {
      println!("🔍 Showing window via command");
      let _ = window.unminimize();
      // Set position to center if not set
      let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x: 100.0, y: 100.0 }));
      window.show().map_err(|e| e.to_string())?;
      let _ = window.set_focus();
      println!("✅ Window should now be visible");
    }
  }
  } else {
  println!("❌ Main window not found in toggle command!");
  return Err("Main window not found".to_string());
  }
  Ok(())
}

#[tauri::command]
pub fn force_show_window(app: tauri::AppHandle) -> Result<(), String> {
  println!("🔍 Force show window called");
  if let Some(window) = app.get_webview_window("main") {
  println!("🔍 Found window, forcing it to show");

  // Force position window to center and show
  let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: 350.0, height: 450.0 }));
  let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x: 200.0, y: 200.0 }));
  let _ = window.unminimize();
  let _ = window.show();
  let _ = window.set_focus();
  let _ = window.set_always_on_top(true);

  println!("✅ Force show completed");
  Ok(())
  } else {
  println!("❌ Window not found");
  Err("Window not found".to_string())
  }
}

#[tauri::command]
pub async fn show_system_notification(
  title: String,
  message: String,
  urgent: bool
) -> Result<(), String> {
  use std::process::Command;

  #[cfg(target_os = "macos")]
  {
      let mut cmd = Command::new("osascript");
      cmd.arg("-e");

      let sound = if urgent { "Basso" } else { "Ping" };
      let script = format!(
      r#"display notification "{}" with title "{}" sound name "{}""#,
      message.replace("\"", "\\\""),
      title.replace("\"", "\\\""),
      sound
      );

      cmd.arg(&script);
      let _ = cmd.output();
  }

  #[cfg(target_os = "windows")]
  {
      // Windows notification using PowerShell
      let mut cmd = Command::new("powershell");
      cmd.arg("-Command");

      let script = format!(
      r#"Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('{}', '{}')"#,
      message.replace("'", "''"),
      title.replace("'", "''")
      );

      cmd.arg(&script);
      let _ = cmd.output();
  }

  #[cfg(target_os = "linux")]
  {
      // Linux notification using notify-send
      let mut cmd = Command::new("notify-send");
      cmd.arg(&title).arg(&message);

      if urgent {
      cmd.arg("-u").arg("critical");
      }

      let _ = cmd.output();
  }

  Ok(())
}

#[tauri::command]
pub async fn play_warning_sound(urgent: bool) -> Result<(), String> {
  use std::process::Command;

  #[cfg(target_os = "macos")]
  {
      let sound = if urgent { "Basso" } else { "Ping" };
      let mut cmd = Command::new("afplay");
      cmd.arg(&format!("/System/Library/Sounds/{}.aiff", sound));
      let _ = cmd.output();
  }

  #[cfg(target_os = "windows")]
  {
      let mut cmd = Command::new("powershell");
      cmd.arg("-c");
      cmd.arg(&format!("[console]::beep({})", if urgent { "800,500" } else { "400,200" }));
      let _ = cmd.output();
  }

  #[cfg(target_os = "linux")]
  {
      let mut cmd = Command::new("paplay");
      cmd.arg("/usr/share/sounds/alsa/Front_Right.wav");
      let _ = cmd.output();
  }

  Ok(())
}
