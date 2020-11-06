extern crate kopek;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use ringbuf::{Consumer, Producer, RingBuffer};
mod consts;
mod utils;

pub fn start() {
    nannou::app(model).update(update).run();
}

struct InputModel {
    pub producer: Producer<f32>,
}

struct Model {
    input_stream: audio::Stream<InputModel>,
    consumer: Consumer<f32>,
    time_line_points: Vec<Point2>,
    frequency_line_points: Vec<Point2>,
    scale_points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(consts::SCREEN_WIDTH, consts::SCREEN_HEIGHT)
        .title("kopek")
        .view(view)
        .build()
        .unwrap();

    let audio_host = audio::Host::new();

    let ring_buffer = RingBuffer::<f32>::new(1024 * 2); // Add some latency
    let (producer, consumer) = ring_buffer.split();

    let input_model = InputModel { producer };
    let input_stream = audio_host
        .new_input_stream(input_model)
        .capture(capture)
        .build()
        .unwrap();

    Model {
        input_stream,
        consumer,
        time_line_points: vec![],
        frequency_line_points: vec![],
        scale_points: vec![],
    }
}

fn capture(model: &mut InputModel, buffer: &Buffer) {
    for frame in buffer.frames() {
        for sample in frame {
            let r = model.producer.push(*sample);
            match r {
                Ok(_) => (),
                Err(_) => (),
            }
        }
    }
}

fn update(_app: &App, model: &mut Model, _udpate: Update) {
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

    model.time_line_points = get_waveform_graph(&frames);
    model.frequency_line_points = utils::get_frequency_domain_graph(&fft_output, 4.0);
    model.scale_points = utils::get_scale(consts::X_SCALE);

    // println!("frames count: {}", frames.len());

    std::thread::sleep(std::time::Duration::from_millis(33));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.polyline()
        .weight(1.0)
        .points(model.time_line_points.clone())
        .color(CRIMSON);

    // if model.frequency_line_points.len() == 2048 {
    //     draw.polyline()
    //         .weight(1.0)
    //         .points(model.frequency_line_points.clone())
    //         .color(GREEN);
    // }

    if model.frequency_line_points.len() == 2048 {
        let average_bins = get_spectrum(&model.frequency_line_points);
        // draw.polyline().weight(1.0).points(average_bins).color(RED);
        for bin in average_bins {
            draw.rect()
                .x_y(bin.x, -100.0)
                .w_h(90.0, 200.0 - bin.y.abs())
                .color(GREEN);
        }
    }

    // Draw the scales, binsize calculation is probably wrong
    for (i, point) in model.scale_points.iter().enumerate() {
        draw.rect().w_h(1.0, 10.0).xy(*point).color(BLACK);
        let bin_text = i as f32 * consts::BIN_SIZE * consts::X_SCALE * 8.0;
        draw.text(&format!("{:0.0}hz", bin_text))
            .font_size(6)
            .x_y(point.x, -80.0)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();

    draw.background().color(WHITE);
}

fn get_spectrum_analysis() {}

fn get_spectrum(frequency_line_points: &Vec<Point2>) -> Vec<Point2> {
    // implement another view to have non-linear bin sizes
    // e.g. 32-64-125-250-500-1k-2k-4k-8k-16k Hz
    // get half of model.frequency_line_points
    let mut sum = 1;
    let bin_sizes: Vec<i32> = (0..9)
        .map(|i| {
            sum += 2_i32.pow(i);
            sum
        })
        .collect();
    // println!("bin_sizes: {:?}", bin_sizes);
    // After this bin sizes are 4, 4, 8, 16, 32, 64, 128, 256. In total 512 data points, half of frequency_line_points
    let mut bin_averages: Vec<Point2> = vec![];
    let mut start_index = 0;
    for (i, end_index) in bin_sizes.into_iter().enumerate() {
        let sum: f32 = frequency_line_points[start_index as usize..end_index as usize]
            .iter()
            .map(|v| v.y)
            .sum();
        let average = sum / (end_index - start_index) as f32;
        // println!("{} {} average: {}", start_index, end_index, average);
        bin_averages.push(Point2 {
            x: -462.0 + 100.0 * i as f32,
            y: average,
        });
        start_index = end_index;
    }

    bin_averages
}

fn get_waveform_graph(frame_slice: &Vec<f32>) -> Vec<Point2> {
    let mut x = -513;
    let waveform_points = frame_slice
        .iter()
        .step_by(2)
        .map(|frame| {
            x = x + 1;
            Point2 {
                x: x as f32,
                y: 100.0 + frame * 1000.0,
            }
        })
        .collect();

    waveform_points
}
