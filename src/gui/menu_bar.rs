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
        ui: &mut egui::Ui,
        clear_callback: impl FnOnce(),
        show_info_callback: impl FnOnce(),
        settings: &mut AppSettings,
    ) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Info").clicked() {
                        show_info_callback();
                        ui.close();
                    }
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
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

                    // Ensure at least one panel is always enabled
                    ui.add_enabled(
                        settings.show_text_panel,
                        egui::Checkbox::new(&mut settings.show_chart_panel, "Show chart panel"),
                    );
                    ui.add_enabled(
                        settings.show_chart_panel,
                        egui::Checkbox::new(&mut settings.show_text_panel, "Show text panel"),
                    );
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

#[cfg(test)]
mod tests {
    use egui::FontId;

    #[test]
    fn theme_icons_are_available_in_the_default_fonts() {
        // egui 0.37's native theme selector is icon-only, and its emoji fonts
        // are opt-in. Keep the `monochrome_emoji_fonts` feature in Cargo.toml
        // from regressing, otherwise the selector renders as missing glyphs.
        let ctx = egui::Context::default();

        ctx.run_ui(Default::default(), |ui| {
            ui.ctx().fonts_mut(|fonts| {
                let font_id = FontId::proportional(14.0);

                for icon in ['☀', '🌙', '💻'] {
                    assert!(
                        fonts.has_glyph(&font_id, icon),
                        "missing glyph for theme icon {icon:?}"
                    );
                }
            });
        })
        .drop_without_applying_deltas();
    }
}
