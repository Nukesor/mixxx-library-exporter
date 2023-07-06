use std::collections::BTreeMap;
use std::io::Cursor;

use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use prost::Message;
use serde_derive::{Deserialize, Serialize};

use crate::mixxx::helper::convert_mixxx_position;

use super::schema::beats::BeatGrid;
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
    pub location: String,
    pub filename: String,
    pub directory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackTechnicalInfo {
    pub duration: f64,
    pub bitrate: i64,
    pub samplerate: i64,
    pub bpm: f64,
    pub beats: Option<Vec<u8>>,
    pub beats_version: Option<String>,
    pub key: String,
    pub replaygain: f64,
    pub replaygain_peak: f64,
    pub source_synchronized_ms: Option<i64>,
}

impl TrackTechnicalInfo {
    pub fn get_start_of_beatgrid(&self) -> Result<Option<f64>> {
        if let Some(bytes) = &self.beats {
            let grid = BeatGrid::decode(&mut Cursor::new(bytes))
                .context("Failed to decode beatgrid info")?;

            if let Some(beat) = &grid.first_beat {
                if let Some(position) = beat.frame_position {
                    // Get the adjusted beatgrid position
                    let mut position = convert_mixxx_position(position.into(), self.samplerate);

                    let beat_length = 60.0 / self.bpm;

                    // Mixxx tends to use negative numbers to indicate the first beat.
                    // Rekordbox however doesn't do this, which is why we have to adjust this.
                    if position.is_sign_negative() {
                        // Simply add one beat to get to the start of first beat that's actually
                        // inside of the track.
                        position += beat_length;
                    }

                    // For some reason, Mixxx sometimes refers to the second beat that's inside
                    // the track. Subtract a beat, if that's the case.
                    if position > beat_length {
                        position -= beat_length;
                    }

                    return Ok(Some(position));
                }
            }
        }

        Ok(None)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackMetadata {
    pub rating: i64,
    pub played: bool,
    pub timesplayed: i64,
    pub deleted: bool,
    pub datetime_added: NaiveDateTime,
    pub last_played_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub id: usize,
    pub name: String,
    pub position: usize,
    pub hidden: bool,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
    pub track_ids: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crate {
    pub id: usize,
    pub name: String,
    pub count: i64,
    pub hidden: bool,
    pub track_ids: Vec<usize>,
}
