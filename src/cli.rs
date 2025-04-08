use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_name = "PATH")]
    pub file_to_run: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Option<Commands>
}

impl Args {
    pub fn no_args_passed(&self) -> bool {
        self.file_to_run.is_none() && self.command.is_none()
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Bless
}