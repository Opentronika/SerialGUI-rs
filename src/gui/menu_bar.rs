pub struct MenuBar {
    pub show_info: bool,
}

impl MenuBar {
    pub fn new() -> Self {
        Self { show_info: false }
    }

    pub fn show(&mut self, ctx: &egui::Context, clear_callback: impl FnOnce()) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Info").clicked() {
                        self.show_info = true;
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
                if ui.button("Clear output").clicked() {
                    clear_callback();
                }
            });
        });
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}
