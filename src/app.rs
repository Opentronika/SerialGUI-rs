use crate::communicationtrait::{CommunicationEvent, CommunicationManager};
use crate::serial_impl::SerialCommunication;
use chrono::prelude::Local;
use core::f32;
use egui::Vec2;
use guistrings::GuiStrings;
use serialport::{FlowControl, Parity, StopBits};
use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::{mpsc, Arc, Mutex};

use crate::guistrings;
use crate::serial_impl::{PortSettings, BAUD_RATES};

use crate::info::info_popup;
use crate::update::{check_new_version, update_popup};

use crate::generalsettings::{
    LOG_FILE_DEFAULT_EXTENSION, LOG_FILE_DEFAULT_NAME, MAX_LOG_STRING_LENGTH,
};

fn flow_control_iter() -> impl Iterator<Item = FlowControl> {
    [
        FlowControl::None,
        FlowControl::Software,
        FlowControl::Hardware,
    ]
    .iter()
    .cloned()
}

fn parity_iter() -> impl Iterator<Item = Parity> {
    [Parity::None, Parity::Even, Parity::Odd].iter().cloned()
}

fn stop_bits_iter() -> impl Iterator<Item = StopBits> {
    [StopBits::One, StopBits::Two].iter().cloned()
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    logstring: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    port_settings: PortSettings,
    port_list: Vec<String>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    buttonportstring: String,
    sendmessagestring: String,
    filelogpath: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    filelog: Option<File>,
    logfilebutton: String,
    show_info_popup: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    serial_manager: Option<Box<dyn CommunicationManager>>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    serial_events_rx: Option<mpsc::Receiver<CommunicationEvent>>,
    show_update_popup: Arc<Mutex<bool>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            port_list: Vec::new(),
            logstring: "Starting app\n".to_owned(),
            port_settings: PortSettings::default(),
            buttonportstring: String::from("Open port"),
            sendmessagestring: String::new(),
            filelogpath: String::from(LOG_FILE_DEFAULT_NAME) + LOG_FILE_DEFAULT_EXTENSION,
            filelog: None,
            logfilebutton: String::from(GuiStrings::STARTLOGFILE),
            show_info_popup: false,
            show_update_popup: Arc::new(Mutex::new(false)),
            serial_manager: Some(Box::new(SerialCommunication::new())),
            serial_events_rx: None,
        }
    }
}

impl TemplateApp {
    const DEFAULT_PORT: &'static str = "No port";
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: TemplateApp = Default::default();

