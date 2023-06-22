use anyhow::{Context, Result};
use sqlx::SqliteConnection;

use crate::mixxx::schema::track::{Track, TrackLocation};

pub async fn get_tracks(con: &mut SqliteConnection) -> Result<Vec<Track>> {
    let tracks = sqlx::query_as!(
        Track,
        r#"
        SELECT
            id,
            artist,
            title,
            album,
            year,
            genre,
            composer as "composer!",
            tracknumber,
            location,
            comment,
            url,
            duration,
            bitrate,
            samplerate,
            cuepoint,
            bpm,
            channels,
            datetime_added as "datetime_added!",
            mixxx_deleted,
            played,
            header_parsed as "header_parsed!",
            filetype as "filetype!",
            replaygain as "replaygain!",
            timesplayed as "timesplayed!",
            rating as "rating!",
            key as "key!",
            -- //pub beats: BLOB,
            -- //pub beats_version: TEXT,
            -- //pub bpm_lock: i64,
            -- //pub beats_sub_version: String,
            -- //pub keys: Vec<u8>,
            -- //pub keys_version: String,
            -- //pub keys_sub_version: String,
            -- //pub key_id: i64,
            grouping as "grouping!",
            album_artist as "album_artist!",
            -- //pub coverart_source: INTEGER DEFAULT 0,
            -- //pub coverart_type: INTEGER DEFAULT 0,
            -- //pub coverart_location: TEXT DEFAULT "",
            -- //pub coverart_hash: INTEGER DEFAULT 0,
            replaygain_peak as "replaygain_peak!",
            -- //pub tracktotal: TEXT DEFAULT '//',
            color,
            last_played_at,
            source_synchronized_ms
        FROM library
        "#
    )
    .fetch_all(con)
    .await?;

    Ok(tracks)
}

pub async fn get_track_location(con: &mut SqliteConnection, id: i64) -> Result<TrackLocation> {
    let location = sqlx::query_as!(
        TrackLocation,
        r#"
        SELECT
            id,
            location,
            filename,
            directory
        FROM track_locations
        WHERE
            id = $1
        "#,
        id
    )
    .fetch_one(con)
    .await
    .context(format!("Failed to get location for id {id}"))?;

    Ok(location)
}
