pub fn decode<P>(path: P) -> Vec<[i16; 2]>
where
    P: AsRef<std::path::Path>,
{
    let file_path = path.as_ref().display().to_string();
    let mut file = audrey::open(path).expect("Error while opening file");
    println!("{} -> {:?}", file_path, file.description());
    let channel_count = file.description().channel_count();
    let frames: Vec<[i16; 2]>;
    match channel_count {
        1 => {
            let mono_frames = file
                .frames::<[i16; 1]>()
                .map(Result::unwrap)
                .collect::<Vec<[i16; 1]>>();

            frames = mono_frames
                .iter()
                .map(|f| [f[0], f[0]])
                .collect::<Vec<[i16; 2]>>();
        }
        _ => {
            frames = file
                .frames::<[i16; 2]>()
                .map(Result::unwrap)
                .collect::<Vec<[i16; 2]>>();
        }
    }

    frames
}

// Assumes that sample rate is always 44100 Hz
pub fn get_duration_in_seconds(frames: Vec<[i16; 2]>) -> u32 {
    frames.len() as u32 / 44100
}

pub fn detect_bpm(frames: Vec<[i16; 2]>) -> u32 {
    const C: f32 = 1.5;
    let f_frames: Vec<[f32; 2]> = frames
        .iter()
        .map(|f| [f[0] as f32 / i16::MAX as f32, f[1] as f32 / i16::MAX as f32])
        .collect();

    let factor = 16;
    let mut beats = 0;
    for i in 0..factor {
        let start = i * 44032;
        let end = (i + 1) * 44032;
        let E: f32 = (start..end)
            .map(|i| f_frames[i][0].powf(2.0) + f_frames[i][1].powf(2.0))
            .sum();
        let E = E * (1024.0 / 44100.0);
        let frame_slice = &f_frames[start..end];

        for i in 0..43 {
            let (start, end) = (i * 1024, (i + 1) * 1024);
            let e: f32 = (start..end)
                .map(|i| frame_slice[i][0].powf(2.0) + frame_slice[i][1].powf(2.0))
                .sum();

            if e > C * E {
                beats += 1;
            }
        }
    }

    beats * 15 / factor as u32
}
