use anyhow::Result;
use sqlx::SqliteConnection;

use crate::models::track::Track;

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
