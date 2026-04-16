use crate::app_state::ClickerState;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

static LAST_ZONE_SHOW: Mutex<Option<Instant>> = Mutex::new(None);
pub static OVERLAY_THREAD_RUNNING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);

pub fn init_overlay(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("overlay")
        .ok_or_else(|| "Overlay window not found".to_string())?;

    log::info!("[Overlay] Running one-time init...");

    window
        .set_ignore_cursor_events(true)
        .map_err(|e| e.to_string())?;
    let _ = window.set_fullscreen(true);
    let _ = window.set_decorations(false);

    log::info!("[Overlay] Init complete — window configured but hidden");
    Ok(())
}

pub fn show_overlay(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<ClickerState>();
    if !state.settings_initialized.load(Ordering::SeqCst) {
        return Ok(());
    }
    {
        let settings = state.settings.lock().unwrap();
        if !settings.show_stop_overlay {
            return Ok(());
        }
    }

    let window = app
        .get_webview_window("overlay")
        .ok_or_else(|| "Overlay window not found".to_string())?;

    *LAST_ZONE_SHOW.lock().unwrap() = Some(Instant::now());

    // Get screen dimensions
    let monitor = app
        .primary_monitor()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No primary monitor found".to_string())?;

    let scale = monitor.scale_factor(); // Adjust for display scaling
    let sw = (monitor.size().width as f64 / scale) as u32;
    let sh = (monitor.size().height as f64 / scale) as u32;

    let settings = state.settings.lock().unwrap();
    let _ = window.emit(
        "zone-data",
        serde_json::json!({
            "edgeStopEnabled": settings.edge_stop_enabled,
            "edgeStopTop": settings.edge_stop_top,
            "edgeStopRight": settings.edge_stop_right,
            "edgeStopBottom": settings.edge_stop_bottom,
            "edgeStopLeft": settings.edge_stop_left,
            "cornerStopEnabled": settings.corner_stop_enabled,
            "cornerStopTL": settings.corner_stop_tl,
            "cornerStopTR": settings.corner_stop_tr,
            "cornerStopBL": settings.corner_stop_bl,
            "cornerStopBR": settings.corner_stop_br,
            "screenWidth": sw,
            "screenHeight": sh,
            "_showDisabledEdges": !settings.edge_stop_enabled,
            "_showDisabledCorners": !settings.corner_stop_enabled,
        }),
    );

    Ok(())
}

// ---- Background timer ----

pub fn check_auto_hide(app: &AppHandle) {
    let mut last = LAST_ZONE_SHOW.lock().unwrap();
    if let Some(instant) = *last {
        if instant.elapsed() >= Duration::from_secs(3) {
            // ↑ auto-hide after timer

            *last = None;
            if let Some(window) = app.get_webview_window("overlay") {
                log::info!("[Overlay] Auto-hide: hiding window");
            }
        }
    }
}

#[tauri::command]
pub fn hide_overlay(app: AppHandle) -> Result<(), String> {
    *LAST_ZONE_SHOW.lock().unwrap() = None;
    if let Some(window) = app.get_webview_window("overlay") {
        #[cfg(target_os = "windows")]
        {
            if let Ok(hwnd) = get_hwnd(&window) {
                unsafe { ShowWindow(hwnd, 0) };
            }
        }
        #[cfg(not(target_os = "windows"))]
        let _ = window.hide();
    }
    Ok(())
}