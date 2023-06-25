use serde_derive::{Deserialize, Serialize};

/// This is the parent object that contains all tracks in this library.
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

/// A single track in the library.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "TRACK", rename_all = "camelCase")]
pub struct Track {
    #[serde(rename = "TrackID")]
    pub track_id: String,
    pub name: String,
    pub artist: String,
    pub composer: String,
    pub album: String,
    pub grouping: String,
    pub genre: String,
    pub kind: TrackKind,
    pub size: String,
    pub total_time: u32,
    pub disc_number: u32,
    pub track_number: String,
    pub year: String,
    pub average_bpm: String,
    pub date_added: String,
    pub bit_rate: i64,
    pub sample_rate: i64,
    pub comments: String,
    pub play_count: i64,
    pub rating: i64,
    pub location: String,
    pub remixer: String,
    pub tonality: String,
    pub label: String,
    pub mix: String,

    // There can be multiple `Cue` entries, which is why we have to declare
    // the content of a Track like this.
    // This notation allows to have multiple tags with the same name as a child.
    #[serde(rename = "$value")]
    pub values: Vec<TrackContent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TrackContent {
    Tempo(Tempo),
    Cue(Cue),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "TEMPO", rename_all = "camelCase")]
pub struct Tempo {
    pub inizio: f64,
    pub bpm: u32,
    pub metro: String,
    pub battito: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "POSITION_MARK", rename_all = "camelCase")]
pub struct Cue {
    pub name: String,
    pub r#type: u32,
    pub start: f64,
    pub num: i64,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Cue {
    pub fn new(name: String, start: f64, num: i64) -> Self {
        Cue {
            name,
            r#type: 0,
            start,
            num,
            red: 40,
            green: 226,
            blue: 20,
        }
    }
}

/// A string describing the type of audio file.
#[derive(Serialize, Deserialize, Debug)]
pub enum TrackKind {
    #[serde(rename = "MP3 File")]
    Mp3,
}

impl TrackKind {
    pub fn from_string(filetype: String) -> TrackKind {
        if filetype == "mp3" {
            return TrackKind::Mp3;
        }

        panic!("Found unknown filetype {filetype} in Mixxx database");
    }
}

/// Many programs use something called the Camelot wheel to categorize tonal keys.
/// Rekordbox uses a different notation, which we have to map to.
///
/// A = 11B
/// Ab/G# = 4B
/// Am = 8A
/// B = 1B
/// Bb = 6B
/// Bbm/A#m = 3A
/// Bm = 10A
/// C = 8B
/// C#m/Dbm = 12A
/// Cm = 5A
/// D = 10B
/// Db = 3B
/// Dm = 7A
/// E = 12B
/// Eb/D# = 5B
/// Ebm = 2A
/// Em = 9A
/// F = 7B
/// F#m = 11A
/// Fm = 4A
/// G = 9B
/// G#m/Abm = 1A
/// Gb/F# = 2B
/// Gm = 6A
pub fn translate_key(_mixxx_key: &str) -> &'static str {
    "Am"
}
