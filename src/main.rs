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

	// let source = read_to_string();
	//
	// let lua = Lua::new();
	// lua.sandbox(true)?;
	//
	// lua.load(
	// 	r#"
	// 		print(`Hello, world`)
	// 	"#
	// )
	// .exec()?;

	Ok(())
}
