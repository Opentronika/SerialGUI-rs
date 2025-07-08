#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod communicationtrait;
mod generalsettings;
mod gui;
mod guistrings;
mod info;
mod serial_impl;
mod update;
pub use app::TemplateApp;
