#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/SerialGUI-rs-logo.png")[..],
                )
                .expect("Failed to load icon"),
            )
            .with_inner_size([1050.0, 500.0])
            .with_min_inner_size([1050.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SerialGUI-Rs",
        native_options,
        Box::new(|cc| Ok(Box::new(serialgui_rs::TemplateApp::new(cc)))),
    )
}
