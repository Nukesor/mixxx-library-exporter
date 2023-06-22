use std::{collections::BTreeMap, path::PathBuf};

use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

use super::schema::cue::Cue;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Library {
    pub tracks: BTreeMap<usize, Track>,
    pub playlists: Vec<Playlist>,
    pub crates: Vec<Crate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub id: usize,
    pub artist: String,
    pub composer: String,
    pub title: String,
    pub album: Option<String>,
    pub year: String,
    pub genre: Option<String>,
    pub tracknumber: Option<String>,
    pub filetype: String,
    pub comment: Option<String>,
    pub url: Option<String>,
    pub location: TrackLocation,
    pub technical_info: TrackTechnicalInfo,
    pub metadata: TrackMetadata,
    pub cues: Vec<Cue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackLocation {
    pub location: PathBuf,
    pub filename: String,
    pub directory: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackTechnicalInfo {
    pub duration: f64,
    pub bitrate: f64,
    pub samplerate: f64,
    pub bpm: f64,
    pub key: String,
    pub replaygain: f64,
    pub replaygain_peak: f64,
    pub source_synchronized_ms: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackMetadata {
    pub rating: i64,
    pub played: bool,
    pub timesplayed: i64,
    pub deleted: bool,
    pub datetime_added: String,
    pub last_played_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub position: usize,
    pub hidden: bool,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
    pub track_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crate {
    pub id: i64,
    pub name: String,
    pub count: i64,
    pub hidden: bool,
    pub track_ids: Vec<i64>,
}
