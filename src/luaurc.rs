use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Component, Path, PathBuf};
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use crate::fs_util::open_file_if_exists;

#[derive(Debug, Deserialize, Default)]
pub struct LuauRc {
    #[serde(default)]
    pub aliases: HashMap<String, PathBuf>
}

impl LuauRc {
    pub fn canonicalise(
        mut self
    ) -> Result<CanonicalLuauRc> {
        Ok(Self {
            aliases: self.aliases.drain().map(Path::canonicalize).collect()?
        })
    }
}

#[derive(Debug, Default)]
pub struct CanonicalLuauRc {
    pub aliases: HashMap<OsString, PathBuf>
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

    pub fn resolve_module_path(
        &self,
        script_location: &Path,
        module_path: &Path
    ) -> Result<PathBuf> {
        let Some(first) = module_path.components().next() else { bail!("Module path cannot be empty") };

        let starting_directory = match first {
            Component::Prefix(_) => bail!("Module path cannot be absolute"),
            Component::RootDir => bail!("Module path cannot be absolute"),
            Component::CurDir => script_location,
            Component::ParentDir => script_location.parent().context("Module path cannot visit parent")?,
            Component::Normal(name) => {
                let name = name.to_str().context("Module path must be valid UTF-8")?;
                if !name.starts_with("@") {
                    bail!("Module paths must start with a valid prefix");
                }
                self.aliases.get(&name).context("Alias in module path has not been defined")?
            }
        };

        todo!()
    }
}

pub fn load_composite_luau_rc(
    path: &Path
) -> Result<CanonicalLuauRc> {
    path.ancestors()
        .into_iter()
        .filter_map(|ancestor| open_file_if_exists(&ancestor.join(".luaurc")).transpose())
        .map(|file| {
            let rc: LuauRc = serde_json::from_reader(file?)?;
            rc.canonicalise()
        })
        .try_fold(
            CanonicalLuauRc::default(),
            |accum, item| item.map(|item| item.compose_atop(accum))
        )
}