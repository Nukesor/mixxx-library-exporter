use std::collections::BTreeMap;

use anyhow::{Context, Result};
use chrono::DateTime;
use sqlx::SqliteConnection;

use super::library::{
    Crate, Library, Playlist, Track, TrackLocation, TrackMetadata, TrackTechnicalInfo,
};
use crate::mixxx::storage;

pub async fn read_library(con: &mut SqliteConnection) -> Result<Library> {
    let tracks = get_tracks(con).await?;
    let playlists = read_playlists(con, &tracks).await?;
    let crates = read_crates(con, &tracks).await?;

    let library = Library {
        tracks,
        playlists,
        crates,
    };

    Ok(library)
}

pub async fn get_tracks(con: &mut SqliteConnection) -> Result<BTreeMap<usize, Track>> {
    let mut tracks = BTreeMap::new();

    let raw_tracks = storage::track::get_tracks(con).await?;
    for raw_track in raw_tracks.into_iter() {
        let location = storage::track::get_track_location(con, raw_track.id).await?;
        let cues = storage::cue::get_track_cues(con, raw_track.id).await?;

        let location = TrackLocation {
            location: location.location.unwrap(),
            filename: location.filename.unwrap(),
            directory: location.directory.unwrap(),
        };

        let technical_info = TrackTechnicalInfo {
            duration: raw_track.duration,
            bitrate: raw_track.bitrate,
            samplerate: raw_track.samplerate,
            bpm: raw_track.bpm,
            beats: raw_track.beats,
            beats_version: raw_track.beats_version,
            key: raw_track.key,
            replaygain: raw_track.replaygain,
            replaygain_peak: raw_track.replaygain_peak,
            source_synchronized_ms: raw_track.source_synchronized_ms,
        };

        // This particular field is returned as a string rather than as a Datetime.
        // We have to parse it ourselve.
        let datetime_added = DateTime::parse_from_rfc3339(&raw_track.datetime_added)
            .context(format!(
                "Failed to parse datetime_added: {}",
                raw_track.datetime_added
            ))?
            .naive_local();

        let metadata = TrackMetadata {
            rating: raw_track.rating,
            // Played, if the value is bigger than 0
            played: raw_track.played.unwrap_or_default() > 0,
            timesplayed: raw_track.timesplayed,
            deleted: raw_track.mixxx_deleted.unwrap_or_default() > 0,
            datetime_added,
            last_played_at: raw_track.last_played_at,
        };

        let track = Track {
            id: raw_track
                .id
                .try_into()
                .context("Failed to convert id into usize")?,
            artist: raw_track.artist.unwrap_or_default(),
            composer: raw_track.composer,
            title: raw_track.title.unwrap(),
            album: raw_track.album,
            year: raw_track.year.unwrap_or_default(),
            genre: raw_track.genre,
            tracknumber: raw_track.tracknumber,
            filetype: raw_track.filetype,
            comment: raw_track.comment,
            url: raw_track.url,
            cues,
            location,
            technical_info,
            metadata,
        };

        tracks.insert(track.id, track);
    }

    Ok(tracks)
}

pub async fn read_playlists(
    con: &mut SqliteConnection,
    _tracks: &BTreeMap<usize, Track>,
) -> Result<Vec<Playlist>> {
    let mut playlists = Vec::new();

    let raw_playlists = storage::playlist::get_playlists(con).await?;
    for raw_playlist in raw_playlists.into_iter() {
        let track_ids: Vec<usize> = storage::playlist::get_playlist_tracks(con, raw_playlist.id)
            .await?
            .into_iter()
            .map(|id| {
                id.try_into()
                    .expect("Got track id that doesn't fit into usize")
            })
            .collect();

        let playlist = Playlist {
            id: raw_playlist
                .id
                .try_into()
                .expect("Got playlist id that doesn't fit into usize"),
            name: raw_playlist.name.unwrap(),
            position: raw_playlist.position.unwrap().try_into().unwrap(),
            hidden: raw_playlist.hidden > 0,
            date_created: raw_playlist.date_created.unwrap(),
            date_modified: raw_playlist.date_modified.unwrap(),
            track_ids,
        };

        playlists.push(playlist);
    }

    Ok(playlists)
}

pub async fn read_crates(
    con: &mut SqliteConnection,
    _tracks: &BTreeMap<usize, Track>,
) -> Result<Vec<Crate>> {
    let mut crates = Vec::new();

    let raw_crates = storage::mcrate::get_crates(con).await?;
    for raw_crate in raw_crates.into_iter() {
        let track_ids: Vec<usize> = storage::mcrate::get_crate_tracks(con, raw_crate.id)
            .await?
            .into_iter()
            .map(|id| {
                id.try_into()
                    .expect("Got track id that doesn't fit into usize")
            })
            .collect();

        let mcrate = Crate {
            id: raw_crate
                .id
                .try_into()
                .expect("Got crate id that doesn't fit into usize"),
            name: raw_crate.name,
            hidden: !(raw_crate.show.unwrap() > 0),
            count: raw_crate.count.unwrap(),
            track_ids,
        };

        crates.push(mcrate);
    }

    Ok(crates)
}
