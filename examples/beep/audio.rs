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
use ringbuf::HeapConsumer;

const LATENCY_MS: f32 = 10.0;

pub struct Model {
    output_stream: Stream,
}

impl Model {
    pub fn new(mut consumer: HeapConsumer<f32>) -> Result<Model, anyhow::Error> {
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
        let config: cpal::StreamConfig = output_device.default_output_config()?.into();

        // Create a delay in case the input and output devices aren't synced.
        // let latency_frames = (LATENCY_MS / 1_000.0) * config.sample_rate.0 as f32;
        // let latency_samples = latency_frames as usize * config.channels as usize;

        // The buffer to share samples
        // let ring = HeapRb::new(4096);
        // let (producer, mut consumer) = ring.split();

        // Fill the samples with 0.0 equal to the length of the delay.
        // for _ in 0..latency_samples {
        //     // The ring buffer has twice as much space as necessary to add latency here,
        //     // so this should never fail
        //     producer.push(0.0).unwrap();
        // }

        let channel_count = config.channels as usize;

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                if let Some(input) = consumer.pop() {
                    *sample = input;
                }
            }
        };

        // Build streams.
        println!(
            "Attempting to build both streams with f32 samples and `{:?}`.",
            config
        );
        let output_stream =
            output_device.build_output_stream(&config, output_data_fn, err_fn, None)?;
        println!("Successfully built streams.");

        // Play the streams.
        println!(
            "Starting the output stream with `{}` milliseconds of latency.",
            LATENCY_MS
        );
        output_stream.play().expect("Can't play output stream");

        Ok(Model { output_stream })
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
