use crate::communicationtrait::CommunicationManager;
use egui::Vec2;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SendPanel {
    pub message: String,
}

impl SendPanel {
    pub fn new() -> Self {
        Self {
            message: String::new(),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        manager: &mut Option<Box<dyn CommunicationManager>>,
        available_size: Vec2,
    ) {
        ui.horizontal(|ui| {
            let text_size = Vec2::new(available_size.x * 0.9, 20.0);
            ui.add_sized(text_size, egui::TextEdit::singleline(&mut self.message));

            if ui.button("Send").clicked() {
                self.send_message(manager);
            }
        });
    }

    fn send_message(&mut self, manager: &mut Option<Box<dyn CommunicationManager>>) {
        if let Some(ref mut manager) = manager {
            if manager.is_running() {
                if let Err(e) = manager.send_data(self.message.as_bytes().to_vec()) {
                    eprintln!("Error sending data: {e}");
                }
            } else {
                eprintln!("Port is not open, cannot send data.");
            }
        } else {
            eprintln!("Serial manager is not initialized.");
        }
    }
}

impl Default for SendPanel {
    fn default() -> Self {
        Self::new()
    }
}
