mod lib;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

fn main() {
    let mut domi_ogg = audrey::open("domi.ogg").unwrap();

    let mut cycling = domi_ogg
        .frames::<[i16; 2]>()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut cycling = cycling.iter().cloned().cycle();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("output device not found");

    let config_range = device
        .supported_output_formats()
        .unwrap()
        .next()
        .expect("Failed get a config");

    let mut format = config_range.with_max_sample_rate();

    let channels = format.channels as usize;
    let event_loop = host.event_loop();
    let stream_id = event_loop
        .build_output_stream(&device, &format)
        .expect("Failed to create a voice");
    fn write_to_buffer<S, I>(mut buffer: cpal::OutputBuffer<S>, channels: usize, sine: &mut I)
    where
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
                for (frame, sine_frame) in buffer.chunks_mut(channels).zip(sine) {
                    for (sample, &sine_sample) in frame.iter_mut().zip(&sine_frame) {
                        *sample = audrey::sample::Sample::to_sample(sine_sample);
                    }
                }
            }

            _ => unimplemented!(),
        }
    }

    event_loop
        .play_stream(stream_id)
        .expect("failed to play_stream");

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
            } => write_to_buffer(buffer, usize::from(format.channels), &mut cycling),
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::I16(buffer),
            } => write_to_buffer(buffer, usize::from(format.channels), &mut cycling),
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::F32(buffer),
            } => write_to_buffer(buffer, usize::from(format.channels), &mut cycling),
            _ => (),
        }
    });
}
