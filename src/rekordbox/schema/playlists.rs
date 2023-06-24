use serde_derive::{Deserialize, Serialize};

/// This is the playlists tag that contains all playlist nodes.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "PLAYLISTS")]
pub struct Playlists {
    playlists_root: PlaylistsRoot,
}

impl Playlists {
    /// Create a new node with the type of PlaylistRoot
    pub fn new(playlists_root: PlaylistsRoot) -> Self {
        Playlists { playlists_root }
    }
}

/// This is the representation of the "root folder".
/// This node must always exist and is the very first node.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "NODE", rename_all = "camelCase")]
pub struct PlaylistsRoot {
    r#type: u32,
    name: String,
    count: usize,
    playlists: Vec<Playlist>,
}

impl PlaylistsRoot {
    /// Create a new node with the type of PlaylistRoot
    pub fn new(count: usize, playlists: Vec<Playlist>) -> Self {
        PlaylistsRoot {
            r#type: 0,
            name: "ROOT".into(),
            count,
            playlists,
        }
    }
}

/// The representation of a single Playlist.
/// A playlist contains multiple `TRACK` elements with TrackIds that point to actual tracks.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "NODE", rename_all = "camelCase")]
pub struct Playlist {
    r#type: u32,
    name: String,
    key_type: u32,
    entries: usize,
    tracks: Vec<PlaylistTrack>,
}

impl Playlist {
    /// Create a new node with the type of Playlist
    pub fn new(tracks: Vec<PlaylistTrack>) -> Self {
        Playlist {
            r#type: 1,
            name: "ROOT".into(),
            key_type: 0,
            entries: tracks.len(),
            tracks,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "TRACK", rename_all = "camelCase")]
pub struct PlaylistTrack {
    key: usize,
}

impl PlaylistTrack {
    /// Create a new track entry used inside a Playlist declaration.
    /// The `key` argument is the id that's used in the respective track in the Collection.tracks.
    pub fn new(key: usize) -> Self {
        PlaylistTrack { key }
    }
}
