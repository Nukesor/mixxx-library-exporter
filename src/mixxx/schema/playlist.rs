use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Playlist {
    pub id: i64,
    pub name: Option<String>,
    pub position: Option<i64>,
    pub hidden: i64,
    pub date_created: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
}
