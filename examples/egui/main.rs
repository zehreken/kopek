#![forbid(unsafe_code)]
// #![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod consts;
mod player;
mod utils;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = app::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    let player = player::Player::new();
    eframe::run_native(Box::new(app), native_options);
}