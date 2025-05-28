pub mod decoder;
pub mod envelope;
pub mod fft;
pub mod metronome;
pub mod noise;
pub mod noise_generator;
pub mod oscillator;
pub mod time_signature;
pub mod utils;

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
        assert_eq!(2.0, duration_in_seconds(frames, 44100.0))
    }

    #[test]
    fn detect_bpm_test() {
        let frames = decode(PATH);
        let bpm = detect_bpm(frames, 44100.0);
        assert_eq!(83, bpm);
    }
}
