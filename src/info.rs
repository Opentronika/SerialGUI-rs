use eframe::egui;

pub fn info_popup(ctx: &egui::Context, show_popup: &mut bool) {
    if *show_popup {
        egui::Window::new("SerialGUI-rs Information")
            .collapsible(false)
            .resizable(false)
            .fixed_size([350.0, 200.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.heading(env!("CARGO_PKG_NAME"));
                    ui.add_space(5.0);
                    ui.label(format!("Version: {}", env!("VERGEN_GIT_DESCRIBE")));
                    ui.label(format!("By: {}", env!("CARGO_PKG_AUTHORS")));
                    ui.add_space(15.0);

                    ui.separator();

                    ui.add_space(15.0);

                    if ui.button("OK").clicked() {
                        *show_popup = false;
                    }
                });
            });
    }
}
