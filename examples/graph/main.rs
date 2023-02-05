#![forbid(unsafe_code)]
// #![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod consts;
mod player;
mod utils;
mod view;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let view = view::AnalysisView::default();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(view)),
    );
}
