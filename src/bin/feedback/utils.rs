use nannou::prelude::*;

// This is just a equally spaced scale, ruler, nothing fancy
pub fn get_scale(x_scale: f32) -> Vec<Point2> {
    // First, the total range is 22050 if sample rate is 44100
    // Frequency bin size is for each element in the output vector
    // For example if the bin size is 22050 / 1024 = 21.53 and
    // If the screen width is 1024, then each pixel will represent 21.53Hz
    let scale_points: Vec<Point2> = (0..128)
        .into_iter()
        .map(|i| Point2 {
            x: -512.0 + 8.0 * i as f32 * x_scale,
            y: -100.0,
        })
        .collect();

    scale_points
}
