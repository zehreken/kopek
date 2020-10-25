extern crate kopek;
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
    input_stream: audio::Stream<InputModel>,
    consumer: Consumer<f32>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1100, 600)
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
    let mut frames = vec![];
    for _ in 0..model.consumer.len() {
        let recorded_sample = match model.consumer.pop() {
            Some(f) => f,
            None => 0.0,
        };
        frames.push(recorded_sample);
    }

    println!("frames count: {}", frames.len());

    std::thread::sleep(std::time::Duration::from_millis(33));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
}
