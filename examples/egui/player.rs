use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, StreamConfig,
};

use super::utils;
use super::utils::*;
use std::sync::mpsc::{Receiver, Sender};

pub const PATHS: [&str; 1] = [
    // "sine_100.ogg",
    // "sine_200.ogg",
    // "sine_440.ogg",
    // "sine_500.ogg",
    // "sine_1000.ogg",
    // "sine_10000.ogg",
    // "sine_440hz_stereo.ogg",
    // "stress_free.wav",
    // "overture.wav",
    // "100_200_400_1000_10000.wav",
    "sample.wav",
];

pub struct Player {
    audio_host: Host,
    sender: Sender<Vec<[f32; 2]>>,
    receiver: Receiver<Vec<[f32; 2]>>,
    pub time_line_points: Vec<Point2>,
    pub frequency_line_points: Vec<Point2>,
    pub scale_points: Vec<Point2>,
}

impl Player {
    pub fn new() -> Self {
        /* This part is not really necessary
        let sample_size = 1024;
        let start = 0;
        let end = start + sample_size;
        let mut frames_sum: Vec<[i16; 2]> = vec![[0, 0]; sample_size]; // Not used currently
        for path in PATHS.iter() {
            let frames = &kopek::decoder::decode(path)[start..end];
            for (i, frame) in frames.iter().enumerate() {
                frames_sum[i][0] += frame[0] / PATHS.len() as i16; // First divide by the number of waves and then sum because i16 overflows easily
                frames_sum[i][1] += frame[1] / PATHS.len() as i16;
            }
        }
        */

        let host = cpal::default_host();
        for device in host.devices().unwrap() {
            println!("Device: {:?}", device.name());
            if let Ok(input_config) = device.default_input_config() {
                println!("Input buffer size: {:?}", input_config.buffer_size());
                println!("Input channel count: {:?}", input_config.channels());
            } else {
                println!("No input config for this device");
            }

            if let Ok(output_config) = device.default_output_config() {
                println!("Output buffer size: {:?}", output_config.buffer_size());
                println!("Output channel count: {:?}", output_config.channels());
            } else {
                println!("No output config for this device");
            }
            println!("\n");
        }

        let (sender, receiver) = std::sync::mpsc::channel::<Vec<[f32; 2]>>();
        // play_ogg(PATHS[PATHS.len() - 1], sender);
        // play(PATHS[PATHS.len() - 1]);

        Player {
            audio_host: host,
            sender,
            receiver,
            time_line_points: vec![],
            frequency_line_points: vec![],
            scale_points: vec![],
        }
    }

    pub fn update(&mut self) {
        let mut frames = vec![[0.0; 2]; 1024];
        // Get the most recent frame
        for f in self.receiver.try_iter() {
            frames = f;
        }

        println!("received: {:?}", frames[0]);
        let fft_input: Vec<_> = frames
            .iter()
            .map(|frame| std::convert::From::from(frame[0] as f64 / std::i16::MAX as f64))
            .collect();

        let fft_output = kopek::fft::fft(&fft_input);

        if frames.len() > 0 {
            let frame_slice = frames
                .iter()
                .map(|frame| 100.0 + frame[0] as f32 / 500.0)
                .collect();
            self.time_line_points = utils::get_waveform_graph(&frame_slice, 1.0);
            self.frequency_line_points = utils::get_frequency_domain_graph(&fft_output, 2.0);
            self.scale_points = utils::get_scale(128);
        }
    }

    pub fn play<P>(&self, path: P)
    where
        P: AsRef<std::path::Path>,
    {
        let frames = kopek::decoder::decode(path);

        let factor = 0.00002;
        let frames: Vec<f32> = frames
            .iter()
            .map(|frame| [factor * frame[0] as f32, factor * frame[1] as f32])
            .flatten()
            .collect();

        println!("frames: {}", frames.len());

        let output_device = self
            .audio_host
            .default_output_device()
            .expect("Output device not found");
        let output_config: StreamConfig = output_device.default_output_config().unwrap().into();
        println!(
            "{:?}, {:?}",
            output_config.channels, output_config.sample_rate,
        );

        let s = self.sender.clone();
        std::thread::spawn(move || {
            let output_stream = create_output_stream(&output_device, &output_config, frames, s);
            output_stream.play().expect("Error while playing");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        });
    }
}

fn create_output_stream(
    output_device: &Device,
    config: &StreamConfig,
    track: Vec<f32>,
    sender: Sender<Vec<[f32; 2]>>,
) -> cpal::Stream {
    let mut index = 0;
    let output_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut frames: Vec<[f32; 2]> = vec![];
        for sample in data {
            *sample = track[index];
            frames.push([*sample; 2]);
            index += 1;
        }

        match sender.send(frames) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {}", err),
        }
    };

    output_device
        .build_output_stream(config, output_fn, err_fn)
        .unwrap()
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occured on stream: {}", err);
}
