use super::utils::*;
use std::sync::mpsc::{Receiver, Sender};

const PATHS: [&str; 1] = [
    // "sine_100.ogg",
    // "sine_200.ogg",
    // "sine_440.ogg",
    // "sine_500.ogg",
    // "sine_1000.ogg",
    // "sine_10000.ogg",
    // "sine_440hz_stereo.ogg",
    // "stress_free.wav",
    // "overture.wav",
    "100_200_400_1000_10000.wav",
];

struct Model {
    receiver: Receiver<Vec<[i16; 2]>>,
    time_line_points: Vec<Point2>,
    frequency_line_points: Vec<Point2>,
    scale_points: Vec<Point2>,
}

impl Model {
    pub fn new() -> Self {
        let sample_size = 1024;
        let start = 0;
        let end = start + sample_size;
        let mut frames_sum: Vec<[i16; 2]> = vec![[0, 0]; sample_size];
        for path in PATHS.iter() {
            let frames = &kopek::decoder::decode(path)[start..end];
            for (i, frame) in frames.iter().enumerate() {
                frames_sum[i][0] += frame[0] / PATHS.len() as i16; // First divide by the number of waves and then sum because i16 overflows easily
                frames_sum[i][1] += frame[1] / PATHS.len() as i16;
            }
        }

        let (sender, receiver) = std::sync::mpsc::channel::<Vec<[i16; 2]>>();
        // play_ogg(PATHS[PATHS.len() - 1], sender);

        Model {
            receiver,
            time_line_points: vec![],
            frequency_line_points: vec![],
            scale_points: vec![],
        }
    }
}
