use super::utils;
use super::utils::*;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, StreamConfig,
};
use std::sync::mpsc::{Receiver, Sender};

pub const PATHS: [&str; 11] = [
    "assets/audio_samples/sine_100.ogg",
    "assets/audio_samples/sine_200.ogg",
    "assets/audio_samples/sine_440.ogg",
    "assets/audio_samples/sine_500.ogg",
    "assets/audio_samples/sine_1000.ogg",
    "assets/audio_samples/sine_10000.ogg",
    "assets/audio_samples/sine_440hz_stereo.ogg",
    "assets/audio_samples/stress_free.wav",
    "assets/audio_samples/overture.wav",
    "assets/audio_samples/100_200_400_1000_10000.wav",
    "assets/audio_samples/sample.wav",
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
        let audio_host = cpal::default_host();

        let (sender, receiver) = std::sync::mpsc::channel::<Vec<[f32; 2]>>();

        Player {
            audio_host,
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

    pub fn record(&self) {
        let input_device = self
            .audio_host
            .default_input_device()
            .expect("Input device not found");
        let input_config: StreamConfig = input_device.default_input_config().unwrap().into();
        println!(
            "channels: {:?}, sample rate: {:?}",
            input_config.channels, input_config.sample_rate
        );

        let sender = self.sender.clone();
        std::thread::spawn(move || {
            let input_stream = create_input_stream(&input_device, &input_config, sender);
            input_stream.play().expect("Error while playing");
            std::thread::sleep(std::time::Duration::from_secs_f32(100.0));
        });
    }

    pub fn play(&self) {
        let output_device = self
            .audio_host
            .default_output_device()
            .expect("Output device not found");
        let output_config: StreamConfig = output_device.default_output_config().unwrap().into();
        println!(
            "channels: {:?}, sample rate: {:?}",
            output_config.channels, output_config.sample_rate,
        );

        let sender = self.sender.clone();
        let track = self.track.clone();

        let duration_in_seconds: f32 = track.len() as f32 / (44100.0 * 2.0);
        std::thread::spawn(move || {
            let output_stream = create_output_stream(&output_device, &output_config, track, sender);
            output_stream.play().expect("Error while playing");
            std::thread::sleep(std::time::Duration::from_secs_f32(duration_in_seconds));
        });
    }

    pub fn get_waveform_graph_points(&self) -> &Vec<Point2> {
        &self.waveform_graph_points
    }

    pub fn get_frequency_graph_points(&self) -> &Vec<Point2> {
        &self.frequency_graph_points
    }
}

fn create_input_stream(
    input_device: &Device,
    config: &StreamConfig,
    sender: Sender<Vec<[f32; 2]>>,
) -> cpal::Stream {
    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut frames: Vec<[f32; 2]> = vec![];
        for &sample in data {
            frames.push([sample; 2]);
        }

        match sender.send(frames) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {}", err),
        }
    };

    input_device
        .build_input_stream(config, input_data_fn, err_fn, None)
        .unwrap()
}

fn create_output_stream(
    output_device: &Device,
    config: &StreamConfig,
    track: Vec<f32>,
    sender: Sender<Vec<[f32; 2]>>,
) -> cpal::Stream {
    let mut index = 0;
    let channel_count: usize = config.channels as usize;
    let output_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut frames: Vec<[f32; 2]> = vec![];
        for frame in data.chunks_mut(channel_count) {
            if index < track.len() {
                frames.push([track[index]; 2]);
                for sample in frame.iter_mut().take(2) {
                    *sample = track[index];
                    index += 1;
                }
            } // sample can be set to 0.0 but the noise sounds cooler
        }

        match sender.send(frames) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {}", err),
        }
    };

    output_device
        .build_output_stream(config, output_fn, err_fn, None)
        .unwrap()
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occured on stream: {}", err);
}
