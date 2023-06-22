use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Crate {
    pub id: i64,
    pub name: String,
    pub count: Option<i64>,
    pub show: Option<i64>,
}
