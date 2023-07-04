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
    pub rating: u8,
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
    pub bpm: f64,
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
    #[serde(rename = "flac")]
    Flac,
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
pub fn translate_key(mixxx_key: &str) -> &'static str {
    match mixxx_key {
        "11B" => "A",
        "4B" => "Ab/G#",
        "8A" => "Am",
        "1B" => "B",
        "6B" => "Bb",
        "3A" => "Bbm/A#m",
        "10A" => "Bm",
        "8B" => "C",
        "12A" => "C#m/Dbm",
        "5A" => "Cm",
        "10B" => "D",
        "3B" => "Db",
        "7A" => "Dm",
        "12B" => "E",
        "5B" => "Eb/D#",
        "2A" => "Ebm",
        "9A" => "Em",
        "7B" => "F",
        "11A" => "F#m",
        "4A" => "Fm",
        "9B" => "G",
        "1A" => "G#m/Abm",
        "2B" => "Gb/F#",
        "6A" => "Gm",
        _ => "",
    }
}

/// Rekordbox uses a byte based rating format.
pub fn translate_rating(rating: i64) -> u8 {
    match rating {
        0 => 0,
        1 => 51,
        2 => 102,
        3 => 153,
        4 => 204,
        5 => 255,
        _ => {
            if rating > 5 {
                255
            } else {
                0
            }
        }
    }
}
