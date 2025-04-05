mod extension;

use mlua::Lua;
use mlua::Result;

fn main() -> Result<()> {
	let lua = Lua::new();
	lua.sandbox(true)?;

	lua.load(
		r#"
			print(`Hello, world`)
		"#
	)
	.exec()?;

	Ok(())
}
