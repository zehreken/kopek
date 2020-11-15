extern crate kopek;
use super::consts;
use super::utils;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use nannou::prelude::*;
use pprof;
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

pub fn start() {
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
        .size(consts::SCREEN_WIDTH, consts::SCREEN_HEIGHT)
        .title("kopek_play_ogg")
        .view(view)
        .build()
        .unwrap();

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
    play_ogg(PATHS[PATHS.len() - 1], sender);

    Model {
        receiver,
        time_line_points: vec![],
        frequency_line_points: vec![],
        scale_points: vec![],
        guard,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
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

    let fft_input: Vec<_> = frames
        .iter()
        .map(|frame| std::convert::From::from(frame[0] as f64 / std::i16::MAX as f64))
        .collect();

    let fft_output = kopek::fft::fft(&fft_input);

    if frames.len() > 0 {
        let (time_line, frequency_line) = analyze(frames);
        model.time_line_points = time_line;
        model.frequency_line_points = utils::get_frequency_domain_graph(&fft_output, 1.0);
        model.scale_points = utils::get_scale(consts::X_SCALE);
    }

    std::thread::sleep(std::time::Duration::from_millis(33)); // Roughly set to 30 FPS
}

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

    if model.frequency_line_points.len() == 1024 {
        let average_bins = utils::get_spectrum(&model.frequency_line_points);
        // draw.polyline().weight(1.0).points(average_bins).color(RED);
        for bin in average_bins {
            // TODO: Fix and remove NaN check
            if !bin.y.is_nan() {
                draw.rect()
                    .x_y(bin.x, -100.0)
                    .w_h(90.0, 200.0 - bin.y.abs())
                    .color(GREEN);
            }
        }

        for (i, point) in model.scale_points.iter().enumerate() {
            draw.rect().w_h(1.0, 10.0).xy(*point).color(BLACK);
            let bin_text = i as f32 * consts::BIN_SIZE * consts::X_SCALE * 8.0;
            draw.text(&format!("{:0.0}hz", bin_text))
                .font_size(6)
                .x_y(point.x, -80.0)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
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

fn analyze(frames_slice: Vec<[i16; 2]>) -> (Vec<Point2>, Vec<Point2>) {
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
            x = x + 1024.0 / sample_size as f32 * consts::X_SCALE;
            p
        })
        .collect();

    (time_line_points, frequency_line_points)
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
