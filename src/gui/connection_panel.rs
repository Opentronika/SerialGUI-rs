use crate::communicationtrait::{CommunicationEvent, CommunicationManager};
use crate::serial_impl::{PortSettings, BAUD_RATES};
use serialport::{FlowControl, Parity, StopBits};
use std::sync::mpsc;

pub struct ConnectionPanel {
    pub port_settings: PortSettings,
    pub port_list: Vec<String>,
    pub button_text: String,
}

impl ConnectionPanel {
    pub fn new() -> Self {
        Self {
            port_settings: PortSettings::default(),
            port_list: Vec::new(),
            button_text: "Open port".to_string(),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        manager: &mut Option<Box<dyn CommunicationManager>>,
        events_rx: &mut Option<mpsc::Receiver<CommunicationEvent>>,
    ) {
        ui.horizontal_wrapped(|ui| {
            // Update ports button
            if ui.button("Update ports").clicked() {
                self.update_ports(manager);
            }

            // Port selection
            ui.label("Select port");
            egui::ComboBox::from_id_salt("port_combo")
                .selected_text(&self.port_settings.port_name)
                .show_ui(ui, |ui| {
                    for port_name in &self.port_list {
                        ui.selectable_value(
                            &mut self.port_settings.port_name,
                            port_name.clone(),
                            port_name,
                        );
                    }
                });

            // Baud rate
            ui.label("Baud rate");
            egui::ComboBox::from_id_salt("baud_combo")
                .selected_text(format!("{}", self.port_settings.baudrate))
                .show_ui(ui, |ui| {
                    for baudrate in &BAUD_RATES {
                        ui.selectable_value(
                            &mut self.port_settings.baudrate,
                            baudrate.numeric_repr,
                            baudrate.string_repr,
                        );
                    }
                });

            // Flow control
            ui.label("Flow control");
            egui::ComboBox::from_id_salt("flow_combo")
                .selected_text(self.port_settings.flowcontrol.to_string())
                .show_ui(ui, |ui| {
                    for flow in self.flow_control_iter() {
                        ui.selectable_value(
                            &mut self.port_settings.flowcontrol,
                            flow,
                            flow.to_string(),
                        );
                    }
                });

            // Parity
            ui.label("Parity");
            egui::ComboBox::from_id_salt("parity_combo")
                .selected_text(self.port_settings.parity.to_string())
                .show_ui(ui, |ui| {
                    for parity in self.parity_iter() {
                        ui.selectable_value(
                            &mut self.port_settings.parity,
                            parity,
                            parity.to_string(),
                        );
                    }
                });

            // Stop bits
            ui.label("Stop bits");
            egui::ComboBox::from_id_salt("stop_bits_combo")
                .selected_text(self.port_settings.stop_bits.to_string())
                .show_ui(ui, |ui| {
                    for stop_bit in self.stop_bits_iter() {
                        ui.selectable_value(
                            &mut self.port_settings.stop_bits,
                            stop_bit,
                            stop_bit.to_string(),
                        );
                    }
                });

            // Connect/Disconnect button
            if ui.button(self.button_text.clone()).clicked() {
                self.handle_connection_button(manager, events_rx);
            }
        });
    }

    pub fn update_ports(&mut self, manager: &mut Option<Box<dyn CommunicationManager>>) {
        if let Some(ref mut manager) = manager {
            self.port_list = manager.get_available_connections();
            if !self.port_list.is_empty() {
                self.port_settings.port_name = self.port_list[0].clone();
            } else {
                self.port_settings.port_name = "No port".to_string();
            }
        } else {
            eprintln!("Serial manager is not initialized.");
        }
    }

    fn handle_connection_button(
        &mut self,
        manager: &mut Option<Box<dyn CommunicationManager>>,
        events_rx: &mut Option<mpsc::Receiver<CommunicationEvent>>,
    ) {
        if let Some(ref mut manager) = manager {
            if manager.is_running() {
                if let Err(e) = manager.stop() {
                    eprintln!("Error stopping port: {e}");
                }
                self.button_text = "Open port".to_string();
            } else {
                if let Err(e) = manager.update_settings(&self.port_settings) {
                    eprintln!("Error updating port settings: {e}");
                    return;
                }
                let (tx, rx) = mpsc::channel();
                if let Err(e) = manager.start(tx) {
                    eprintln!("Error starting port: {e}");
                } else {
                    self.button_text = "Close port".to_string();
                    *events_rx = Some(rx);
                }
            }
        } else {
            eprintln!("Serial manager is not initialized.");
        }
    }

    pub fn update_button_text(&mut self, manager: &Option<Box<dyn CommunicationManager>>) {
        if let Some(ref manager) = manager {
            if !manager.is_running() {
                self.button_text = "Open port".to_string();
            }
        }
    }

    fn flow_control_iter(&self) -> impl Iterator<Item = FlowControl> {
        [
            FlowControl::None,
            FlowControl::Software,
            FlowControl::Hardware,
        ]
        .iter()
        .cloned()
    }

    fn parity_iter(&self) -> impl Iterator<Item = Parity> {
        [Parity::None, Parity::Even, Parity::Odd].iter().cloned()
    }

    fn stop_bits_iter(&self) -> impl Iterator<Item = StopBits> {
        [StopBits::One, StopBits::Two].iter().cloned()
    }
}

impl Default for ConnectionPanel {
    fn default() -> Self {
        Self::new()
    }
}
