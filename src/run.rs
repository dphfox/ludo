use std::fs;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;
use ansi_term::Color::{Blue, Red, Yellow};
use ansi_term::Style;
use anyhow::{bail, Context, Result};
use mlua::{ChunkMode, Compiler, ErrorContext, Lua};
use mlua::prelude::LuaFunction;
use crate::native::{collect_transitive_natives, load_native_library, TransitiveNative};
use crate::luaurc::{load_composite_luau_rc, CanonicalLuauRc};
use crate::ludorc::{load_workspace_rc, load_user_rc, UserRc, WorkspaceRc};

#[derive(Debug, Clone)]
pub struct ScriptContext {
    pub user_rc: Rc<UserRc>,
    pub workspace_rc: WorkspaceRc,
    pub luau_rc: CanonicalLuauRc,
    pub script_location: PathBuf
}

impl ScriptContext {
    pub fn new_from_fs(
        user_rc: Rc<UserRc>,
        script_location: PathBuf
    ) -> Result<Self> {
        let workspace = script_location.parent().context("Ludo scripts must exist inside of a workspace")?;
        let workspace_rc = load_workspace_rc(&workspace).context("Failed to construct workspace .ludorc")?;
        let luau_rc = load_composite_luau_rc(&workspace).context("Failed to construct .luaurc")?;
        Ok(Self { user_rc, workspace_rc, luau_rc, script_location })
    }

    pub fn lua_chunk_name(
        &self
    ) -> String {
        let friendly_name = self.script_location.file_stem()
            .or(self.script_location.file_name())
            .unwrap_or(self.script_location.as_os_str());
        format!("@TODO-PATH/{}", friendly_name.to_string_lossy())
    }
}

pub fn terminate_if_not_blessed(
    context: &ScriptContext,
) -> Result<()> {
    let transitive_natives = collect_transitive_natives(&context)?;
    let not_blessed: Vec<_> = transitive_natives.iter().filter(|x| !x.is_blessed()).collect();
    if not_blessed.is_empty() { return Ok(()) }

    println!("\n");
    println!("{}", Red.bold().blink().paint("==================== Hold up! ===================="));
    println!();
    println!("This Ludo script is trying to run native libraries that you haven't run before.");
    println!("Running these libraries is risky because they have full access to your system.");
    println!("If you didn't expect to see this message, {}", Red.bold().paint("stop here!"));
    println!();
    println!("These are the native libraries Ludo found, alongside their hashes:");
    for transitive_native in not_blessed {
        println!();
        println!("-> {}", Yellow.bold().paint(&transitive_native.bless.title));
        println!("   hash: {}", &transitive_native.bless.hash);

        println!("   {}", Style::new().dimmed().paint(&transitive_native.bless.path.display().to_string()));
    }
    println!();
    println!("Ensure these hashes match the public hash for the library you're using.");
    println!("(The public hash may be found in the library's online documentation, for example.)");
    println!();
    println!("Once verified, run {} to allow running these libraries next time.", Blue.paint("ludo bless"));
    println!();
    println!("{}", Red.bold().blink().paint("=================================================="));
    println!("\n");

    exit(1);
}

pub fn run_script(
    context: &ScriptContext,
) -> Result<()> {
    terminate_if_not_blessed(&context)?;
    let source = fs::read_to_string(&context.script_location)
        .with_context(|| format!("Could not read source file at {}", context.script_location.display()))?;
    let lua = Lua::new();
    if let Some(native) = &context.workspace_rc.native {
        unsafe { load_native_library(&lua, native) }.context("Failed to load native library")?;
    }
    let Ok(_) = lua.sandbox(true) else { bail!("Failed to initialise Luau sandbox") };
    let func = lua.load(source)
        .set_name(context.lua_chunk_name())
        .set_mode(ChunkMode::Text);
    match func.exec() {
        Ok(_) => Ok(()),
        Err(e) => bail!(e.to_string())
    }
}

pub fn run_from_fs(
    user_rc: Rc<UserRc>,
    script_location: PathBuf
) -> Result<()> {
    let context = ScriptContext::new_from_fs(user_rc, script_location.clone()).context("Failed to construct script context")?;
    run_script(&context)
}