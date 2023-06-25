use serde_derive::{Deserialize, Serialize};

/// This is the parent object that contains all tracks in this library.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "COLLECTION")]
pub struct Tracks {
    #[serde(rename = "@Entries")]
    entries: usize,
    #[serde(rename = "TRACK")]
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
pub struct Track {
    #[serde(rename = "@TrackID")]
    pub track_id: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Artist")]
    pub artist: String,
    #[serde(rename = "@Composer")]
    pub composer: String,
    #[serde(rename = "@Album")]
    pub album: String,
    #[serde(rename = "@Grouping")]
    pub grouping: String,
    #[serde(rename = "@Genre")]
    pub genre: String,
    #[serde(rename = "@Kind")]
    pub kind: TrackKind,
    #[serde(rename = "@Size")]
    pub size: String,
    #[serde(rename = "@TotalTime")]
    pub total_time: u32,
    #[serde(rename = "@DiscNumber")]
    pub disc_number: u32,
    #[serde(rename = "@TrackNumber")]
    pub track_number: String,
    #[serde(rename = "@Year")]
    pub year: String,
    #[serde(rename = "@AverageBpm")]
    pub average_bpm: String,
    #[serde(rename = "@DateAdded")]
    pub date_added: String,
    #[serde(rename = "@BitRate")]
    pub bit_rate: i64,
    #[serde(rename = "@SampleRate")]
    pub sample_rate: i64,
    #[serde(rename = "@Comments")]
    pub comments: String,
    #[serde(rename = "@PlayCount")]
    pub play_count: i64,
    #[serde(rename = "@Rating")]
    pub rating: i64,
    #[serde(rename = "@Location")]
    pub location: String,
    #[serde(rename = "@Remixer")]
    pub remixer: String,
    #[serde(rename = "@Tonality")]
    pub tonality: String,
    #[serde(rename = "@Label")]
    pub label: String,
    #[serde(rename = "@Mix")]
    pub mix: String,

    // There can be multiple `Cue` entries, which is why we have to declare
    // the content of a Track like this.
    // This notation allows to have multiple tags with the same name as a child.
    #[serde(rename = "$value")]
    pub values: Vec<TrackContent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TrackContent {
    #[serde(rename = "TEMPO")]
    Tempo(Tempo),
    #[serde(rename = "POSITION_MARK")]
    Cue(Cue),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tempo {
    #[serde(rename = "@Inizio")]
    pub inizio: f64,
    #[serde(rename = "@Bpm")]
    pub bpm: u32,
    #[serde(rename = "@Metro")]
    pub metro: String,
    #[serde(rename = "@Battito")]
    pub battito: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cue {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub r#type: u32,
    #[serde(rename = "@Start")]
    pub start: f64,
    #[serde(rename = "@Num")]
    pub num: i64,
    #[serde(rename = "@Red")]
    pub red: u8,
    #[serde(rename = "@Green")]
    pub green: u8,
    #[serde(rename = "@Blue")]
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
