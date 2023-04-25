mod audio;
mod generator; // oscillator?
mod view;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let view = view::View::default();

    let native_options = eframe::NativeOptions::default();
    let _result = eframe::run_native("beep", native_options, Box::new(|_| Box::new(view)));
}
