use serde_derive::{Deserialize, Serialize};

/// This is the playlists tag that contains all playlist nodes.
#[derive(Serialize, Deserialize, Debug)]
pub struct Playlists {
    #[serde(rename = "NODE")]
    playlists_root: PlaylistsRoot,
}

impl Playlists {
    /// Create a new node with the type of PlaylistRoot
    pub fn new(playlists: Vec<Playlist>) -> Self {
        Playlists {
            playlists_root: PlaylistsRoot::new(playlists),
        }
    }
}

/// This is the representation of the "root folder".
/// This node must always exist and is the very first node.
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistsRoot {
    #[serde(rename = "@Type")]
    r#type: u32,
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Count")]
    count: usize,
    #[serde(rename = "NODE")]
    playlists: Vec<Playlist>,
}

impl PlaylistsRoot {
    /// Create a new node with the type of PlaylistRoot
    pub fn new(playlists: Vec<Playlist>) -> Self {
        PlaylistsRoot {
            r#type: 0,
            name: "ROOT".into(),
            count: playlists.len(),
            playlists,
        }
    }
}

/// The representation of a single Playlist.
/// A playlist contains multiple `TRACK` elements with TrackIds that point to actual tracks.
#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    #[serde(rename = "@Type")]
    r#type: u32,
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@KeyType")]
    key_type: u32,
    #[serde(rename = "@Entries")]
    entries: usize,
    #[serde(rename = "TRACK")]
    tracks: Vec<PlaylistTrack>,
}

impl Playlist {
    /// Create a new node with the type of Playlist
    pub fn new(name: String, tracks: Vec<PlaylistTrack>) -> Self {
        Playlist {
            r#type: 1,
            name,
            key_type: 0,
            entries: tracks.len(),
            tracks,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistTrack {
    #[serde(rename = "@Key")]
    key: usize,
}

impl PlaylistTrack {
    /// Create a new track entry used inside a Playlist declaration.
    /// The `key` argument is the id that's used in the respective track in the Collection.tracks.
    pub fn new(key: usize) -> Self {
        PlaylistTrack { key }
    }
}
