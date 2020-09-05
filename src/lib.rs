pub mod decoder;

#[cfg(test)]
mod tests {
    use super::decoder::*;
    const PATH: &str = "sine_440hz_stereo.ogg";
    #[test]
    fn first_frame_test() {
        let frames = decode(PATH);
        assert_eq!([119, 120], frames[0]);
    }

    #[test]
    fn frame_length_test() {
        let frames = decode(PATH);
        assert_eq!(44608, frames.len());
    }

    #[test]
    fn detect_bpm_test() {
        let frames = decode(PATH);
        let bpm = detect_bpm(frames);
        assert_eq!(0.0, bpm);
    }
}
