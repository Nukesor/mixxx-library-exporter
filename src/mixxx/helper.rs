/// In mixxx, track positions are saved as absolute `actual_timestamp_seconds * sample_rate`.
/// For some reason, this doesn't always use the sample_rate of the track, but rather 88_200 Hz.
/// TODO: Find out how the sample rate is determined!
///
/// To get the actual position, we just devide by the sample_rate.
pub fn convert_mixxx_position(cue_position: f64, sample_rate: i64) -> f64 {
    cue_position / sample_rate as f64
}
