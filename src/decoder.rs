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

pub fn detect_bpm(frames: Vec<[i16; 2]>) -> f32 {
    let f_frames: Vec<[f32; 2]> = frames
        .iter()
        .map(|f| [f[0] as f32 / i16::MAX as f32, f[1] as f32 / i16::MAX as f32])
        .collect();
    let e: f32 = (0..1024)
        .map(|i| f_frames[i][0].powf(2.0) + f_frames[i][1].powf(2.0))
        .sum();

    e as f32
}
