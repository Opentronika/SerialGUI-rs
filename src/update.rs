use std::{
    sync::{Arc, Mutex},
    thread,
};

use egui::{Align, Context, Layout, Vec2};
use reqwest::Client;

pub async fn request_latest_version() -> Result<Option<String>, reqwest::Error> {
    let url = "https://api.github.com/repos/Opentronika/SerialGUI-rs/releases/latest";
    let client = Client::new();
    let request = client
        .get(url)
        .header("User-Agent", "serialgui_rs")
        .build()?;

    let response = client.execute(request).await?;
    let json = response.json::<serde_json::Value>().await?;
    Ok(json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

pub fn check_new_version(ctx: &Context, update_available: Arc<Mutex<bool>>) {
    let ctx_clone = ctx.clone();
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime in background thread");

        let latest_version = rt.block_on(async { request_latest_version().await });

        if let Ok(Some(tag)) = latest_version {
            if tag != env!("VERGEN_GIT_DESCRIBE") {
                eprintln!(
                    "New version available: {tag} != {}",
                    env!("VERGEN_GIT_DESCRIBE")
                );
                if let Ok(mut flag) = update_available.lock() {
                    *flag = true;
                }
            }
        }
        ctx_clone.request_repaint();
    });
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
                                 eprintln!("Failed to send update check result: {e}");
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
