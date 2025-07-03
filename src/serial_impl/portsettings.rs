use serialport::{FlowControl, Parity, StopBits};

pub struct BaudRate {
    pub string_repr: &'static str,
    pub numeric_repr: u32,
}

pub const BAUD_RATES: [BaudRate; 3] = [
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
pub struct PortSettings {
    pub port_name: String,
    pub baudrate: u32,
    pub flowcontrol: FlowControl,
    pub parity: Parity,
    pub stop_bits: StopBits,
}

impl Default for PortSettings {
    fn default() -> Self {
        PortSettings {
            port_name: String::new(),
            baudrate: BAUD_RATES[2].numeric_repr, // Por ejemplo, 115200
            flowcontrol: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
        }
    }
}

impl Clone for PortSettings {
    fn clone(&self) -> Self {
        PortSettings {
            port_name: self.port_name.clone(),
            baudrate: self.baudrate,
            flowcontrol: self.flowcontrol,
            parity: self.parity,
            stop_bits: self.stop_bits,
        }
    }
}
