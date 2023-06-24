use std::{
    fs::{remove_file, File},
    io::Write,
};

use anyhow::{Context, Result};
use clap::Parser;

use cli::CliArguments;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;

use crate::{config::Config, mixxx::aggregator::read_library};

/// Commandline argument parsing
mod cli;
/// Configuration file.
mod config;
/// Low-level DB related logic
mod db;
/// All mixxx facing logic.
mod mixxx;
/// Rekordbox related logic.
mod rekordbox;

#[tokio::main]
async fn main() -> Result<()> {
    // Read any .env files
    dotenv::dotenv().ok();
    // Parse commandline options.
    let opt = CliArguments::parse();

    // Initalize everything
    init_app(opt.verbose)?;

    let config = Config::read().context("Failed to read config file")?;

    // Read the mixxx library and convert it into our own clean format.
    let mut con = db::new_connection(&config.mixxx_db).await?;
    let library = read_library(&mut con).await?;

    // Json export logic.
    if opt.json_export {
        // Get the target path for the json file.
        let json_target_file = config.target_directory().join("mixxx_library.json");

        // Remove any existing json file
        if json_target_file.exists() {
            info!("Removing existing json library file at: {json_target_file:?}");
            remove_file(&json_target_file)?;
        }

        // Export the library.
        let library_json = serde_json::to_string(&library)?;
        let mut file = File::create(json_target_file)?;
        file.write_all(library_json.as_bytes())?;
    }

    Ok(())
}

/// Init better_panics
/// Initialize logging
fn init_app(verbosity: u8) -> Result<()> {
    // Beautify panics for better debug output.
    better_panic::install();

    // Set the verbosity level and initialize the logger.
    let level = match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };
    Builder::new().filter_level(level).init();

    Ok(())
}
