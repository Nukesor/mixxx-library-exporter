use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use confique::Config as Confique;
use log::{debug, warn};
use shellexpand::tilde;

#[derive(Confique)]
pub struct Config {
    /// The exact location of the mixxxdb.sqlite file.
    pub mixxx_db: String,
    /// The directory to which the library will be exported to.
    target_directory: PathBuf,

    /// This program only works, if all music files are located in a single directory.
    /// This directory should then also be available in the target OS
    /// For Linux this would look something like "/home/your_user/Mixxx"
    pub source_library_root: String,
    /// This program only works, if all music files are located in a single directory.
    /// This is the path to the library root in the target OS.
    /// For Windows this would look something like "C:/Users/your_user/Music/Mixxx"
    pub target_library_root: PathBuf,
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
        let mut config_builder = Self::builder();

        // Also read from the configuration, if we find a config directory.
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("mixxx_library_exporter.yml");
            debug!("Looking vor config at: {config_path:?}");
            if !config_path.exists() {
                warn!("Couldn't find config at path: {config_path:?}!");
            } else {
                config_builder = config_builder.file(&config_path);
            }
        }

        // Also read from the configuration, if we find a document directory.
        if let Some(document_dir) = dirs::document_dir() {
            let document_path = document_dir.join("mixxx_library_exporter.yml");
            debug!("Looking vor config at: {document_path:?}");
            if !document_path.exists() {
                warn!("Couldn't find config at path: {document_path:?}!");
            } else {
                config_builder = config_builder.file(&document_path);
            }
        }

        // Also read from the configuration, if we find a home directory.
        if let Some(home_dir) = dirs::home_dir() {
            let home_path = home_dir.join("mixxx_library_exporter.yml");
            debug!("Looking vor config at: {home_path:?}");
            if !home_path.exists() {
                warn!("Couldn't find config at path: {home_path:?}!");
            } else {
                config_builder = config_builder.file(&home_path);
            }
        }

        let config = config_builder.load()?;

        Ok(config)
    }

    /// Run some configuration sanity checks
    pub fn validate(&self) -> Result<()> {
        // Make sure the target_directory actually exists.
        if !self.target_directory.exists() {
            bail!(
                "Target directory doesn't seem to exist: '{:?}'",
                self.target_directory.to_string_lossy()
            )
        }

        // Make sure the target_library_root actually exists.
        if !self.target_library_root.exists() {
            bail!(
                "Target library root doesn't seem to exist: '{:?}'",
                self.target_library_root.to_string_lossy()
            )
        }

        // Make sure the target_library_root is absolute.
        if !self.target_library_root.is_absolute() {
            bail!(
                "target_library_root has to be an absolute path: '{:?}'",
                self.target_library_root.to_string_lossy()
            )
        }

        Ok(())
    }
}
