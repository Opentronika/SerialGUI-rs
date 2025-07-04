use std::io::Result;
use std::sync::mpsc;

#[derive(PartialEq)]
pub enum EPortState {
    Open,
    Closed,
    Opening,
}
/// Events that can be emitted by a communication channel.
#[allow(dead_code)]
pub enum CommunicationEvent {
    DataReceived(Vec<u8>),
    ConnectionClosed,
    Error(String),
}

/// Trait for managing a generic byte stream communication channel (serial, TCP, etc.).
pub trait CommunicationManager: Send {
    /// Start the communication and background thread, sending events to the provided channel.
    fn start(&mut self, tx: mpsc::Sender<CommunicationEvent>) -> Result<()>;

    /// Stop the communication and background thread.
    fn stop(&mut self) -> Result<()>;

    /// Returns true if the communication is currently running.
    fn is_running(&self) -> bool;

    /// Send data asynchronously to the channel.
    fn send_data(&mut self, data: Vec<u8>) -> Result<()>;

    /// List available connections (e.g., serial ports, network endpoints).
    fn get_available_connections(&self) -> Vec<String>;

    /// (Optional) Update the communication settings. Implementation may downcast the type.
    fn update_settings(&mut self, _settings: &dyn std::any::Any) -> Result<()> {
        Ok(())
    }
}
