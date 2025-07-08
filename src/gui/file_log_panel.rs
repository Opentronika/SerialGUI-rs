use crate::guistrings::GuiStrings;
use egui::Vec2;
use std::fs::File;
use std::io::Write;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FileLogPanel {
    pub file_path: String,
    pub button_text: String,
    #[serde(skip)]
    pub log_file: Option<File>,
}

impl FileLogPanel {
    pub fn new(default_path: String) -> Self {
        Self {
            file_path: default_path,
            button_text: GuiStrings::STARTLOGFILE.to_string(),
            log_file: None,
        }
    }

    pub fn generate_filename(&self) -> String {
        use crate::generalsettings::{LOG_FILE_DEFAULT_EXTENSION, LOG_FILE_DEFAULT_NAME};
        use chrono::prelude::Local;
        use std::env;

        match env::current_dir() {
            Ok(dir) => {
                format!(
                    "{}/{}_{}.{}",
                    dir.display(),
                    LOG_FILE_DEFAULT_NAME,
                    Local::now().format("%Y-%m-%d_%H-%M-%S"),
                    LOG_FILE_DEFAULT_EXTENSION.trim_start_matches('.')
                )
            }
            Err(_) => {
                format!(
                    "{}_{}{}",
                    LOG_FILE_DEFAULT_NAME,
                    Local::now().format("%Y-%m-%d_%H-%M-%S"),
                    LOG_FILE_DEFAULT_EXTENSION
                )
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.add_sized(
                Vec2::new(500.0, 20.0),
                egui::TextEdit::singleline(&mut self.file_path),
            );

            if ui.button(self.button_text.clone()).clicked() {
                self.toggle_file_logging();
            }
        });
    }

    fn toggle_file_logging(&mut self) {
        if self.log_file.is_none() {
            // Start logging
            match File::create(&self.file_path) {
                Ok(file) => {
                    self.log_file = Some(file);
                    self.button_text = GuiStrings::STOPLOGFILE.to_string();
                }
                Err(e) => {
                    eprintln!("Failed to create log file: {e}");
                }
            }
        } else {
            // Stop logging
            self.log_file = None;
            self.button_text = GuiStrings::STARTLOGFILE.to_string();
            // Generate new filename for next session
            self.file_path = self.generate_filename();
        }
    }

    pub fn write_to_file(&mut self, message: &str) {
        if let Some(ref mut file) = self.log_file {
            if let Err(e) = file.write_all(message.as_bytes()) {
                eprintln!("Error writing to log file: {e:?}");
            }
        }
    }

    pub fn _is_logging(&self) -> bool {
        self.log_file.is_some()
    }
}

impl Default for FileLogPanel {
    fn default() -> Self {
        use crate::generalsettings::{LOG_FILE_DEFAULT_EXTENSION, LOG_FILE_DEFAULT_NAME};
        let default_filename = format!("{LOG_FILE_DEFAULT_NAME}{LOG_FILE_DEFAULT_EXTENSION}");
        Self::new(default_filename)
    }
}
