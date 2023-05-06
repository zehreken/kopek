pub mod decoder;
pub mod fft;
pub mod metronome;
pub mod noise;
pub mod oscillator;
pub mod time_signature;

#[cfg(test)]
mod tests {
    use super::decoder::*;
    const PATH: &str = "sample.wav";
    #[test]
    fn first_frame_test() {
        let frames = decode(PATH);
        assert_eq!([79, 79], frames[0]);
    }

    #[test]
    fn frame_length_test() {
        let frames = decode(PATH);
        assert_eq!(124443, frames.len());
    }

    #[test]
    fn get_duration_in_seconds_test() {
        let frames = decode(PATH);
        assert_eq!(2, get_duration_in_seconds(frames))
    }

    #[test]
    fn detect_bpm_test() {
        let frames = decode(PATH);
        let bpm = detect_bpm(frames);
        assert_eq!(83, bpm);
    }
}
