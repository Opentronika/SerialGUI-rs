use serialport::{available_ports, SerialPort, SerialPortType};
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{sync::mpsc, thread};

use crate::{
    communicationtrait::{CommunicationEvent, CommunicationManager, EPortState},
    serial_impl::PortSettings,
};

pub struct SerialCommunication {
    port_settings: PortSettings,
    port_state: Arc<Mutex<EPortState>>,
    port_thread: Option<thread::JoinHandle<()>>,
    tx_to_serial: Option<mpsc::Sender<Vec<u8>>>,
}
impl SerialCommunication {
    pub(crate) fn new() -> Self {
        Self {
            port_settings: PortSettings::default(),
            port_state: Arc::new(Mutex::new(EPortState::Closed)),
            port_thread: None,
            tx_to_serial: None,
        }
    }
}

impl CommunicationManager for SerialCommunication {
    fn start(&mut self, tx: mpsc::Sender<CommunicationEvent>) -> Result<()> {
        {
            *self.port_state.lock().unwrap() = EPortState::Opening;
        }
        let port_settings_clone = self.port_settings.clone();
        let port_state_clone = Arc::clone(&self.port_state);
        let (tx_to_serial, rx_from_app) = mpsc::channel();
        self.tx_to_serial = Some(tx_to_serial);
        let handle = thread::spawn(move || {
            // some work here
            let mut port: Option<Box<dyn SerialPort>>;
            {
                let mut port_state = port_state_clone.lock().unwrap();
                let portopen = serialport::new(
                    port_settings_clone.port_name.clone(),
                    port_settings_clone.baudrate,
                )
                .flow_control(port_settings_clone.flowcontrol)
                .parity(port_settings_clone.parity)
                .stop_bits(port_settings_clone.stop_bits)
                .timeout(Duration::from_millis(10))
                .open();
                match portopen {
                    Ok(portopen) => {
                        // port = Some(portopen);
                        *port_state = EPortState::Open;
                        port = Some(portopen);
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to open \"{}\". Error: {}",
                            port_settings_clone.port_name, e
                        );
                        *port_state = EPortState::Closed;
                        return;
                    }
                }
            }

            while *port_state_clone.lock().unwrap() == EPortState::Open {
                if let Some(ref mut port_instance) = port {
                    let size = port_instance.bytes_to_read().unwrap_or(0);
                    if size > 0 {
                        let mut serial_buf: Vec<u8> = vec![0; size as usize];
                        match port_instance.read_exact(&mut serial_buf) {
                            Ok(_) => {
                                // Handle channel send errors gracefully
                                if tx
                                    .send(CommunicationEvent::DataReceived(serial_buf))
                                    .is_err()
                                {
                                    eprintln!("GUI channel disconnected, stopping serial thread");
                                    break; // Exit the loop if GUI is gone
                                }
                            }
                            Err(e) => {
                                eprintln!("Serial read error: {e}");
                            }
                        }
                    }
                }

                if let Ok(message) = rx_from_app.try_recv() {
                    // self.write_log(message.as_str());
                    if let Some(ref mut port_instance) = port {
                        match port_instance.write_all(&message) {
                            Ok(_) => eprintln!("Write success"),
                            Err(e) => eprintln!("{e:?}"),
                        }
                    }
                }
            }
        });
        self.port_thread = Some(handle);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        {
            let mut port_state = self.port_state.lock().unwrap();
            if *port_state == EPortState::Open {
                *port_state = EPortState::Closed;
            }
        }
        if let Some(handle) = self.port_thread.take() {
            handle.join().unwrap();
        }
        self.port_thread = None;
        Ok(())
    }

    fn is_running(&self) -> bool {
        let port_state = self.port_state.lock().unwrap();
        *port_state == EPortState::Open || *port_state == EPortState::Opening
    }

    fn send_data(&mut self, data: Vec<u8>) -> Result<()> {
        if let Some(tx) = &self.tx_to_serial {
            tx.send(data)
                .map_err(|e| std::io::Error::other(format!("Failed to send data: {e}")))?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Serial port is not open",
            ));
        }
        Ok(())
    }

    fn get_available_connections(&self) -> Vec<String> {
        let mut port_list: Vec<String> = vec![];
        match available_ports() {
            Ok(mut ports) => {
                ports.sort_by_key(|i| i.port_name.clone());
                port_list.clear();
                match ports.len() {
                    0 => println!("No ports found."),
                    1 => println!("Found 1 port:"),
                    n => println!("Found {n} ports:"),
                };

                for p in ports {
                    println!("    {}", p.port_name);
                    port_list.push(p.port_name);
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
                eprintln!("{e:?}");
                eprintln!("Error listing serial ports");
            }
        }
        port_list
    }

    fn update_settings(&mut self, settings: &dyn std::any::Any) -> Result<()> {
        if let Some(new_settings) = settings.downcast_ref::<PortSettings>() {
            self.port_settings = new_settings.clone();
            eprintln!("Updated port settings: {:?}", self.port_settings.baudrate);
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid settings type",
            ))
        }
    }
}
