use crate::communicationtrait::{CommunicationEvent, CommunicationManager};
use crate::generalsettings::AppSettings;
use crate::gui::{ConnectionPanel, FileLogPanel, MenuBar, RxPanel, SendPanel};
use crate::serial_impl::SerialCommunication;
use std::sync::{mpsc, Arc, Mutex};

use crate::info::info_popup;
use crate::update::{check_new_version, update_popup};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    pub settings: AppSettings,

    #[serde(skip)]
    connection_panel: ConnectionPanel,
    rx_panel: RxPanel,
    #[serde(skip)]
    menu_bar: MenuBar,
    send_panel: SendPanel,
    file_log_panel: FileLogPanel,

    // Core state
    #[serde(skip)]
    serial_manager: Option<Box<dyn CommunicationManager>>,
    #[serde(skip)]
    serial_events_rx: Option<mpsc::Receiver<CommunicationEvent>>,

    #[serde(skip)]
    show_info_popup: bool,
    #[serde(skip)]
    show_update_popup: Arc<Mutex<bool>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let settings = AppSettings::default();
        let default_filename = format!(
            "{}{}",
            settings.log_file_default_name, settings.log_file_default_extension
        );

        Self {
            settings: settings.clone(),
            connection_panel: ConnectionPanel::new(),
            rx_panel: RxPanel::new(settings.max_log_string_length),
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
        let mut app: TemplateApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        app.connection_panel = ConnectionPanel::new();
        app.menu_bar = MenuBar::new();
        app.serial_manager = Some(Box::new(SerialCommunication::new()));
        app.serial_events_rx = None;
        app.show_info_popup = false;
        app.show_update_popup = Arc::new(Mutex::new(false));
        app.file_log_panel.file_path = app.file_log_panel.generate_filename(&app.settings);
        // Initialize connection panel with available ports
        app.connection_panel.update_ports(&mut app.serial_manager);

        // Set up file logging with generated filename
        app.file_log_panel.file_path = app.settings.generate_log_filename();

        // Check for updates if enabled in settings
        if app.settings.update_check_on_startup {
            let context_clone = cc.egui_ctx.clone();
            check_new_version(&context_clone, app.show_update_popup.clone());
        }

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
        self.connection_panel
            .update_button_text(&self.serial_manager);

        let mut events = Vec::new();
        if let Some(ref rx) = self.serial_events_rx {
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }
        }

        for event in events {
            match event {
                CommunicationEvent::DataReceived(data) => {
                    let message = if self.settings.byte_mode {
                        // Convert bytes to hex string representation with packet separator
                        let hex_string = data
                            .iter()
                            .map(|byte| format!("{byte:02X}"))
                            .collect::<Vec<String>>()
                            .join(" ");
                        format!("{hex_string} ")
                    } else {
                        String::from_utf8_lossy(&data).to_string()
                    };
                    self.write_log(&message);
                    self.file_log_panel.write_to_file(&data);
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
        self.rx_panel
            .append_log(message, self.settings.max_log_string_length);
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_popups(ctx);

        self.menu_bar.show(
            ctx,
            || {
                self.rx_panel.clear();
            },
            || {
                self.show_info_popup = true;
            },
            &mut self.settings,
        );

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            self.rx_panel
                .show(ui, available_size, self.settings.auto_scroll_log);
            ui.separator();

            self.connection_panel
                .show(ui, &mut self.serial_manager, &mut self.serial_events_rx);

            self.file_log_panel.show(ui, &self.settings);

            self.send_panel
                .show(ui, &mut self.serial_manager, available_size, &self.settings);
        });

        self.handle_serial_events(ctx);
        ctx.request_repaint_after(self.settings.get_repaint_interval());
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
