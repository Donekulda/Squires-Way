#![allow(non_snake_case)]

pub mod app;
pub mod core;
pub mod di;
pub mod game;
pub mod io;
pub mod localization;
pub mod merging;
pub mod parser;
pub mod platform;
pub mod services;
pub mod storage;

/// Run the desktop shell (Bevy + egui). Prefer calling from `main`.
pub fn run_app() {
    app::run();
}
