extern crate kopek;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use ggez::event::{self, EventHandler};
use ggez::{graphics, nalgebra, Context, ContextBuilder, GameResult};

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("kopek_test", "zehreken")
        .build()
        .expect("Could not create ggez context!");

    graphics::set_window_title(&ctx, "kopek_test");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut game = Game::new(&mut ctx);

    fft_test();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

fn fft_test() {
    let frames = kopek::decoder::decode("sine_440hz_stereo.ogg");
    let input: Vec<_> = frames[..128]
        .iter()
        .map(|frame| num::Complex::from(frame[0] as f64 / std::i16::MAX as f64))
        .collect();

    kopek::fft::show("input: ", &input);
    let output = kopek::fft::fft(&input);
    kopek::fft::show("output: ", &output);
}

struct Game {
    line: graphics::Mesh,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        let frames = kopek::decoder::decode("sine_440hz_stereo.ogg");
        let mut x = 0;
        let points: Vec<nalgebra::Point2<f32>> = frames
            .iter()
            .step_by(100)
            .map(|frame| {
                x = x + 1;
                nalgebra::Point2::new(x as f32, 300.0 + (frame[0] as f32) / 500.0)
            })
            .collect();

        println!("{}", points.len());

        let mut mesh_builder = graphics::MeshBuilder::new();

        let line = mesh_builder
            .line(&points[..], 1.0, graphics::Color::from_rgb(255, 0, 55))
            .unwrap()
            .build(ctx)
            .unwrap();

        Game { line }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        graphics::draw(ctx, &self.line, graphics::DrawParam::default()).unwrap();

        graphics::present(ctx)
    }
}

fn test_ogg() {
    let domi_frames = kopek::decoder::decode("domi.ogg");

    let mut cycling = domi_frames.iter().cloned().cycle();

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
