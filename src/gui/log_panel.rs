use egui::Vec2;

pub struct LogPanel {
    pub content: String,
    should_scroll_to_bottom: bool,
}

impl LogPanel {
    pub fn new(_max_length: usize) -> Self {
        Self {
            content: "Starting app\n".to_string(),
            should_scroll_to_bottom: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, available_size: Vec2, autoscroll: bool) {
        let text_size = Vec2::new(available_size.x, available_size.y * 0.85);

        let scroll_area = egui::ScrollArea::vertical()
            .max_height(text_size.y)
            .stick_to_bottom(autoscroll);

        scroll_area.show(ui, |ui| {
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

        // If we need to scroll to bottom and autoscroll is enabled
        if self.should_scroll_to_bottom && autoscroll {
            // Use the correct method to scroll to bottom
            ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
            self.should_scroll_to_bottom = false;
        }
    }

    pub fn append_log(&mut self, message: &str, max_length: usize) {
        eprintln!("{message}");
        self.content += message;

        if self.content.len() > max_length {
            let excess_len = self.content.len() - max_length;
            self.content.drain(0..excess_len);
        }

        // Mark that we need to scroll to bottom
        self.should_scroll_to_bottom = true;
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
            should_scroll_to_bottom: false,
        })
    }
}

impl Default for LogPanel {
    fn default() -> Self {
        Self {
            content: "Starting app\n".to_string(),
            should_scroll_to_bottom: false,
        }
    }
}
