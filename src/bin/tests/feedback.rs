extern crate kopek;
use super::consts;
use super::utils;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use ringbuf::{Consumer, Producer, RingBuffer};

pub fn start() {
    nannou::app(model).update(update).run();
}

struct InputModel {
    pub producer: Producer<f32>,
}

struct Model {
    _input_stream: audio::Stream<InputModel>,
    consumer: Consumer<f32>,
    time_line_points: Vec<Point2>,
    frequency_line_points: Vec<Point2>,
    scale_points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(consts::SCREEN_WIDTH, consts::SCREEN_HEIGHT)
        .title("kopek_feedback")
        .view(view)
        .build()
        .unwrap();

    let audio_host = audio::Host::new();

    let ring_buffer = RingBuffer::<f32>::new(1024); // Add some latency
    let (producer, consumer) = ring_buffer.split();

    let input_model = InputModel { producer };
    let _input_stream = audio_host
        .new_input_stream(input_model)
        .capture(capture)
        .build()
        .unwrap();

    Model {
        _input_stream,
        consumer,
        time_line_points: vec![],
        frequency_line_points: vec![],
        scale_points: vec![],
    }
}

fn capture(model: &mut InputModel, buffer: &Buffer) {
    for frame in buffer.frames() {
        let mut average = 0.0;
        for sample in frame {
            average += sample;
        }
        average = average / frame.len() as f32;

        let r = model.producer.push(average);
        match r {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}

fn update(_app: &App, model: &mut Model, _udpate: Update) {
    // Collect frames from the input stream
    let mut frames: Vec<f32> = vec![];
    for _ in 0..model.consumer.len() {
        let recorded_sample = match model.consumer.pop() {
            Some(f) => f,
            None => 0.0,
        };
        frames.push(recorded_sample);
    }

    let fft_input: Vec<_> = frames
        .iter()
        .map(|frame| std::convert::From::from(*frame as f64))
        .collect();
    let fft_output = kopek::fft::fft(&fft_input);

    model.time_line_points = utils::get_waveform_graph(&frames, 1000.0);
    model.frequency_line_points = utils::get_frequency_domain_graph(&fft_output, 1.0);
    model.scale_points = utils::get_scale(32);

    // println!("frames count: {}", frames.len());

    std::thread::sleep(std::time::Duration::from_millis(33)); // Roughly set to 30 FPS
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    if model.frequency_line_points.len() == 1024 {
        // Time domain
        draw.polyline()
            .weight(1.0)
            .points(model.time_line_points.clone())
            .color(CRIMSON);
        // ===========

        // Frequency domain wide range
        draw.polyline()
            .weight(1.0)
            .points(model.frequency_line_points.clone())
            .color(GREEN);

        for (i, point) in model.scale_points.iter().enumerate() {
            draw.rect().w_h(1.0, 10.0).xy(*point).color(BLACK);
            let bin_text = i as f32 * consts::BIN_SIZE * (1024 / 32) as f32;
            draw.text(&format!("{:0.0}hz", bin_text))
                .font_size(6)
                .x_y(point.x, -80.0)
                .color(BLACK);
        }
        // ===========================

        // Frequency domain narrow ranges
        let average_bins = utils::get_narrow_bar_spectrum(&model.frequency_line_points);
        // println!(
        //     "0: {:.2} 1: {:.2} 2: {:.2}",
        //     &average_bins[0].y, &average_bins[1].y, &average_bins[2].y
        // );
        for bin in average_bins {
            draw.rect()
                .x_y(bin.x, 0.0)
                .w_h(90.0, 200.0 - bin.y.abs())
                .color(GREEN);
        }

        let spectrum_scale = utils::get_narrow_bar_spectrum_scale();
        for i in 0..spectrum_scale.len() {
            draw.text(&format!("{:0.0}", spectrum_scale[i]))
                .font_size(20)
                .x_y(-462.0 + 100.0 * i as f32, 0.0)
                .color(BLACK);
        }
        // ==============================
    }

    draw.to_frame(app, &frame).unwrap();

    draw.background().color(WHITE);
}
