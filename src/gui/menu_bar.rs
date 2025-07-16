use super::settings_panel::SettingsPanel;
use crate::generalsettings::AppSettings;

pub struct MenuBar {
    settings_panel: SettingsPanel,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            settings_panel: SettingsPanel::new(),
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        clear_callback: impl FnOnce(),
        show_info_callback: impl FnOnce(),
        settings: &mut AppSettings,
    ) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Info").clicked() {
                        show_info_callback();
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Settings", |ui| {
                    self.settings_panel.show(ui, settings);
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                    if ui.button("Clear output").clicked() {
                        clear_callback();
                    }
                    ui.checkbox(&mut settings.byte_mode, "Byte mode");
                });
            });
        });
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}
