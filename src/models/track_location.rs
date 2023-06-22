use std::path::PathBuf;

use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct TrackLocation {
    pub id: u64,
    pub location: PathBuf,
    pub filename: String,
    pub directory: PathBuf,
}
