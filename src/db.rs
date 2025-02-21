use anyhow::Result;
use sqlx::{Connection, sqlite::SqliteConnection};

pub async fn new_connection(db_path: &str) -> Result<SqliteConnection> {
    let conn = SqliteConnection::connect(db_path).await?;

    Ok(conn)
}
