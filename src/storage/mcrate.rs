use anyhow::{Context, Result};
use sqlx::SqliteConnection;

use crate::models::mcrate::Crate;

pub async fn get_crates(con: &mut SqliteConnection) -> Result<Vec<Crate>> {
    let crates = sqlx::query_as!(
        Crate,
        r#"
        SELECT
            id,
            name,
            count,
            show
        FROM crates
        "#,
    )
    .fetch_all(con)
    .await
    .context("Failed to get crates")?;

    Ok(crates)
}

pub async fn get_crate_tracks(con: &mut SqliteConnection, crate_id: i64) -> Result<Vec<i64>> {
    let track_ids = sqlx::query!(
        r#"
        SELECT track_id
        FROM crate_tracks
        WHERE crate_id = $1
        "#,
        crate_id
    )
    .fetch_all(con)
    .await
    .context(format!("Failed to get tracks for crate {crate_id}"))?;

    // Get the actual integers from the returned record structs.
    let track_ids = track_ids
        .into_iter()
        .map(|record| record.track_id)
        .collect();

    Ok(track_ids)
}
