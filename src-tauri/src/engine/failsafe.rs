use super::mouse::current_screen_size;
use super::ClickerConfig;
use enigo::{Enigo, Settings, Mouse};

pub fn should_stop_for_failsafe(config: &ClickerConfig) -> Option<String> {
    pub fn current_cursor_position() -> Option<(i32, i32)> {
       let mouse = Enigo::new(&Settings::default()).unwrap();
       mouse.location().ok()
    }

    let cursor = current_cursor_position()?;
    let screen = current_screen_size()?;

    if config.corner_stop_enabled {
        if cursor.0 <= config.corner_stop_tl && cursor.1 <= config.corner_stop_tl {
            return Some(String::from("Top-left corner failsafe"));
        }
        if cursor.0 >= screen.0 - config.corner_stop_tr && cursor.1 <= config.corner_stop_tr {
            return Some(String::from("Top-right corner failsafe"));
        }
        if cursor.0 <= config.corner_stop_bl && cursor.1 >= screen.1 - config.corner_stop_bl {
            return Some(String::from("Bottom-left corner failsafe"));
        }
        if cursor.0 >= screen.0 - config.corner_stop_br
            && cursor.1 >= screen.1 - config.corner_stop_br
        {
            return Some(String::from("Bottom-right corner failsafe"));
        }
    }

    if config.edge_stop_enabled {
        if cursor.1 <= config.edge_stop_top {
            return Some(String::from("Top edge failsafe"));
        }
        if cursor.0 >= screen.0 - config.edge_stop_right {
            return Some(String::from("Right edge failsafe"));
        }
        if cursor.1 >= screen.1 - config.edge_stop_bottom {
            return Some(String::from("Bottom edge failsafe"));
        }
        if cursor.0 <= config.edge_stop_left {
            return Some(String::from("Left edge failsafe"));
        }
    }

    None
}
