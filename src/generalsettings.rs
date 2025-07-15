use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub max_log_string_length: usize,
    pub log_file_default_name: String,
    pub log_file_default_extension: String,
    pub clear_message_after_send: bool,
    pub auto_scroll_log: bool,
    pub update_check_on_startup: bool,
    pub repaint_interval_ms: u64,
    pub window_width: f32,
    pub window_height: f32,
    pub byte_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            max_log_string_length: 30000,
            log_file_default_name: "LogFile".to_string(),
            log_file_default_extension: ".txt".to_string(),
            clear_message_after_send: false,
            auto_scroll_log: true,
            update_check_on_startup: true,
            repaint_interval_ms: 50,
            window_width: 1050.0,
            window_height: 500.0,
            byte_mode: false,
        }
    }
}

// Funciones de conveniencia (sin globals, reciben settings como parámetro)
impl AppSettings {
    pub fn get_repaint_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.repaint_interval_ms)
    }

    pub fn generate_log_filename(&self) -> String {
        use chrono::prelude::Local;
        use std::env;

        match env::current_dir() {
            Ok(dir) => {
                format!(
                    "{}/{}_{}.{}",
                    dir.display(),
                    self.log_file_default_name,
                    Local::now().format("%Y-%m-%d_%H-%M-%S"),
                    self.log_file_default_extension.trim_start_matches('.')
                )
            }
            Err(_) => {
                format!(
                    "{}_{}{}",
                    self.log_file_default_name,
                    Local::now().format("%Y-%m-%d_%H-%M-%S"),
                    self.log_file_default_extension
                )
            }
        }
    }
}
