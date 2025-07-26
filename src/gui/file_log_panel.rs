use crate::generalsettings::AppSettings;
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

    pub fn generate_filename(&self, settings: &AppSettings) -> String {
        use chrono::prelude::Local;
        use std::env;

        match env::current_dir() {
            Ok(dir) => {
                format!(
                    "{}/{}_{}.{}",
                    dir.display(),
                    settings.log_file_default_name,
                    Local::now().format("%Y-%m-%d_%H-%M-%S"),
                    settings.log_file_default_extension.trim_start_matches('.')
                )
            }
            Err(_) => {
                format!(
                    "{}_{}{}",
                    settings.log_file_default_name,
                    Local::now().format("%Y-%m-%d_%H-%M-%S"),
                    settings.log_file_default_extension
                )
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, settings: &AppSettings) {
        ui.horizontal_wrapped(|ui| {
            ui.add_sized(
                Vec2::new(500.0, 20.0),
                egui::TextEdit::singleline(&mut self.file_path),
            );

            if ui.button(self.button_text.clone()).clicked() {
                self.toggle_file_logging(settings);
            }
        });
    }

    fn toggle_file_logging(&mut self, settings: &AppSettings) {
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
            // Generate new filename for next session using current settings
            self.file_path = self.generate_filename(settings);
        }
    }

    pub fn write_to_file(&mut self, message: &[u8]) {
        if let Some(ref mut file) = self.log_file {
            if let Err(e) = file.write_all(message) {
                eprintln!("Error writing to log file: {e:?}");
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_logging(&self) -> bool {
        self.log_file.is_some()
    }
}

impl Default for FileLogPanel {
    fn default() -> Self {
        let default_filename = "LogFile.txt".to_string();
        Self::new(default_filename)
    }
}
