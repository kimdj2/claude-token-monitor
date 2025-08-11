use tauri::{
  tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
  menu::{MenuBuilder, MenuItem},
  Manager, AppHandle, Emitter, image::Image
};

pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
  let refresh = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;

  let menu = MenuBuilder::new(app)
  .items(&[&refresh, &quit])
  .build()?;

  // Create purple circular tray icon
  let icon = create_purple_tray_icon()?;

  let _tray = TrayIconBuilder::new()
  .menu(&menu)
  .show_menu_on_left_click(false)
  .icon(icon)
  .on_menu_event(move |app, event| {
    match event.id.as_ref() {
      "quit" => app.exit(0),
      "refresh" => {
    if let Some(window) = app.get_webview_window("main") {
      let _ = window.emit("refresh-usage", ());
    }
      }
      _ => {}
    }
  })
  .on_tray_icon_event(|tray, event| {
    match event {
      TrayIconEvent::Click { button, rect, button_state, .. } => {
    let app = tray.app_handle();
    if matches!(button, MouseButton::Left) && matches!(button_state, MouseButtonState::Down) {
      let app_clone = app.clone();
      let rect_clone = rect.clone();
      std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if let Some(window) = app_clone.get_webview_window("main") {
          let is_visible = window.is_visible().unwrap_or(false);
          if is_visible {
      let _ = window.hide();
          } else {
      let _ = position_popup_near_tray(&window, rect_clone);
      let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: 400.0, height: 580.0 }));
      let _ = window.unminimize();
      let _ = window.show();
      let _ = window.set_always_on_top(true);
      let _ = window.set_focus();
      println!("🔍 Tray click: Window shown and focused");
          }
        }
      });
    }
      }
      _ => {}
    }
  })
  .build(app)?;

  Ok(())
}

fn position_popup_near_tray(
  window: &tauri::WebviewWindow,
  tray_rect: tauri::Rect
) -> Result<(), Box<dyn std::error::Error>> {
  let is_empty_rect = match (tray_rect.position, tray_rect.size) {
  (tauri::Position::Physical(pos), tauri::Size::Physical(size)) => {
    pos.x == 0 && pos.y == 0 && size.width == 0 && size.height == 0
  }
  (tauri::Position::Logical(pos), tauri::Size::Logical(size)) => {
    pos.x == 0.0 && pos.y == 0.0 && size.width == 0.0 && size.height == 0.0
  }
  _ => false,
  };

  if is_empty_rect {
  let actual_tray_x = 896.0;
  let x = actual_tray_x;
  let y = 37.0;
  window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }))?;
  return Ok(());
  }
  let monitor = window.current_monitor()?.ok_or("No monitor found")?;
  let monitor_size = monitor.size();
  let scale = monitor.scale_factor();

  let window_width = 400.0;
  let window_height = 580.0;
  let padding = 2.0;

  let (tray_x, tray_y) = match tray_rect.position {
  tauri::Position::Physical(pos) => (pos.x as f64 / scale, pos.y as f64 / scale),
  tauri::Position::Logical(pos) => (pos.x, pos.y),
  };

  let screen_width = monitor_size.width as f64 / scale;
  let screen_height = monitor_size.height as f64 / scale;

  let mut popup_x = tray_x;
  if popup_x + window_width > screen_width {
  popup_x = screen_width - window_width - padding;
  }
  if popup_x < 0.0 {
  popup_x = padding;
  }

  let (_, size_height) = match tray_rect.size {
  tauri::Size::Physical(size) => (size.width as f64 / scale, size.height as f64 / scale),
  tauri::Size::Logical(size) => (size.width, size.height),
  };

  let mut popup_y = tray_y + size_height + padding;
  if popup_y + window_height > screen_height {
  popup_y = tray_y - window_height - padding;
  }
  if popup_y < 0.0 {
  popup_y = padding;
  }
  window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x: popup_x, y: popup_y }))?;
  Ok(())
}

fn create_purple_tray_icon() -> Result<Image<'static>, Box<dyn std::error::Error>> {
  // Create a 16x16 colorful monitoring icon with chart elements
  let size = 16;
  let mut rgba_data = Vec::with_capacity(size * size * 4);

  for y in 0..size {
    for x in 0..size {
      // Create a blue circle background
      let center_x = size as f32 / 2.0;
      let center_y = size as f32 / 2.0;
      let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();

      if distance <= 6.0 {
        // Blue background (34, 139, 230)
        let mut r = 34;
        let mut g = 139;
        let mut b = 230;
        let a = 255;
        
        // Add colorful chart elements
        let is_chart_line = (y == 6 && x >= 4 && x <= 12) ||
                           (y == 8 && x >= 4 && x <= 11) ||
                           (y == 10 && x >= 4 && x <= 10) ||
                           (y == 12 && x >= 4 && x <= 12);
        
        // Colorful chart points
        if x == 5 && y == 6 {
          r = 52; g = 199; b = 89; // Green
        } else if x == 7 && y == 5 {
          r = 255; g = 204; b = 0; // Yellow
        } else if x == 9 && y == 4 {
          r = 255; g = 94; b = 132; // Pink
        } else if x == 11 && y == 3 {
          r = 90; g = 200; b = 250; // Light Blue
        } else if is_chart_line {
          r = 255; g = 255; b = 255; // White lines
        }
        
        rgba_data.push(r);
        rgba_data.push(g);
        rgba_data.push(b);
        rgba_data.push(a);
      } else {
        // Transparent
        rgba_data.push(0);
        rgba_data.push(0);
        rgba_data.push(0);
        rgba_data.push(0);
      }
    }
  }

  let image = Image::new_owned(rgba_data, size as u32, size as u32);
  Ok(image)
}
