mod cli;
mod extension;
mod ludorc;
mod run;
mod bless;
mod luaurc;
mod fs_util;

use crate::cli::Args;
use crate::ludorc::load_user_rc;
use anyhow::{Context, Result};
use clap::Parser;
use std::rc::Rc;
use ansi_term::Color::Green;
use ansi_term::Style;
use log::warn;
use crate::run::run_from_fs;

fn main() -> Result<()> {
	if ansi_term::enable_ansi_support().is_err() {
		warn!("Could not enable ANSI support");
	}
	let args = Args::parse();
	if args.no_args_passed() {
		let crate_version = env!("CARGO_PKG_VERSION");
		println!("ludo {crate_version}");
		return Ok(());
	}
	let user_rc = Rc::new(load_user_rc().context("Failed to load user .ludorc")?.unwrap_or_default());
	if let Some(file_to_run) = args.file_to_run {
		run_from_fs(user_rc, file_to_run)
	}
	else {
		todo!();
	}
}
