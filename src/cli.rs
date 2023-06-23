use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "dj-converter",
    about = "Convert your dj set between different formats.",
    author,
    version
)]
pub struct CliArguments {
    /// Verbose mode (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,

    /// Set this to create a `mixxx_library.json` file in the target directory
    #[arg(short, long, action = ArgAction::Count)]
    pub json_export: bool,
}
