mod app;
mod audio;
mod view;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use egui::vec2;

    let view = view::View::default();

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(vec2(1080.0, 1080.0));
    let _result = eframe::run_native("music time", native_options, Box::new(|_| Box::new(view)));
}
