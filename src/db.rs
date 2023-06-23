use anyhow::Result;
use sqlx::{sqlite::SqliteConnection, Connection};

pub async fn new_connection(db_path: &String) -> Result<SqliteConnection> {
    let conn = SqliteConnection::connect(&db_path).await?;

    Ok(conn)
}
