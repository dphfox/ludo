use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct WorkspaceRc {
    pub version: u32,
    #[serde(default)]
    pub permissions: HashMap<String, Permissions>,
    #[serde(default)]
    pub native: Vec<Native>
}

impl Default for WorkspaceRc {
    fn default() -> Self {
        Self {
            version: 1,
            permissions: HashMap::new(),
            native: vec![]
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Permissions {
    native: bool
}

#[derive(Debug, Deserialize)]
pub struct Native {
    path: PathBuf,
    entry_point: CString
}

impl WorkspaceRc {
    pub fn compose_atop(
        mut self,
        mut ancestor: Self
    ) -> Result<Self> {
        for (key, value) in self.permissions.drain() {
            ancestor.permissions.insert(key, value);
        }
        if !ancestor.native.is_empty() {
            bail!(".ludorc files with native paths cannot have descendant .ludorc files")
        }
        Ok(ancestor)
    }
}

pub fn load_user_rc() -> Result<Option<UserRc>> {
    let Some(home_dir) = dirs::home_dir() else { Ok(None) };
    let rc_path = home_dir?.join(".ludorc");
    let Some(file) = open_file_if_exists(&rc_path)? else { Ok(None) };
    let rc: UserRc = serde_json::from_reader(file)?;
    if rc.version != 1 {
        bail!("Unsupported ludorc version: {}", rc.version);
    }
    Ok(Some(rc))
}

pub fn load_composite_workspace_rc(
    path: &Path
) -> Result<WorkspaceRc> {
    path.ancestors()
        .into_iter()
        .filter_map(|ancestor| open_file_if_exists(&ancestor.join(".ludorc")).transpose())
        .map(|file| {
            let rc: WorkspaceRc = serde_json::from_reader(file?)?;
            if rc.version != 1 {
                bail!("Unsupported ludorc version: {}", rc.version);
            }
            Ok(rc)
        })
        .try_fold(
            WorkspaceRc::default(),
            |accum, item| item?.compose_atop(accum)
        )
}