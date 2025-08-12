use crate::generalsettings::AppSettings;

pub struct SettingsPanel {}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, settings: &mut AppSettings) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Max log length:");
                let mut max_log_length_str = settings.max_log_string_length.to_string();
                if ui.text_edit_singleline(&mut max_log_length_str).changed() {
                    if let Ok(val) = max_log_length_str.parse::<usize>() {
                        settings.max_log_string_length = val;
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Default file name:");
                ui.text_edit_singleline(&mut settings.log_file_default_name);
            });

            ui.checkbox(
                &mut settings.clear_message_after_send,
                "Clear message after send",
            );
            ui.checkbox(&mut settings.auto_scroll_log, "Auto-scroll log");
            ui.checkbox(
                &mut settings.update_check_on_startup,
                "Check updates on startup",
            );

            ui.horizontal(|ui| {
                ui.label("Repaint interval (ms):");
                ui.add(egui::DragValue::new(&mut settings.repaint_interval_ms).range(16..=1000));
            });

            ui.checkbox(&mut settings.byte_mode, "Byte mode");
            ui.checkbox(&mut settings.show_chart_panel, "Show chart panel");
            ui.checkbox(&mut settings.show_text_panel, "Show text panel");
        });
    }
}