        app.filelogpath = app.generate_filename();
        let context_clone = _cc.egui_ctx.clone();
        check_new_version(&context_clone, app.show_update_popup.clone());
        app.update_ports();
        if !app.port_list.is_empty() {
            app.port_settings.port_name = app.port_list[0].clone();
        } else {
            app.port_settings.port_name = String::from(TemplateApp::DEFAULT_PORT);
        }
        app
    }

    fn generate_filename(&self) -> String {
        let mut filename = String::from("");
        match env::current_dir() {
            Ok(_dir) => {
                eprintln!("{}", _dir.display());
                filename += _dir.display().to_string().as_str();
                filename += "/";
                filename += LOG_FILE_DEFAULT_NAME;
                filename += "_";
                filename += Local::now()
                    .format("%Y-%m-%d_%H-%M-%S")
                    .to_string()
                    .as_str();
                filename += LOG_FILE_DEFAULT_EXTENSION;
            }
            Err(_) => {
                filename = String::from(LOG_FILE_DEFAULT_NAME) + LOG_FILE_DEFAULT_EXTENSION;
            }
        }
        filename
    }

    fn update_ports(&mut self) {
        if let Some(ref mut manager) = self.serial_manager {
            self.port_list = manager.get_available_connections();
            if !self.port_list.is_empty() {
                self.port_settings.port_name = self.port_list[0].clone();
            } else {
                self.port_settings.port_name = String::from(TemplateApp::DEFAULT_PORT);
            }
        } else {
            eprintln!("Serial manager is not initialized.");
        }
    }

    fn write_log(&mut self, message: &str) {
        eprintln!("{message}");
        self.logstring += message;
        if self.logstring.len() > MAX_LOG_STRING_LENGTH {
            let excess_len = self.logstring.len() - MAX_LOG_STRING_LENGTH;
            self.logstring.drain(0..excess_len);
        }
        if let Some(ref mut file) = self.filelog {
            match file.write_all(message.as_bytes()) {
                Ok(_) => {}
                Err(e) => eprintln!("{e:?}"),
            }
        }
        // self.logstring += "\n";
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_info_popup {
            info_popup(ctx, &mut self.show_info_popup);
        }
        if let Ok(mut show_update) = self.show_update_popup.lock() {
            if *show_update {
                update_popup(ctx, &mut show_update);
            }
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Info").clicked() {
                        self.show_info_popup = true;
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
                if ui.button("Clear output").clicked() {
                    self.logstring.clear();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let sizeavailable = ui.available_size();
            let sizetext = Vec2::new(sizeavailable.x, sizeavailable.y * 0.85);
            egui::ScrollArea::vertical()
                .max_height(sizetext.y)
                .show(ui, |ui| {
                    ui.add_sized(
                        sizetext,
                        egui::TextEdit::multiline(&mut self.logstring)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY),
                    );
                });

            ui.separator();

            ui.horizontal_wrapped(|ui| {
                if ui.button("Update ports").clicked() {
                    self.update_ports();
                    if !self.port_list.is_empty() {
                        self.port_settings.port_name = self.port_list[0].clone();
                    } else {
                        self.port_settings.port_name = String::from(TemplateApp::DEFAULT_PORT);
                    }
                }
                ui.label("Select port");
                egui::ComboBox::from_id_salt(500)
                    .selected_text(format!("{:?}", self.port_settings.port_name))
                    .show_ui(ui, |ui| {
                        for port_name in &self.port_list {
                            ui.selectable_value(
                                &mut self.port_settings.port_name,
                                port_name.clone(),
                                port_name,
                            );
                        }
                    });
                ui.label("Baud rate");
                egui::ComboBox::from_id_salt(501)
                    .selected_text(format!("{:?}", self.port_settings.baudrate))
                    .show_ui(ui, |ui| {
                        for baudrate in &BAUD_RATES {
                            ui.selectable_value(
                                &mut self.port_settings.baudrate,
                                baudrate.numeric_repr,
                                baudrate.string_repr,
                            );
                        }
                    });
                ui.label("Flow control");
                egui::ComboBox::from_id_salt(502)
                    .selected_text(self.port_settings.flowcontrol.to_string())
                    .show_ui(ui, |ui| {
                        for flow in flow_control_iter() {
                            ui.selectable_value(
                                &mut self.port_settings.flowcontrol,
                                flow,
                                flow.to_string(),
                            );
                        }
                    });
                ui.label("Parity");
                egui::ComboBox::from_id_salt(503)
                    .selected_text(self.port_settings.parity.to_string())
                    .show_ui(ui, |ui| {
                        for parity in parity_iter() {
                            ui.selectable_value(
                                &mut self.port_settings.parity,
                                parity,
                                parity.to_string(),
                            );
                        }
                    });
                ui.label("Stop bits");
                egui::ComboBox::from_id_salt(504)
                    .selected_text(self.port_settings.stop_bits.to_string())
                    .show_ui(ui, |ui| {
                        for stop_bit in stop_bits_iter() {
                            ui.selectable_value(
                                &mut self.port_settings.stop_bits,
                                stop_bit,
                                stop_bit.to_string(),
                            );
                        }
                    });

                if ui.button(self.buttonportstring.clone()).clicked() {
                    if let Some(ref mut manager) = self.serial_manager {
                        // let mut port_state = self.port_state.lock().unwrap();
                        if manager.is_running() {
                            if let Err(e) = manager.stop() {
                                eprintln!("Error stopping port: {e}");
                            }
                            self.buttonportstring = "Open port".to_string();
                        } else {
                            if let Err(e) = manager.update_settings(&self.port_settings) {
                                eprintln!("Error updating port settings: {e}");
                                return;
                            }
                            let (tx, rx) = std::sync::mpsc::channel();
                            if let Err(e) = manager.start(tx) {
                                eprintln!("Error starting port: {e}");
                            } else {
                                self.buttonportstring = "Close port".to_string();
                                self.serial_events_rx = Some(rx);
                            }
                        }
                    } else {
                        eprintln!("Serial manager is not initialized.");
                    }
                }
            });
            ui.horizontal_wrapped(|ui| {
                ui.add_sized(
                    Vec2::new(500.0, 20.0),
                    egui::TextEdit::singleline(&mut self.filelogpath.clone()),
                );
                if ui.button(self.logfilebutton.clone()).clicked() {
                    if self.filelog.is_none() {
                        let openfile = File::create(self.filelogpath.clone());
                        match openfile {
                            Ok(file) => {
                                self.filelog = Some(file);
                                self.logfilebutton = String::from(GuiStrings::STOPLOGFILE);
                            }
                            Err(e) => {
                                self.write_log("Open file failed \n");
                                eprintln!("{e}");
                            }
                        }
                    } else {
                        self.logfilebutton = String::from(GuiStrings::STARTLOGFILE);
                        self.filelog = None;
                        self.filelogpath = self.generate_filename();
                    }
                }
            });
            ui.horizontal(|ui| {
                let sizesend = Vec2::new(sizeavailable.x * 0.9, 20.0);
                ui.add_sized(
                    sizesend,
                    egui::TextEdit::singleline(&mut self.sendmessagestring),
                );
                if ui.button("Send").clicked() {
                    if let Some(ref mut manager) = self.serial_manager {
                        if manager.is_running() {
                            if let Err(e) =
                                manager.send_data(self.sendmessagestring.as_bytes().to_vec())
                            {
                                eprintln!("Error sending data: {e}");
                            }
                        } else {
                            eprintln!("Port is not open, cannot send data.");
                        }
                    }
                    eprintln!("Serial manager is not initialized.");
                }
            });
        });

        if let Some(ref mut manager) = self.serial_manager {
            if !manager.is_running() {
                self.buttonportstring = "Open port".to_string();
            }
        }

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
                    self.buttonportstring = "Open port".to_string();
                }
                CommunicationEvent::Error(err) => {
                    eprintln!("Error: {err}");
                }
            }
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}
