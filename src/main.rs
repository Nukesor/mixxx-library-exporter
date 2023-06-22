use std::{fs::File, io::Write};

use anyhow::Result;
use clap::Parser;

use cli::CliArguments;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;

use crate::mixxx::aggregator::read_library;

/// Commandline argument parsing
mod cli;
/// Low-level DB related logic
mod db;
/// All mixxx facing logic.
mod mixxx;

#[tokio::main]
async fn main() -> Result<()> {
    // Read any .env files
    dotenv::dotenv().ok();
    // Parse commandline options.
    let opt = CliArguments::parse();

    // Initalize everything
    init_app(opt.verbose)?;

    let mut con = db::new_connection().await?;
    let library = read_library(&mut con).await?;
    dbg!(&library);

    let library_json = serde_json::to_string(&library)?;
    let mut file = File::create("/tmp/library.json")?;
    file.write_all(library_json.as_bytes())?;

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
