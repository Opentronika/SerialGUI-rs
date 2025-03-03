use serialport::{FlowControl, Parity, StopBits};
pub struct PortSettings {
    pub port_name: String,
    pub baudrate: u32,
    pub flowcontrol: FlowControl,
    pub parity: Parity,
    pub stop_bits: StopBits,
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
