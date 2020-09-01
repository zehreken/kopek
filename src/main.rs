mod lib;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let mut domi_ogg = audrey::open("domi.ogg").unwrap();

    let mut cycling = domi_ogg
        .frames::<[i16; 2]>()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .iter()
        .cloned()
        .cycle();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("output device not found");

    let config_range = device
        .supported_output_configs()
        .unwrap()
        .next()
        .expect("Failed get a config");

    let mut config = config_range.with_max_sample_rate();

    let channels = config.channels() as usize;
    let mut data_callback = move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
        match channels {
            1 => {
                // for (frame, ogg_frame) in data.chunks_mut(config.channels() as usize).zip(cycling) {

                // }
            }
            2 => {}
            _ => unimplemented!(),
        }
    };
    let error_callback = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = device
        .build_output_stream(&config.into(), data_callback, error_callback)
        .unwrap();

    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(3000));
}

/*
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
    */
