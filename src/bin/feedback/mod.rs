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
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1100, 600)
        .title("kopek")
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

    Model { input_stream }
}

fn capture(model: &mut InputModel, buffer: &Buffer) {
    for frame in buffer.frames() {
        for sample in frame {
            model.producer.push(*sample).unwrap();
        }
    }
}

fn update(app: &App, model: &mut Model, _udpate: Update) {}
