use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;
use ansi_term::Color::{Blue, Red, Yellow};
use ansi_term::Style;
use anyhow::{Context, Result};
use crate::bless::{collect_transitive_natives, TransitiveNative};
use crate::luaurc::{load_composite_luau_rc, CanonicalLuauRc};
use crate::ludorc::{load_composite_workspace_rc, load_user_rc, UserRc, WorkspaceRc};

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
        let workspace_rc = load_composite_workspace_rc(&workspace).context("Failed to load workspace .ludorc")?;
        let luau_rc = load_composite_luau_rc(&workspace).context("Failed to load .luaurc")?;
        Ok(Self { user_rc, workspace_rc, luau_rc, script_location })
    }
}

pub fn interactive_bless_check(
    transitive_natives: &[TransitiveNative]
) -> Result<()> {
    let not_blessed: Vec<_> = transitive_natives.iter().filter(|x| !x.is_blessed()).collect();
    if not_blessed.is_empty() { return Ok(()) }

    println!("\n\n");
    println!("{}", Red.bold().blink().paint("Hold up!"));
    println!();
    println!("Ludo has detected binary libraries that you haven't run before and has stopped running.\
    Running these libraries is risky because they have full access to your system.\
    If you didn't expect to see this message, {}", Red.bold().paint("stop here!"));
    println!();
    println!("These are the binary libraries Ludo found, alongside their hashes:");
    for transitive_native in not_blessed {
        println!(
            "-> {} {} hash: {}",
            Yellow.bold().paint(&transitive_native.bless.title),
            Style::new().dimmed().paint(&transitive_native.bless.path.display().to_string()),
            &transitive_native.bless.hash
        );
    }
    println!("Ensure these hashes match the public hash for the library you're using.\
    (The public hash may be found in the library's online documentation, for example.)");
    println!();
    println!("Once verified, run {} to allow these modified libraries to run.", Blue.paint("ludo bless"));

    exit(1);
}

pub fn run_from_fs(
    user_rc: Rc<UserRc>,
    path: PathBuf
) -> Result<()> {
    let context = ScriptContext::new_from_fs(user_rc, path)?;

    let transitive_natives = collect_transitive_natives(&context)?;
    interactive_bless_check(&transitive_natives)?;


    Ok(())
}