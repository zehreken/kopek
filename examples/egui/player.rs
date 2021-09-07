use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, StreamConfig,
};

use super::utils;
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

pub struct Player {
    receiver: Receiver<Vec<[i16; 2]>>,
    time_line_points: Vec<Point2>,
    frequency_line_points: Vec<Point2>,
    scale_points: Vec<Point2>,
}

impl Player {
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
        play(PATHS[PATHS.len() - 1]);

        Player {
            receiver,
            time_line_points: vec![],
            frequency_line_points: vec![],
            scale_points: vec![],
        }
    }

    pub fn update(&mut self) {
        let mut frames = vec![[0; 2]; 1024];
        // Get the most recent frame
        for f in self.receiver.try_iter() {
            frames = f;
        }

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
}

fn play<P>(path: P)
where
    P: AsRef<std::path::Path>,
{
    let frames = kopek::decoder::decode(path);

    let frames: Vec<[f32; 2]> = frames
        .iter()
        .map(|frame| [frame[0] as f32, frame[1] as f32])
        .collect();

    // let mut cycling = frames.into_iter().clone().cycle();

    let host = cpal::default_host();

    let output_device = host
        .default_output_device()
        .expect("Output device not found!");
    let output_config: StreamConfig = output_device.default_output_config().unwrap().into();
    println!(
        "{:?}, {:?}",
        output_config.channels, output_config.sample_rate
    );

    let output_stream = create_output_stream(&output_device, &output_config, frames);

    output_stream.play();
    std::thread::sleep(std::time::Duration::from_millis(1000));
}

fn create_output_stream(
    output_device: &Device,
    config: &StreamConfig,
    track: Vec<[f32; 2]>,
) -> cpal::Stream {
    let output_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data {
            *sample = track.iter().next().unwrap()[0];
        }
    };

    output_device
        .build_output_stream(config, output_fn, err_fn)
        .unwrap()
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occured on stream: {}", err);
}

/*
fn play_ogg<P>(path: P, sender: Sender<Vec<[i16; 2]>>)
where
    P: AsRef<std::path::Path>,
{
    let frames = kopek::decoder::decode(path);

    let mut cycling = frames.into_iter().clone().cycle();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("output device not found");

    let config_range = device
        .supported_output_formats()
        .unwrap()
        .next()
        .expect("Failed get a config");

    let format = config_range.with_max_sample_rate();

    let event_loop = host.event_loop();
    let stream_id = event_loop
        .build_output_stream(&device, &format)
        .expect("Failed to create a voice");
    fn write_to_buffer<S, I>(
        mut buffer: cpal::OutputBuffer<S>,
        channels: usize,
        sine: &mut I,
        sender: Sender<Vec<[i16; 2]>>,
    ) where
        S: cpal::Sample + audrey::sample::FromSample<i16>,
        I: Iterator<Item = [i16; 2]>,
    {
        match channels {
            // Mono
            1 => {
                for (frame, sine_frame) in buffer.chunks_mut(channels).zip(sine) {
                    let sum = sine_frame[0] + sine_frame[1];
                    frame[0] = audrey::sample::Sample::to_sample(sum);
                }
            }

            // Stereo
            2 => {
                let mut frames: Vec<[i16; 2]> = vec![];
                for (frame, sine_frame) in buffer.chunks_mut(channels).zip(sine) {
                    for (sample, &sine_sample) in frame.iter_mut().zip(&sine_frame) {
                        *sample = audrey::sample::Sample::to_sample(sine_sample);
                        frames.push([sine_sample; 2]);
                    }
                }
                // println!("{:?}", frames.len());
                match sender.send(frames) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", err),
                }
            }

            _ => unimplemented!(),
        }
    }

    event_loop
        .play_stream(stream_id)
        .expect("failed to play_stream");

    std::thread::spawn(move || {
        event_loop.run(move |stream_id, buffer| {
            let stream_data = match buffer {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                    return;
                }
            };

            match stream_data {
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::U16(buffer),
                } => write_to_buffer(
                    buffer,
                    std::convert::From::from(format.channels),
                    &mut cycling,
                    sender.clone(),
                ),
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::I16(buffer),
                } => write_to_buffer(
                    buffer,
                    std::convert::From::from(format.channels),
                    &mut cycling,
                    sender.clone(),
                ),
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::F32(buffer),
                } => write_to_buffer(
                    buffer,
                    std::convert::From::from(format.channels),
                    &mut cycling,
                    sender.clone(),
                ),
                _ => (),
            }
        });
    });
}
*/
