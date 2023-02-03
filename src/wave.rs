pub fn get_sine() {

    let freq: f32 = if first_beat {
        utils::C_FREQ
    } else {
        utils::A_FREQ
    };
    let volume = 0.2;
    t_index as f32 * 2.0 * std::f32::consts::PI * freq / 44100.0).sin();
}