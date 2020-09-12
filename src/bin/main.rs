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

    // fft_test();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

fn fft_test() {
    let frames = kopek::decoder::decode("sine_440hz_stereo.ogg");
    let input: Vec<_> = frames[..1024]
        .iter()
        .map(|frame| num::Complex::from(frame[0] as f64 / std::i16::MAX as f64))
        .collect();

    kopek::fft::show("input: ", &input);
    let output = kopek::fft::fft(&input);
    kopek::fft::show("output: ", &output);
}

struct Game {
    time_line: graphics::Mesh,
    frequency_line: graphics::Mesh,
    circles: Vec<graphics::Mesh>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        let paths = [
            "sine_100.ogg",
            "sine_200.ogg",
            "sine_500.ogg",
            "sine_1000.ogg",
        ];
        let sample_size = 4096;
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

        let mut x = 0;
        let points: Vec<nalgebra::Point2<f32>> = frames_sum
            .iter()
            .map(|frame| {
                x = x + 1;
                nalgebra::Point2::new(x as f32, 100.0 + (frame[0] as f32) / 500.0)
            })
            .collect();

        let mut mesh_builder = graphics::MeshBuilder::new();

        let time_line = mesh_builder
            .line(&points[..], 2.0, graphics::Color::from_rgb(255, 0, 55))
            .unwrap()
            .build(ctx)
            .unwrap();

        let input: Vec<_> = frames_sum
            .iter()
            .map(|frame| num::Complex::from(frame[0] as f64 / std::i16::MAX as f64))
            .collect();

        let output = kopek::fft::fft(&input);
        x = 0;
        let points: Vec<nalgebra::Point2<f32>> = output
            .iter()
            .map(|c| {
                x = x + 1;
                nalgebra::Point2::new(
                    x as f32,
                    500.0 - ((c.re as f32).powf(2.0) + (c.im as f32).powf(2.0)).sqrt(),
                )
            })
            .collect();
        let frequency_line = mesh_builder
            .line(&points[..], 2.0, graphics::Color::from_rgb(0, 0, 255))
            .unwrap()
            .build(ctx)
            .unwrap();

        // One pixel is 10Hz, 10 pixel is 100Hz
        let bin_size = 44100.0 / sample_size as f32;
        let points: Vec<nalgebra::Point2<f32>> = (0..80)
            .into_iter()
            .map(|i| nalgebra::Point2::new(10.0 * i as f32, 300.0))
            .collect();

        x = 0;
        let mut circles: Vec<graphics::Mesh> = vec![];
        for point in points {
            circles.push(
                mesh_builder
                    .circle(
                        graphics::DrawMode::fill(),
                        point,
                        2.0,
                        1.0,
                        if x % 5 == 0 {
                            graphics::Color::from_rgb(255, 0, 0)
                        } else {
                            graphics::Color::from_rgb(0, 0, 0)
                        },
                    )
                    .build(ctx)
                    .unwrap(),
            );
            x += 1;
        }
        Game {
            time_line,
            frequency_line,
            circles,
        }
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
        graphics::draw(ctx, &self.time_line, graphics::DrawParam::default()).unwrap();
        graphics::draw(ctx, &self.frequency_line, graphics::DrawParam::default()).unwrap();
        for circle in &self.circles {
            graphics::draw(ctx, circle, graphics::DrawParam::default()).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
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
