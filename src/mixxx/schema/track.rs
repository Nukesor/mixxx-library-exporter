#![allow(dead_code)]

use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Track {
    pub id: i64,
    pub artist: Option<String>,
    pub composer: String,
    pub title: Option<String>,
    pub album: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub tracknumber: Option<String>,
    pub location: Option<i64>,
    pub comment: Option<String>,
    pub url: Option<String>,
    pub duration: f64,
    pub bitrate: i64,
    pub samplerate: i64,
    pub cuepoint: Option<i64>,
    pub bpm: f64,
    pub channels: Option<i64>,
    pub datetime_added: String,
    pub mixxx_deleted: Option<i64>,
    pub played: Option<i64>,
    //pub header_parsed: i64,
    pub filetype: String,
    pub replaygain: f64,
    pub timesplayed: i64,
    pub rating: i64,
    pub key: String,
    pub beats: Option<Vec<u8>>,
    pub beats_version: Option<String>,
    //pub bpm_lock: i64,
    //pub beats_sub_version: String,
    //pub keys: Vec<u8>,
    //pub keys_version: String,
    //pub keys_sub_version: String,
    //pub key_id: i64,
    //pub grouping: String,
    //pub coverart_source: INTEGER DEFAULT 0,
    //pub coverart_type: INTEGER DEFAULT 0,
    //pub coverart_location: TEXT DEFAULT "",
    //pub coverart_hash: INTEGER DEFAULT 0,
    pub replaygain_peak: f64,
    //pub tracktotal: TEXT DEFAULT '//',
    //pub color: Option<i64>,
}

#[derive(FromRow, Debug, Clone)]
pub struct TrackLocation {
    pub id: i64,
    pub location: Option<String>,
    pub filename: Option<String>,
    pub directory: Option<String>,
}
