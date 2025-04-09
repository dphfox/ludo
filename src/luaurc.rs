use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Component, Path, PathBuf};
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use crate::fs_util::open_file_if_exists;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct LuauRc {
    #[serde(default)]
    pub aliases: HashMap<String, PathBuf>
}

impl LuauRc {
    pub fn canonicalise(
        mut self,
        workspace_root: &Path,
    ) -> Result<CanonicalLuauRc> {
        Ok(CanonicalLuauRc {
            aliases: self.aliases.drain()
                .map(|(alias, path)| Ok((alias, workspace_root.join(path).canonicalize()?)))
                .collect::<Result<_>>()?
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct CanonicalLuauRc {
    pub aliases: HashMap<String, PathBuf>
}

impl CanonicalLuauRc {
    pub fn compose_atop(
        mut self,
        mut ancestor: Self
    ) -> Self {
        for (key, value) in self.aliases.drain() {
            ancestor.aliases.insert(key, value);
        }
        ancestor
    }
}

pub fn load_composite_luau_rc(
    path: &Path
) -> Result<CanonicalLuauRc> {
    path.ancestors()
        .into_iter()
        .map(|ancestor| -> Result<_> {
            let Some(file) = open_file_if_exists(&ancestor.join(".luaurc"))
                .with_context(|| format!("Failed to load .luaurc at {}", ancestor.display()))?
                else { return Ok(None) };
            Ok(Some((ancestor, file)))
        })
        .filter_map(Result::transpose)
        .map(|result| {
            let (ancestor, file) = result?;
            let rc: LuauRc = serde_json::from_reader(file)
                .with_context(|| format!("Failed to decode .luaurc at {}", ancestor.display()))?;
            let rc = rc.canonicalise(ancestor)
                .with_context(|| format!("Failed to canonicalise .luaurc at {}", ancestor.display()))?;
            Ok(rc)
        })
        .try_fold(
            CanonicalLuauRc::default(),
            |accum, item| item.map(|item| item.compose_atop(accum))
        )
}