use anyhow::Result;
use sqlx::{sqlite::SqliteConnection, Connection};

pub async fn new_connection() -> Result<SqliteConnection> {
    let conn =
        SqliteConnection::connect("file:///home/nuke/Syncthing/Mixxx/mixxx_data/mixxxdb.sqlite")
            .await?;

    Ok(conn)
}
