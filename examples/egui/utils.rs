use super::consts;
use num::complex::Complex;

#[derive(Debug)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

// The representable range is 22050 if sample rate is 44100
// Frequency bin size is for each element in the output vector
// For example if the bin size is 22050 / 1024 = 21.53 and
// If the screen width is 1024, then each pixel will represent 21.53Hz
#[deprecated]
pub fn get_scale(num_of_points: u16) -> Vec<Point2> {
    let screen_width = 1024;
    let dist = screen_width / num_of_points;
    let scale_points: Vec<Point2> = (0..num_of_points)
        .into_iter()
        .map(|i| Point2 {
            x: -512.0 + (dist * i) as f32,
            y: -100.0,
        })
        .collect();
    scale_points
}

// This is the Time Domain graph
pub fn get_waveform_graph(frame_slice: &Vec<f32>, scale: f32) -> Vec<Point2> {
    let mut x = -512;
    let waveform_points = frame_slice
        .iter()
        .map(|frame| {
            x = x + 1;
            Point2 {
                x: x as f32,
                y: frame * scale,
            }
        })
        .collect();

    waveform_points
}

// Complex FFT gives z = x + jy, where x is the real part and y is the imaginary part
// Magnitude, |z| = sqrt(x^2 + y^2)
pub fn get_frequency_domain_graph(fft_output: &Vec<Complex<f64>>, x_scale: f32) -> Vec<Point2> {
    let sample_size = 1024 * 2;

    // let output = kopek::fft::fft(&fft_output);
    let mut x = -512.0;
    let frequency_graph_points: Vec<Point2> = fft_output
        .iter()
        .map(|c| {
            let magnitude = ((c.re as f32).powf(2.0) + (c.im as f32).powf(2.0)).sqrt();
            let p = Point2 {
                x,
                y: magnitude * 10_000.0,
            };
            x = x + 2048.0 / sample_size as f32 * x_scale;
            p
        })
        .collect();

    frequency_graph_points
}

pub fn get_narrow_bar_spectrum_scale() -> Vec<Point2> {
    let bin_sizes: Vec<i32> = vec![4, 4, 8, 16, 32, 64, 128, 256];
    let mut accumulator = 0.0;
    let bin_ranges: Vec<Point2> = bin_sizes
        .iter()
        .map(|bin_size| {
            let start_freq = accumulator;
            let end_freq = start_freq + consts::BIN_SIZE * *bin_size as f32;
            accumulator = end_freq;
            Point2 {
                x: start_freq,
                y: end_freq,
            }
        })
        .collect();

    bin_ranges
}

// Returns bar spectrum like the old cd players
pub fn get_narrow_bar_spectrum(frequency_line_points: &Vec<Point2>) -> Vec<Point2> {
    // implement another view to have non-linear bin sizes
    // e.g. 32-64-125-250-500-1k-2k-4k-8k-16k Hz
    // get half of model.frequency_line_points

    // println!("bin_sizes: {:?}", bin_sizes);

    let bin_sizes: Vec<i32> = vec![4, 4, 8, 16, 32, 64, 128, 256];
    // After this bin sizes are 4, 4, 8, 16, 32, 64, 128, 256. In total 512 data points, half of frequency_line_points
    // The other half is kind of a reflection
    // If sample rate is 44100, each data point will represent 44100 / 1024 = 43.07Hz
    let mut average_bins: Vec<Point2> = vec![];
    let mut start_index = 0;
    for (i, bin_size) in bin_sizes.into_iter().enumerate() {
        let end_index = start_index + bin_size;
        let sum: f32 = frequency_line_points[start_index as usize..end_index as usize]
            .iter()
            .map(|v| v.y)
            .sum();
        let average = sum / bin_size as f32;

        // println!("{}, {}, {}", start_index, end_index, sum);
        // println!("{} {} average: {}", start_index, end_index, average);
        average_bins.push(Point2 {
            x: -462.0 + 100.0 * i as f32,
            y: average,
        });
        start_index += bin_size;
    }

    average_bins
}
