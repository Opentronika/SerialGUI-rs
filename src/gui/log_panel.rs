use egui::Vec2;

pub struct LogPanel {
    pub content: String,
    max_length: usize,
}

impl LogPanel {
    pub fn new(max_length: usize) -> Self {
        Self {
            content: "Starting app\n".to_string(),
            max_length,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, available_size: Vec2) {
        let text_size = Vec2::new(available_size.x, available_size.y * 0.85);

        egui::ScrollArea::vertical()
            .max_height(text_size.y)
            .show(ui, |ui| {
                ui.add_sized(
                    text_size,
                    egui::TextEdit::multiline(&mut self.content)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                );
            });
    }

    pub fn append_log(&mut self, message: &str) {
        eprintln!("{message}");
        self.content += message;
        if self.content.len() > self.max_length {
            let excess_len = self.content.len() - self.max_length;
            self.content.drain(0..excess_len);
        }
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }
}

impl serde::Serialize for LogPanel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.content.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for LogPanel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = String::deserialize(deserializer)?;
        Ok(Self {
            content,
            max_length: crate::generalsettings::MAX_LOG_STRING_LENGTH,
        })
    }
}

impl Default for LogPanel {
    fn default() -> Self {
        Self::new(crate::generalsettings::MAX_LOG_STRING_LENGTH)
    }
}
