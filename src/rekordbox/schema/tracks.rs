use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "COLLECTION", rename_all = "camelCase")]
pub struct Tracks {
    entries: usize,
    tracks: Vec<Track>,
}

impl Tracks {
    pub fn new(tracks: Vec<Track>) -> Self {
        Tracks {
            entries: tracks.len(),
            tracks,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "TRACK", rename_all = "camelCase")]
pub struct Track {
    #[serde(rename = "TrackID")]
    track_id: String,
    name: String,
    artist: String,
    composer: String,
    album: String,
    grouping: String,
    genre: String,
    kind: String,
    size: String,
    total_time: String,
    disc_number: String,
    track_number: String,
    year: String,
    average_bpm: String,
    date_added: String,
    bit_rate: String,
    sample_rate: String,
    comments: String,
    play_count: String,
    rating: String,
    location: String,
    remixer: String,
    tonality: String,
    label: String,
    mix: String,
}
