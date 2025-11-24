use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, long_about = Some("Utility for autoformatting RON files."))]
pub struct Arguments {
    /// Sets which file to format
    #[arg(short, long)]
    pub input: PathBuf,

    /// Sets soft max line width for formatting heuristics
    #[arg(short, default_value_t = 40)]
    pub width: usize,

    /// Sets indentation size in spaces
    #[arg(short, default_value_t = 4)]
    pub tab_size: usize,

    /// Prints output to console instead of overwriting the input file
    #[arg(short, default_value_t = false)]
    pub debug: bool,
}
