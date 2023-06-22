use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Cue {
    pub id: i64,
    pub track_id: i64,
    pub r#type: i64,
    pub position: i64,
    pub length: i64,
    pub hotcue: i64,
    pub label: String,
    pub color: i64,
}
