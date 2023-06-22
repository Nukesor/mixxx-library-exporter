use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Track {
    pub id: i64,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub tracknumber: Option<String>,
    pub location: Option<i64>,
    pub comment: Option<String>,
    pub url: Option<String>,
    pub duration: Option<f64>,
    pub bitrate: Option<i64>,
    pub samplerate: Option<i64>,
    pub cuepoint: Option<i64>,
    pub bpm: Option<f64>,
    pub channels: Option<i64>,
    pub datetime_added: Option<String>,
}

#[derive(FromRow, Debug, Clone)]
pub struct TrackLocation {
    pub id: i64,
    pub location: Option<String>,
    pub filename: Option<String>,
    pub directory: Option<String>,
}
