use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "mixxx_library_converter",
    about = "Convert your dj library between different formats.",
    author,
    version
)]
pub struct CliArguments {
    /// Verbose mode (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,

    /// Set this to create a `mixxx_library.json` file in the target directory
    #[arg(short, long)]
    pub json_export: bool,
}
