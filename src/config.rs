use std::path::{Path, PathBuf};

use anyhow::Result;
use confique::Config as Confique;
use log::{debug, warn};
use serde_derive::Deserialize;
use shellexpand::tilde;

#[derive(Confique)]
pub struct Config {
    /// The exact location of the mixxxdb.sqlite file.
    pub mixxx_db: String,
    /// The directory to which the library will be exported to.
    target_directory: PathBuf,

    /// This program only works, if all music files are located in a single directory.
    /// This directory should then
    source_library_root: PathBuf,
    /// The
    target_library_root: PathBuf,

    /// The filesystem of the target system on which rekordbox will be executed.
    /// We need this to build the correct path format, which differs between different OSs.
    target_filesystem: TargetFs,
}

#[derive(Deserialize, Debug)]
pub enum TargetFs {
    Windows,
    Unix,
}

/// Little helper which expands a given path's `~` characters to a fully qualified path.
pub fn expand_home(old_path: &Path) -> PathBuf {
    PathBuf::from(tilde(&old_path.to_string_lossy()).into_owned())
}

impl Config {
    pub fn target_directory(&self) -> PathBuf {
        expand_home(&self.target_directory)
    }
}

impl Config {
    pub fn read() -> Result<Self> {
        // Read the config from the environment by default.
        // Also read from the configuration, if we find a config directory.
        let mut config_builder = Self::builder();
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("mixxx_library_exporter.yml");
            debug!("Looking vor config at: {config_path:?}");
            if !config_path.exists() {
                warn!("Couldn't find config at path: {config_path:?}!");
            } else {
                config_builder = config_builder.file(&config_path);
            }
        }

        let config = config_builder.load()?;

        Ok(config)
    }
}
