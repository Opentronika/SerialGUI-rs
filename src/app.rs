use crate::communicationtrait::{CommunicationEvent, CommunicationManager};
use crate::gui::{ConnectionPanel, FileLogPanel, LogPanel, MenuBar, SendPanel};
use crate::serial_impl::SerialCommunication;
use std::sync::{mpsc, Arc, Mutex};

use crate::generalsettings::{
    LOG_FILE_DEFAULT_EXTENSION, LOG_FILE_DEFAULT_NAME, MAX_LOG_STRING_LENGTH,
};
use crate::info::info_popup;
use crate::update::{check_new_version, update_popup};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    connection_panel: ConnectionPanel,
    log_panel: LogPanel,
    #[serde(skip)]
    menu_bar: MenuBar,
    send_panel: SendPanel,
    file_log_panel: FileLogPanel,
    // Core state
    #[serde(skip)]
    serial_manager: Option<Box<dyn CommunicationManager>>,
    #[serde(skip)]
    serial_events_rx: Option<mpsc::Receiver<CommunicationEvent>>,
    // Popup states
    show_info_popup: bool,
    show_update_popup: Arc<Mutex<bool>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let default_filename = format!("{LOG_FILE_DEFAULT_NAME}{LOG_FILE_DEFAULT_EXTENSION}");
        Self {
            connection_panel: ConnectionPanel::new(),
            log_panel: LogPanel::new(MAX_LOG_STRING_LENGTH),
            menu_bar: MenuBar::new(),
            send_panel: SendPanel::new(),
            file_log_panel: FileLogPanel::new(default_filename),
            serial_manager: Some(Box::new(SerialCommunication::new())),
            serial_events_rx: None,
            show_info_popup: false,
            show_update_popup: Arc::new(Mutex::new(false)),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        // Initialize connection panel with available ports
        app.connection_panel.update_ports(&mut app.serial_manager);
        // Set up file logging with generated filename
        app.file_log_panel.file_path = app.file_log_panel.generate_filename();
        // Check for updates
        let context_clone = cc.egui_ctx.clone();
        check_new_version(&context_clone, app.show_update_popup.clone());
        app
    }

    fn handle_popups(&mut self, ctx: &egui::Context) {
        // Info popup
        if self.show_info_popup {
            info_popup(ctx, &mut self.show_info_popup);
        }

        // Update popup
        if let Ok(mut show_update) = self.show_update_popup.lock() {
            if *show_update {
                update_popup(ctx, &mut show_update);
            }
        }
    }

    fn handle_serial_events(&mut self, ctx: &egui::Context) {
        // Update connection button state
        self.connection_panel
            .update_button_text(&self.serial_manager);

        // Process serial events
        let mut events = Vec::new();
        if let Some(ref rx) = self.serial_events_rx {
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }
        }
        for event in events {
            match event {
                CommunicationEvent::DataReceived(data) => {
                    let message = String::from_utf8_lossy(&data);
                    self.write_log(&message);
                    ctx.request_repaint();
                }
                CommunicationEvent::ConnectionClosed => {
                    eprintln!("Connection closed.");
                    self.connection_panel.button_text = "Open port".to_string();
                }
                CommunicationEvent::Error(err) => {
                    eprintln!("Error: {err}");
                }
            }
        }
    }

    fn write_log(&mut self, message: &str) {
        // Write to log panel
        self.log_panel.append_log(message);

        // Write to file if logging is active
        self.file_log_panel.write_to_file(message);
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle popups
        self.handle_popups(ctx);

        // Menu bar
        self.menu_bar.show(ctx, || {
            self.log_panel.clear();
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            // Log panel (takes most of the space)
            self.log_panel.show(ui, available_size);

            ui.separator();

            // Connection panel
            self.connection_panel
                .show(ui, &mut self.serial_manager, &mut self.serial_events_rx);

            // File log panel
            self.file_log_panel.show(ui);

            // Send panel
            self.send_panel
                .show(ui, &mut self.serial_manager, available_size);
        });

        // Handle serial events and connection state
        self.handle_serial_events(ctx);

        // Request periodic repaints
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}
