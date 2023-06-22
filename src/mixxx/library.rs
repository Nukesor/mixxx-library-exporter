use std::{collections::BTreeMap, path::PathBuf};

use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Library {
    pub tracks: BTreeMap<usize, Track>,
    pub playlists: Vec<Playlist>,
    pub crates: Vec<Crate>,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: usize,
    pub artist: String,
    pub title: String,
    pub album: Option<String>,
    pub year: String,
    pub genre: Option<String>,
    pub tracknumber: Option<String>,
    pub comment: Option<String>,
    pub url: Option<String>,
    pub datetime_added: NaiveDateTime,
    pub location: TrackLocation,
    pub metadata: TrackMetadata,
}

#[derive(Debug, Clone)]
pub struct TrackLocation {
    pub location: PathBuf,
    pub filename: String,
    pub directory: PathBuf,
}

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub duration: f64,
    pub bitrate: i64,
    pub samplerate: i64,
    pub bpm: f64,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub position: usize,
    pub hidden: bool,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
    pub tracks_ids: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct Crate {
    pub id: i64,
    pub name: String,
    pub count: i64,
    pub hidden: bool,
    pub tracks_ids: Vec<i64>,
}
