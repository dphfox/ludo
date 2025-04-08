use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use crate::bless::ensure_blessed;
use crate::ludorc::{load_composite_workspace_rc, load_user_rc, UserRc, WorkspaceRc};

#[derive(Debug)]
pub struct RunContext {
    pub user_rc: UserRc,
    pub workspace_rc: WorkspaceRc,
    pub workspace: PathBuf
}

impl RunContext {
    pub fn new_from_fs(
        path: &Path
    ) -> Result<Self> {
        let user_rc = load_user_rc().context("Failed to load user .ludorc")?.unwrap_or_default();
        let workspace = path.parent().context("Ludo scripts must exist inside of a workspace")?.to_path_buf();
        let workspace_rc = load_composite_workspace_rc(&workspace).context("Failed to load workspace .ludorc")?;
        Ok(Self { user_rc, workspace_rc, workspace })
    }

    pub fn native_paths(&self) -> Vec<PathBuf> {
        todo!()
    }
}

pub fn run_from_fs(
    path: &Path
) -> Result<()> {
    let context = RunContext::new_from_fs(path)?;

    ensure_blessed(context)?;


    Ok(())
}