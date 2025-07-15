pub mod connection_panel;
pub mod file_log_panel;
pub mod menu_bar;
pub mod rx_panel;
pub mod send_panel;
pub mod settings_panel;

// Re-export para facilitar el uso
pub use connection_panel::ConnectionPanel;
pub use file_log_panel::FileLogPanel;
pub use menu_bar::MenuBar;
pub use rx_panel::RxPanel;
pub use send_panel::SendPanel;
