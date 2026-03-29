use serialport::{FlowControl, Parity, StopBits};

pub struct BaudRate {
    pub string_repr: &'static str,
    pub numeric_repr: u32,
}

pub const BAUD_RATES: [BaudRate; 20] = [
    BaudRate {
        string_repr: "110",
        numeric_repr: 110,
    },
    BaudRate {
        string_repr: "300",
        numeric_repr: 300,
    },
    BaudRate {
        string_repr: "600",
        numeric_repr: 600,
    },
    BaudRate {
        string_repr: "1200",
        numeric_repr: 1200,
    },
    BaudRate {
        string_repr: "2400",
        numeric_repr: 2400,
    },
    BaudRate {
        string_repr: "4800",
        numeric_repr: 4800,
    },
    BaudRate {
        string_repr: "9600",
        numeric_repr: 9600,
    },
    BaudRate {
        string_repr: "14400",
        numeric_repr: 14400,
    },
    BaudRate {
        string_repr: "19200",
        numeric_repr: 19200,
    },
    BaudRate {
        string_repr: "38400",
        numeric_repr: 38400,
    },
    BaudRate {
        string_repr: "57600",
        numeric_repr: 57600,
    },
    BaudRate {
        string_repr: "115200",
        numeric_repr: 115200,
    },
    BaudRate {
        string_repr: "128000",
        numeric_repr: 128000,
    },
    BaudRate {
        string_repr: "230400",
        numeric_repr: 230400,
    },
    BaudRate {
        string_repr: "256000",
        numeric_repr: 256000,
    },
    BaudRate {
        string_repr: "460800",
        numeric_repr: 460800,
    },
    BaudRate {
        string_repr: "921600",
        numeric_repr: 921600,
    },
    BaudRate {
        string_repr: "1000000",
        numeric_repr: 1_000_000,
    },
    BaudRate {
        string_repr: "2000000",
        numeric_repr: 2_000_000,
    },
    BaudRate {
        string_repr: "3000000",
        numeric_repr: 3_000_000,
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
            baudrate: BAUD_RATES[11].numeric_repr, // 115200
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
