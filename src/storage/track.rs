use anyhow::{Context, Result};
use sqlx::SqliteConnection;

use crate::models::track::{Track, TrackLocation};

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
            datetime_added
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
