use anyhow::{Context, Result};
use sqlx::SqliteConnection;

use crate::mixxx::schema::cue::Cue;

pub async fn get_track_cues(con: &mut SqliteConnection, track_id: i64) -> Result<Vec<Cue>> {
    let cues = sqlx::query_as!(
        Cue,
        r#"
        SELECT
            id,
            track_id,
            type as cue_type,
            position,
            length,
            hotcue,
            label,
            color
        FROM cues
        WHERE track_id = $1
        "#,
        track_id
    )
    .fetch_all(con)
    .await
    .context(format!("Failed to get queues for track {track_id}"))?;

    Ok(cues)
}
