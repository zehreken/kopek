extern crate kopek;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use nannou::prelude::*;
use pprof;
use std::sync::mpsc::{Receiver, Sender};

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    receiver: Receiver<Vec<[i16; 2]>>,
    time_line_points: Vec<Point2>,
    frequency_line_points: Vec<Point2>,
    scale_points: Vec<Point2>,
    guard: pprof::ProfilerGuard<'static>,
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::rate_rate_fps(30.0)); // This is buggy in current version of nannou
    // Create pprof guard here
    let guard = pprof::ProfilerGuard::new(100).unwrap();

    let _window = app
        .new_window()
        .size(1100, 600)
        .title("kopek")
        .view(view)
        .build()
        .unwrap();

    let paths = [
        // "sine_100.ogg",
        // "sine_200.ogg",
        // "sine_440.ogg",
        // "sine_500.ogg",
        // "sine_1000.ogg",
        // "sine_10000.ogg",
        // "sine_440hz_stereo.ogg",
        // "dimsunk_funky.ogg",
        // "sample.ogg",
        // "stress_free.wav",
        // "overture.wav",
        "100_200_400_1000_10000.wav",
    ];

    let sample_size = 1024;
    let start = 0;
    let end = start + sample_size;
    let mut frames_sum: Vec<[i16; 2]> = vec![[0, 0]; sample_size];
    for path in paths.iter() {
        let frames = &kopek::decoder::decode(path)[start..end];
        for (i, frame) in frames.iter().enumerate() {
            frames_sum[i][0] += frame[0] / paths.len() as i16; // First divide by the number of waves and then sum because i16 overflows easily
            frames_sum[i][1] += frame[1] / paths.len() as i16;
        }
    }

    let frames_slice: Vec<[i16; 2]> = frames_sum[start..end].into();
    let (time_line_points, frequency_line_points, scale_points) = analyze(frames_slice);

    let (sender, receiver) = std::sync::mpsc::channel::<Vec<[i16; 2]>>();
    play_ogg(paths[paths.len() - 1], sender);

    Model {
        receiver,
        time_line_points,
        frequency_line_points,
        scale_points,
        guard,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mut frames_count = 0;
    let mut frames = vec![[0; 2]; 1024];
    for _frames in model.receiver.try_iter() {
        // for i in 0..1024 {
        //     frames[i][0] += _frames[i][0];
        //     frames[i][1] += _frames[i][1];
        // }
        // frames_count += 1;
        frames = _frames;
    }

    // frames_count = (frames_count as f32 / 10.0).ceil() as i16;
    // for f in &mut frames {
    //     f[0] = f[0] / frames_count;
    //     f[1] = f[1] / frames_count;
    // }

    if frames.len() > 0 {
        let (time_line, frequency_line, circles) = analyze(frames);
        model.time_line_points = time_line;
        model.frequency_line_points = frequency_line;
        model.scale_points = circles;
    }

    std::thread::sleep(std::time::Duration::from_millis(33)); // Roughly set to 30 FPS
}

const BIN_SIZE: f32 = 22050.0 / 1024.0;
const X_SCALE: f32 = 4.0;

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.polyline()
        .weight(1.0)
        .points(model.time_line_points.clone())
        .color(CRIMSON);

    // draw.polyline()
    //     .weight(1.0)
    //     .points(model.frequency_line_points.clone())
    //     .color(GREEN);

    let average_bins = get_spectrum(model);
    // draw.polyline().weight(1.0).points(average_bins).color(RED);
    for bin in average_bins {
        draw.rect()
            .x_y(bin.x, -100.0)
            .w_h(90.0, 200.0 - bin.y.abs())
            .color(GREEN);
    }

    for (i, point) in model.scale_points.iter().enumerate() {
        draw.rect().w_h(1.0, 10.0).xy(*point).color(BLACK);
        let bin_text = i as f32 * BIN_SIZE * X_SCALE * 8.0;
        draw.text(&format!("{:0.0}hz", bin_text))
            .font_size(6)
            .x_y(point.x, -80.0)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn get_spectrum(model: &Model) -> Vec<Point2> {
    // implement another view to have non-linear bin sizes
    // e.g. 32-64-125-250-500-1k-2k-4k-8k-16k Hz
    // get half of model.frequency_line_points
    let mut sum = 2;
    let bin_sizes: Vec<i32> = (1..9)
        .map(|i| {
            sum += 2_i32.pow(i);
            sum
        })
        .collect();
    // After this bin sizes are 4, 4, 8, 16, 32, 64, 128, 256. In total 512 data points, half of frequency_line_points
    let mut bin_averages: Vec<Point2> = vec![];
    let mut start_index = 0;
    for (i, end_index) in bin_sizes.into_iter().enumerate() {
        let sum: &f32 = &model.frequency_line_points[start_index as usize..end_index as usize]
            .iter()
            .map(|v| v.y)
            .sum();
        let average = sum / (end_index - start_index) as f32;
        // println!("{} {} average: {}", start_index, end_index, average);
        bin_averages.push(Point2 {
            x: -512.0 + 100.0 * i as f32,
            y: average,
        });
        start_index = end_index;
    }

    bin_averages
}

fn exit(_app: &App, model: Model) {
    if cfg!(debug_assertions) {
        if let Ok(report) = model.guard.report().build() {
            println!("report: {}", &report);
            let file = std::fs::File::create("nannou.svg").unwrap();
            report.flamegraph(file).unwrap();
        }
    }
}

fn analyze(frames_slice: Vec<[i16; 2]>) -> (Vec<Point2>, Vec<Point2>, Vec<Point2>) {
    let sample_size = 1024;
    let mut x = -513;
    let time_line_points: Vec<Point2> = frames_slice
        .iter()
        .map(|frame| {
            x = x + 1;
            Point2 {
                x: x as f32,
                y: 100.0 + (frame[0] as f32 / 500.0),
            }
        })
        .collect();

    let input: Vec<_> = frames_slice
        .iter()
        .map(|frame| std::convert::From::from(frame[0] as f64 / std::i16::MAX as f64))
        .collect();

    let output = kopek::fft::fft(&input);
    let mut x = -512.0;
    let frequency_line_points: Vec<Point2> = output
        .iter()
        .map(|c| {
            let p = Point2 {
                x,
                y: -200.0 + ((c.re as f32).powf(2.0) + (c.im as f32).powf(2.0)).sqrt(),
            };
            x = x + 1024.0 / sample_size as f32 * X_SCALE;
            p
        })
        .collect();

    // First, the total range is 22050 if sample rate is 44100
    // Frequency bin size is for each element in the output vector
    // For example if the bin size is 22050 / 1024 = 21.53 and
    // If the screen width is 1024, then each pixel will represent 21.53Hz
    let scale_points: Vec<Point2> = (0..128)
        .into_iter()
        .map(|i| Point2 {
            x: -512.0 + 8.0 * i as f32 * X_SCALE,
            y: -100.0,
        })
        .collect();

    (time_line_points, frequency_line_points, scale_points)
}

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
