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
