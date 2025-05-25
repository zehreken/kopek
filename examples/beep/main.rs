mod audio;
mod generator;
mod view;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use egui::vec2;

    let view = view::View::default();

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(vec2(720.0, 720.0));
    let _result = eframe::run_native("beep", native_options, Box::new(|_| Box::new(view)));
}
