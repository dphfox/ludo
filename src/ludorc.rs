use crate::fs_util::open_file_if_exists;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::ffi::{CString};
use std::path::{Path, PathBuf};
use crate::native::BlessInfo;

#[derive(Debug, Deserialize, Clone)]
pub struct UserRc {
    pub version: u32,
    #[serde(default)]
    pub blessed: HashSet<String>
}

impl Default for UserRc {
    fn default() -> Self {
        Self {
            version: 1,
            blessed: HashSet::new()
        }
    }
}

impl UserRc {
    pub fn is_blessed(
        &self,
        info: &BlessInfo
    ) -> bool {
        self.blessed.contains(&info.hash)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorkspaceRc {
    pub version: u32,
    #[serde(default)]
    pub permissions: HashMap<String, Permissions>,
    #[serde(default)]
    pub native: Option<Native>
}

impl Default for WorkspaceRc {
    fn default() -> Self {
        Self {
            version: 1,
            permissions: HashMap::new(),
            native: None
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Permissions {
    pub native: bool
}

#[derive(Debug, Deserialize, Clone)]
pub struct Native {
    pub name: String,
    pub parent: PathBuf,
    pub entry_point: CString
}

pub fn load_user_rc() -> Result<Option<UserRc>> {
    let Some(home_dir) = dirs::home_dir() else { return Ok(None) };
    let rc_path = home_dir.join(".ludorc");
    let Some(file) = open_file_if_exists(&rc_path)? else { return Ok(None) };
    let rc: UserRc = serde_json::from_reader(file)?;
    if rc.version != 1 {
        bail!("Unsupported ludorc version: {}", rc.version);
    }
    Ok(Some(rc))
}

pub fn load_workspace_rc(
    path: &Path
) -> Result<WorkspaceRc> {
    path.ancestors()
        .into_iter()
        .map(|ancestor| -> Result<_> {
            let Some(file) = open_file_if_exists(&ancestor.join(".ludorc"))
                .with_context(|| format!("Failed to load .ludorc at {}", ancestor.display()))?
            else { return Ok(None) };
            Ok(Some((ancestor, file)))
        })
        .filter_map(Result::transpose)
        .map(|result| {
            let (ancestor, file) = result?;
            let rc: WorkspaceRc = serde_json::from_reader(file)
                .with_context(|| format!("Failed to decode .ludorc at {}", ancestor.display()))?;
            if rc.version != 1 {
                bail!("Unsupported ludorc version: {}", rc.version);
            }
            Ok(rc)
        })
        .next()
        .unwrap_or(Ok(WorkspaceRc::default()))
}