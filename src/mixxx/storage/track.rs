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
            duration as "duration!",
            bitrate as "bitrate!",
            samplerate as "samplerate!",
            cuepoint,
            bpm as "bpm!",
            channels,
            datetime_added as "datetime_added!",
            mixxx_deleted,
            played,
            -- header_parsed as "header_parsed!",
            filetype as "filetype!",
            replaygain as "replaygain!",
            timesplayed as "timesplayed!",
            rating as "rating!",
            key as "key!",
            beats,
            -- beats_version,
            -- bpm_lock,
            -- beats_sub_version,
            -- keys,
            -- keys_version,
            -- keys_sub_version,
            -- key_id,
            -- grouping as "grouping!",
            -- coverart_source,
            -- coverart_type,
            -- coverart_location,
            -- coverart_hash,
            replaygain_peak as "replaygain_peak!",
            -- tracktotal,
            --color,
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
