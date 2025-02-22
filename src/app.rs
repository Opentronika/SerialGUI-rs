use core::f32;
use std::io::Write;
use egui::Vec2;
use serialport::{available_ports, FlowControl, Parity, SerialPort, SerialPortType, StopBits};
use std::fs::File;
use std::time::Duration;
use guistrings::GuiStrings;

use crate::guistrings;

struct BaudRate {
    string_repr: &'static str,
    numeric_repr: u32,
}

const BAUD_RATES: [BaudRate; 3] = [
    BaudRate {
        string_repr: "9600",
        numeric_repr: 9600,
    },
    BaudRate {
        string_repr: "38400",
        numeric_repr: 38400,
    },
    BaudRate {
        string_repr: "115200",
        numeric_repr: 115200,
    },
];

const LOG_FILE_DEFAULT_NAME: &str = "LogFile";
const LOG_FILE_DEFAULT_EXTENTION: &str = ".txt";

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

struct PortSettings {
    port_name: String,
    baudrate: u32,
    flowcontrol: FlowControl,
    parity: Parity,
    stop_bits: StopBits,
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
    port: Option<Box<dyn SerialPort>>,
    buttonportstring: String,
    sendmessagestring: String,
    filelogpath: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    filelog: Option<File>,
    logfilebutton:String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            port_list: Vec::new(),
            logstring: "Starting app\n".to_owned(),
            port_settings: PortSettings {
                port_name: String::new(),
                baudrate: BAUD_RATES[2].numeric_repr,
                flowcontrol: FlowControl::None,
                parity: Parity::None,
                stop_bits: StopBits::One,
            },
            port: None,
            buttonportstring: String::from("Open port"),
            sendmessagestring: String::new(),
            filelogpath: String::from(LOG_FILE_DEFAULT_NAME)+LOG_FILE_DEFAULT_EXTENTION,
            filelog: None,
            logfilebutton: String::from(GuiStrings::STARTLOGFILE),
        }
    }
}

impl TemplateApp {
    const DEFAULT_PORT: &'static str = "No port";
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        let mut app: TemplateApp = Default::default();
        app.update_ports();
        if app.port_list.len() > 1 {
            app.port_settings.port_name = app.port_list[0].clone();
        } else {
            app.port_settings.port_name = String::from(TemplateApp::DEFAULT_PORT);
        }
        app
    }

    fn update_ports(&mut self) {
        match available_ports() {
            Ok(mut ports) => {
                // Let's output ports in a stable order to facilitate comparing the output from
                // different runs (on different platforms, with different features, ...).
                ports.sort_by_key(|i| i.port_name.clone());

                match ports.len() {
                    0 => println!("No ports found."),
                    1 => println!("Found 1 port:"),
                    n => println!("Found {} ports:", n),
                };

                for p in ports {
                    println!("    {}", p.port_name);
                    self.port_list.push(p.port_name);
                    match p.port_type {
                        SerialPortType::UsbPort(info) => {
                            println!("        Type: USB");
                            println!("        VID: {:04x}", info.vid);
                            println!("        PID: {:04x}", info.pid);
                            println!(
                                "        Serial Number: {}",
                                info.serial_number.as_ref().map_or("", String::as_str)
                            );
                            println!(
                                "        Manufacturer: {}",
                                info.manufacturer.as_ref().map_or("", String::as_str)
                            );
                            println!(
                                "        Product: {}",
                                info.product.as_ref().map_or("", String::as_str)
                            );
                        }
                        SerialPortType::BluetoothPort => {
                            println!("        Type: Bluetooth");
                        }
                        SerialPortType::PciPort => {
                            println!("        Type: PCI");
                        }
                        SerialPortType::Unknown => {
                            println!("        Type: Unknown");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                eprintln!("Error listing serial ports");
            }
        }
    }

    fn open_port(&mut self) -> bool {
        let mut state = false;
        let portopen = serialport::new(
            self.port_settings.port_name.clone(),
            self.port_settings.baudrate,
        )
        .flow_control(self.port_settings.flowcontrol)
        .parity(self.port_settings.parity)
        .stop_bits(self.port_settings.stop_bits)
        .timeout(Duration::from_millis(10))
        .open();
        match portopen {
            Ok(portopen) => {
                self.port = Some(portopen);
                state = true;
            }
            Err(e) => {
                eprintln!(
                    "Failed to open \"{}\". Error: {}",
                    self.port_settings.port_name, e
                );
            }
        }
        state
    }

    fn close_port(&mut self) {
        if self.port.is_some() {
            self.port = None;
        }
    }

    fn write_log(&mut self, message: &str) {
        eprintln!("{}", message);
        self.logstring += message;
        if let Some(ref mut file) = self.filelog {
            match file.write_all(message.as_bytes()) {
                Ok(_) => {},
                Err(e) => eprintln!("{:?}", e),
            }
        }
        // self.logstring += "\n";
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let sizeavailable = ui.available_size();
            let sizetext = Vec2::new(sizeavailable.x, sizeavailable.y * 0.85);
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::ScrollArea::vertical()
                .max_height(sizetext.y)
                .show(ui, |ui| {
                    ui.add_sized(
                        sizetext,
                        egui::TextEdit::multiline(&mut self.logstring)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY), // .layouter(&mut layouter),
                    );
                });

            ui.separator();

            ui.horizontal_wrapped(|ui| {
                if ui.button("Update ports").clicked() {
                    self.update_ports();
                    if self.port_list.len() > 1 {
                        self.port_settings.port_name = self.port_list[0].clone();
                    } else {
                        self.port_settings.port_name = String::from(TemplateApp::DEFAULT_PORT);
                    }
                }
                ui.label("Select port");
                egui::ComboBox::from_id_salt(500)
                    .selected_text(format!("{:?}", self.port_settings.port_name))
                    .show_ui(ui, |ui| {
                        // ui.selectable_value(&mut self.selected, Values::Dos, "First");
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
                        // ui.selectable_value(&mut self.selected, Values::Dos, "First");
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
                    if self.port.is_none() {
                        if self.open_port() {
                            self.buttonportstring = "Close port".to_string();
                        }
                    } else {
                        self.buttonportstring = "Open port".to_string();
                        self.close_port();
                    }
                }
            });
            ui.horizontal_wrapped(|ui| {
                ui.text_edit_singleline(&mut self.filelogpath.clone());
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
                                eprintln!("{}", e);
                            }
                        }
                    } else {
                                self.logfilebutton = String::from(GuiStrings::STARTLOGFILE);
                                self.filelog = None;
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
                    // self.write_log("send");
                    if let Some(ref mut port) = self.port {
                        match port.write_all(self.sendmessagestring.as_bytes()) {
                            Ok(_) => eprintln!("Write success"),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                }
            });
        });

        if let Some(ref mut port) = self.port {
            let size = port.bytes_to_read().unwrap_or(0);
            if size > 0 {
                let mut serial_buf: Vec<u8> = vec![0; size as usize];
                port.read_exact(&mut serial_buf).unwrap();
                let message = String::from_utf8(serial_buf[..size as usize].to_vec());
                self.write_log(message.unwrap_or(String::from("")).as_str());
            }
        }
    }
}
