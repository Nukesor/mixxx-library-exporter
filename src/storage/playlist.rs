use anyhow::{Context, Result};
use sqlx::SqliteConnection;

use crate::models::playlist::Playlist;

pub async fn get_playlists(con: &mut SqliteConnection) -> Result<Vec<Playlist>> {
    let playlists = sqlx::query_as!(
        Playlist,
        r#"
        SELECT
            id,
            name,
            position,
            hidden,
            date_created,
            date_modified
        FROM Playlists
        "#,
    )
    .fetch_all(con)
    .await
    .context("Failed to get playlists")?;

    Ok(playlists)
}

pub async fn get_playlist_tracks(con: &mut SqliteConnection, playlist_id: i64) -> Result<Vec<i64>> {
    let track_ids = sqlx::query!(
        r#"
        SELECT track_id AS "track_id!"
        FROM PlaylistTracks
        WHERE playlist_id = $1
        ORDER BY position ASC
        "#,
        playlist_id
    )
    .fetch_all(con)
    .await
    .context(format!("Failed to get tracks for playlist {playlist_id}"))?;

    // Get the actual integers from the returned record structs.
    let track_ids = track_ids
        .into_iter()
        .map(|record| record.track_id)
        .collect();

    Ok(track_ids)
}
