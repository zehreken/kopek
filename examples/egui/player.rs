use super::utils;
use super::utils::*;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, StreamConfig,
};
use std::sync::mpsc::{Receiver, Sender};

pub const PATHS: [&str; 11] = [
    "sine_100.ogg",
    "sine_200.ogg",
    "sine_440.ogg",
    "sine_500.ogg",
    "sine_1000.ogg",
    "sine_10000.ogg",
    "sine_440hz_stereo.ogg",
    "stress_free.wav",
    "overture.wav",
    "100_200_400_1000_10000.wav",
    "sample.wav",
];

pub struct Player {
    audio_host: Host,
    sender: Sender<Vec<[f32; 2]>>,
    receiver: Receiver<Vec<[f32; 2]>>,
    waveform_graph_points: Vec<Point2>,
    frequency_graph_points: Vec<Point2>,
    track: Vec<f32>,
}

impl Player {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let (sender, receiver) = std::sync::mpsc::channel::<Vec<[f32; 2]>>();

        Player {
            audio_host: host,
            sender,
            receiver,
            waveform_graph_points: vec![],
            frequency_graph_points: vec![],
            track: Player::load_track_at_path(PATHS[0]), // output_stream: create_output_stream(),
        }
    }

    pub fn update(&mut self) {
        let mut frames = vec![[0.0; 2]; 1024];
        // Get the most recent frame
        for f in self.receiver.try_iter() {
            frames = f;
        }

        // println!("received: {:?}", frames[0]);
        let fft_input: Vec<_> = frames
            .iter()
            .map(|frame| std::convert::From::from(frame[0] as f64 / std::i16::MAX as f64))
            .collect();

        let fft_output = kopek::fft::fft(&fft_input);

        if frames.len() > 0 {
            let frame_slice = frames
                .iter()
                .map(|frame| 30.0 + frame[0] as f32 * 20.0)
                .collect();
            self.waveform_graph_points = utils::get_waveform_graph(&frame_slice, 1.0);
            self.frequency_graph_points = utils::get_frequency_domain_graph(&fft_output, 1.0);
            // self.scale_points = utils::get_scale(128);
        }
    }

    pub fn load_track<P>(&mut self, path: P)
    where
        P: AsRef<std::path::Path>,
    {
        self.track = Player::load_track_at_path(path);
    }

    fn load_track_at_path<P>(path: P) -> Vec<f32>
    where
        P: AsRef<std::path::Path>,
    {
        let frames = kopek::decoder::decode(path);
        let volume_factor = 0.00002;
        let frames: Vec<f32> = frames
            .iter()
            .map(|frame| {
                [
                    volume_factor * frame[0] as f32,
                    volume_factor * frame[1] as f32,
                ]
            })
            .flatten()
            .collect();

        println!("frames: {}", frames.len());
        println!("duration: {}", frames.len() as u64 / (44100 * 2));

        frames
    }

    pub fn play(&self) {
        let output_device = self
            .audio_host
            .default_output_device()
            .expect("Output device not found");
        let output_config: StreamConfig = output_device.default_output_config().unwrap().into();
        println!(
            "{:?}, {:?}",
            output_config.channels, output_config.sample_rate,
        );

        let sender = self.sender.clone();
        let track = self.track.clone();
        std::thread::spawn(move || {
            let duration_in_seconds: u64 = track.len() as u64 / (44100 * 2);
            let output_stream = create_output_stream(&output_device, &output_config, track, sender);
            output_stream.play().expect("Error while playing");
            std::thread::sleep(std::time::Duration::from_millis(
                duration_in_seconds * 1000 - 100,
            ));
        });
    }

    pub fn get_waveform_graph_points(&self) -> &Vec<Point2> {
        &self.waveform_graph_points
    }

    pub fn get_frequency_graph_points(&self) -> &Vec<Point2> {
        &self.frequency_graph_points
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
