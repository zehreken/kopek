use super::consts;
use nannou::prelude::*;
use num::complex::Complex;

// The representable range is 22050 if sample rate is 44100
// Frequency bin size is for each element in the output vector
// For example if the bin size is 22050 / 1024 = 21.53 and
// If the screen width is 1024, then each pixel will represent 21.53Hz
pub fn get_scale(x_scale: f32) -> Vec<Point2> {
    let scale_points: Vec<Point2> = (0..128)
        .into_iter()
        .map(|i| Point2 {
            x: -512.0 + 8.0 * i as f32 * x_scale,
            y: -100.0,
        })
        .collect();

    scale_points
}

pub fn get_frequency_domain_graph(fft_output: &Vec<Complex<f64>>, scale: f32) -> Vec<Point2> {
    let sample_size = 1024 * 2;

    let output = kopek::fft::fft(&fft_output);
    let mut x = -512.0;
    let frequency_graph_points: Vec<Point2> = output
        .iter()
        .map(|c| {
            let p = Point2 {
                x,
                y: -200.0 + ((c.re as f32).powf(2.0) + (c.im as f32).powf(scale)).sqrt(),
            };
            x = x + 2048.0 / sample_size as f32 * consts::X_SCALE;
            p
        })
        .collect();

    frequency_graph_points
}
