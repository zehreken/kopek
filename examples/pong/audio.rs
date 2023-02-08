//! Feeds back the input stream directly into the output stream.
//!
//! Assumes that the input and output devices can use the same stream configuration and that they
//! support the f32 sample format.
//!
//! Uses a delay of `LATENCY_MS` milliseconds in case the default input and output streams are not
//! precisely synchronised.

extern crate anyhow;
extern crate cpal;
extern crate ringbuf;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};
use ringbuf::{HeapConsumer, HeapRb};

const LATENCY_MS: f32 = 10.0;

pub struct Model {
    pub input_stream: Stream,
    pub consumer: HeapConsumer<f32>,
}

impl Model {
    pub fn new() -> Result<Model, anyhow::Error> {
        let host = cpal::default_host();

        // Default devices.
        let input_device = host
            .default_input_device()
            .expect("failed to get default input device");
        let output_device = host
            .default_output_device()
            .expect("failed to get default output device");
        println!("Using default input device: \"{}\"", input_device.name()?);
        println!("Using default output device: \"{}\"", output_device.name()?);

        // We'll try and use the same configuration between streams to keep it simple.
        let config: cpal::StreamConfig = input_device.default_input_config()?.into();

        // Create a delay in case the input and output devices aren't synced.
        let latency_frames = (LATENCY_MS / 1_000.0) * config.sample_rate.0 as f32;
        let latency_samples = latency_frames as usize * config.channels as usize;

        // The buffer to share samples
        let ring = HeapRb::new(latency_samples * 2);
        let (mut producer, consumer) = ring.split();

        // Fill the samples with 0.0 equal to the length of the delay.
        for _ in 0..latency_samples {
            // The ring buffer has twice as much space as necessary to add latency here,
            // so this should never fail
            producer.push(0.0).unwrap();
        }

        let channel_count = config.channels as usize;
        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // let mut output_fell_behind = false;
            for frame in data.chunks(channel_count) {
                let mut average = 0.0;
                for sample in frame.iter().take(2) {
                    average += sample;
                }
                average = average / frame.len() as f32;
                if producer.push(average).is_err() {
                    // output_fell_behind = true;
                }
            }
            // if output_fell_behind {
            //     eprintln!("output stream fell behind: try increasing latency");
            // }
        };

        // Build streams.
        println!(
            "Attempting to build both streams with f32 samples and `{:?}`.",
            config
        );
        let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn, None)?;
        println!("Successfully built streams.");

        // Play the streams.
        println!(
            "Starting the input and output streams with `{}` milliseconds of latency.",
            LATENCY_MS
        );
        input_stream.play()?;
        // output_stream.play()?;

        Ok(Model {
            input_stream,
            consumer,
        })
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
