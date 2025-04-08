mod cli;
mod extension;
mod ludorc;
mod run;
mod bless;
mod luaurc;
mod fs_util;

use anyhow::Result;
use std::io::read_to_string;
use clap::Parser;
use mlua::Lua;
use crate::cli::Args;

fn main() -> Result<()> {
	let args = Args::parse();



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
