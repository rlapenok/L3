use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'p', long)]
    pub path: PathBuf,
}
