use egui::{Align, Layout, Vec2};
use reqwest::Client;

pub async fn check_new_version() -> bool {
    let url = "https://api.github.com/repos/Opentronika/SerialGUI-rs/releases/latest";
    let client = Client::new();
    let request = client.get(url).header("User-Agent", "serialgui_rs").build();

    if let Ok(request) = request {
        if let Ok(response) = client.execute(request).await {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(tag) = json.get("tag_name").and_then(|v| v.as_str()) {
                    if tag != env!("VERGEN_GIT_DESCRIBE") {
                        eprint!("New version available: {}", tag);
                        return true;
                    }
                }
            }
        }
    }
    eprintln!("New version not found.");
    false
}

pub fn update_popup(ctx: &egui::Context, show_popup: &mut bool) {
    if *show_popup {
        egui::Window::new("New Version Available")
            .collapsible(false)
            .resizable(false)
            .fixed_size([350.0, 200.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("A new version of SerialGUI-rs is available!".to_string());
                    ui.add_space(15.0);

                    ui.separator();

                    ui.add_space(15.0);

                    ui.columns(2, |columns| {
                        columns[0].allocate_ui_with_layout(
                            Vec2::ZERO,
                            Layout::right_to_left(Align::Center),
                            |ui| {
                                if ui.button("Go to Download").clicked() {
                                    if let Err(e) = webbrowser::open(
                                "https://github.com/Opentronika/SerialGUI-rs/releases/latest",
                            ) {
                                eprintln!("Failed to open browser: {}", e);
                            }
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            },
                        );

                        columns[1].allocate_ui_with_layout(
                            Vec2::ZERO,
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                if ui.button("     Ignore     ").clicked() {
                                    *show_popup = false;
                                }
                            },
                        );
                    });
                });
            });
    }
}
