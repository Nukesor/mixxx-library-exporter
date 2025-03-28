use serde_derive::{Deserialize, Serialize};

use super::{playlists::Playlists, tracks::Tracks};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "DJ_PLAYLISTS")]
pub struct Library {
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "PRODUCT")]
    product: Product,
    #[serde(rename = "COLLECTION")]
    tracks: Tracks,
    #[serde(rename = "PLAYLISTS")]
    playlists: Playlists,
}

impl Library {
    pub fn new(tracks: Tracks, playlists: Playlists) -> Self {
        Library {
            version: "1.0.0".into(),
            product: Product::default(),
            tracks,
            playlists,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "@Company")]
    company: String,
}

impl Default for Product {
    fn default() -> Self {
        Product {
            name: "rekordbox".into(),
            version: "6.7.2".into(),
            company: "AlphaTheta".into(),
        }
    }
}
