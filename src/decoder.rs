pub fn decode<P>(path: P) -> Vec<[i16; 2]>
where
    P: AsRef<std::path::Path>,
{
    let mut file = audrey::open(path).expect("Error while opening file");
    let frames = file
        .frames::<[i16; 2]>()
        .map(Result::unwrap)
        .collect::<Vec<[i16; 2]>>();

    frames
}

pub fn detect_bpm(frames: Vec<[i16; 2]>) -> u32 {
    const C: f32 = 1.3;
    let f_frames: Vec<[f32; 2]> = frames
        .iter()
        .map(|f| [f[0] as f32 / i16::MAX as f32, f[1] as f32 / i16::MAX as f32])
        .collect();

    let E: f32 = (0..44032)
        .map(|i| f_frames[i][0].powf(2.0) + f_frames[i][1].powf(2.0))
        .sum();
    let E = E * (1024.0 / 44100.0);

    let mut beats = 0;
    for i in 0..43 {
        let (start, end) = (i * 1024, (i + 1) * 1024);
        let e: f32 = (start..end)
            .map(|i| f_frames[i][0].powf(2.0) + f_frames[i][1].powf(2.0))
            .sum();

        if e > C * E {
            beats += 1;
        }
    }

    beats
}
