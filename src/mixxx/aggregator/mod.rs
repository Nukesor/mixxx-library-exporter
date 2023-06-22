use std::collections::BTreeMap;

use anyhow::Result;
use sqlx::SqliteConnection;

use super::library::{Crate, Library, Playlist, Track};

pub async fn read_library(con: &mut SqliteConnection) -> Result<Library> {
    let tracks = read_tracks(con).await?;
    let playlists = read_playlists(con, &tracks).await?;
    let crates = read_crates(con, &tracks).await?;

    let library = Library {
        tracks,
        playlists,
        crates,
    };

    Ok(library)
}

pub async fn read_tracks(con: &mut SqliteConnection) -> Result<BTreeMap<usize, Track>> {
    let tracks = BTreeMap::new();

    Ok(tracks)
}

pub async fn read_playlists(
    con: &mut SqliteConnection,
    tracks: &BTreeMap<usize, Track>,
) -> Result<Vec<Playlist>> {
    let playlists = Vec::new();

    Ok(playlists)
}

pub async fn read_crates(
    con: &mut SqliteConnection,
    tracks: &BTreeMap<usize, Track>,
) -> Result<Vec<Crate>> {
    let crates = Vec::new();

    Ok(crates)
}
