use eframe::glow::AND;
use serialport::{available_ports, SerialPort, SerialPortInfo, SerialPortType};
use std::time::Duration;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum STATES {
    Close,
    Open,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    logstring: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    selected: String,
    port_list: Vec<String>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    port: Option<Box<dyn SerialPort>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            selected: String::new(),
            port_list: Vec::new(),
            logstring: "Starting app\n".to_owned(),
            port: None,
        }
    }
}

impl TemplateApp {
    const DEFAULT_PORT: &'static str = "No port";
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            app.selected = app.port_list[0].clone();
        } else {
            app.selected = String::from(TemplateApp::DEFAULT_PORT);
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
                            #[cfg(feature = "usbportinfo-interface")]
                            println!(
                                "        Interface: {}",
                                info.interface
                                    .as_ref()
                                    .map_or("".to_string(), |x| format!("{:02x}", *x))
                            );
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

    fn write_log(&mut self, message: &str) {
        eprintln!("{}", message);
        self.logstring += message;
        self.logstring += "\n";
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
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
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.logstring)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY), // .layouter(&mut layouter),
                );
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Update ports").clicked() {
                    self.update_ports();
                    if self.port_list.len() > 1 {
                        self.selected = self.port_list[0].clone();
                    } else {
                        self.selected = String::from(TemplateApp::DEFAULT_PORT);
                    }
                }
                egui::ComboBox::from_label("Select port")
                    .selected_text(format!("{:?}", self.selected))
                    .show_ui(ui, |ui| {
                        // ui.selectable_value(&mut self.selected, Values::Dos, "First");
                        for port_name in &self.port_list {
                            ui.selectable_value(&mut self.selected, port_name.clone(), port_name);
                        }
                    });
                if ui.button("Open port").clicked() {
                    if self.port.is_none() {
                        let portopen = serialport::new(self.selected.clone(), 115200)
                            .timeout(Duration::from_millis(10))
                            .open();
                        match portopen {
                            Ok(portopen) => {
                                self.port = Some(portopen);
                            }
                            Err(e) => {
                                eprintln!("Failed to open \"{}\". Error: {}", self.selected, e);
                            }
                        }
                    }
                }
            })
        });

        if let Some(ref mut port) = self.port {
            let size = port.bytes_to_read().unwrap_or(0);
            if size > 0 {
                let mut serial_buf: Vec<u8> = vec![0; 1000];
                port.read(&mut serial_buf).unwrap();
                let message = String::from_utf8(serial_buf[..size as usize].to_vec());
                self.write_log(message.unwrap_or(String::from("")).as_str());
            }
        }
    }
}
