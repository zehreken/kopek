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

// This is the Time Domain graph
pub fn get_waveform_graph(frame_slice: &Vec<f32>, scale: f32) -> Vec<Point2> {
    let mut x = -513;
    let waveform_points = frame_slice
        .iter()
        .map(|frame| {
            x = x + 1;
            Point2 {
                x: x as f32,
                y: 100.0 + frame * scale,
            }
        })
        .collect();

    waveform_points
}

pub fn get_frequency_domain_graph(fft_output: &Vec<Complex<f64>>, scale: f32) -> Vec<Point2> {
    let sample_size = 1024 * 2;

    // let output = kopek::fft::fft(&fft_output);
    let mut x = -512.0;
    let frequency_graph_points: Vec<Point2> = fft_output
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

pub fn get_spectrum(frequency_line_points: &Vec<Point2>) -> Vec<Point2> {
    // implement another view to have non-linear bin sizes
    // e.g. 32-64-125-250-500-1k-2k-4k-8k-16k Hz
    // get half of model.frequency_line_points
    let mut sum = 1;
    let bin_sizes: Vec<i32> = (0..9)
        .map(|i| {
            sum += 2_i32.pow(i);
            sum
        })
        .collect();
    // println!("bin_sizes: {:?}", bin_sizes);
    // After this bin sizes are 4, 4, 8, 16, 32, 64, 128, 256. In total 512 data points, half of frequency_line_points
    let mut bin_averages: Vec<Point2> = vec![];
    let mut start_index = 0;
    for (i, end_index) in bin_sizes.into_iter().enumerate() {
        let sum: f32 = frequency_line_points[start_index as usize..end_index as usize]
            .iter()
            .map(|v| v.y)
            .sum();
        let average = sum / (end_index - start_index) as f32;

        // println!("{}, {}, {}", sum, end_index, start_index);
        // println!("{} {} average: {}", start_index, end_index, average);
        bin_averages.push(Point2 {
            x: -462.0 + 100.0 * i as f32,
            y: average,
        });
        start_index = end_index;
    }

    bin_averages
}
