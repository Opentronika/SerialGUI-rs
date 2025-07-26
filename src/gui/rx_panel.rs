use egui::Vec2;
use std::collections::VecDeque;

#[derive(Default)]
pub struct RxPanel {
    pub content: VecDeque<char>,
    should_scroll_to_bottom: bool,
}

impl RxPanel {
    pub fn new(_max_length: usize) -> Self {
        Self {
            content: VecDeque::new(),
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
                egui::TextEdit::multiline(&mut self.content.iter().collect::<String>())
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
        for ch in message.chars() {
            if self.content.len() == max_length {
                self.content.pop_front(); // Elimina el carácter más viejo
            }
            self.content.push_back(ch); // Agrega el nuevo carácter
        }
        // Mark that we need to scroll to bottom
        self.should_scroll_to_bottom = true;
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }
}

impl serde::Serialize for RxPanel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.content.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for RxPanel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = VecDeque::deserialize(deserializer)?;
        Ok(Self {
            content,
            should_scroll_to_bottom: false,
        })
    }
}
