use std::ffi::OsStr;
use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io;
use std::path::{Component, Path, PathBuf};
use libloading::library_filename;
use crate::luaurc::CanonicalLuauRc;

pub fn open_file_if_exists(
    path: &Path
) -> Result<Option<File>> {
    match File::open(path) {
        Ok(f) => Ok(Some(f)),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => Ok(None),
            _ => Err(err.into())
        }
    }
}

pub fn resolve_module_path(
    luau_rc: &CanonicalLuauRc,
    script_location: &Path,
    module_path: &Path
) -> Result<PathBuf> {
    let mut parts = module_path.components();
    let starting_directory = match parts.next() {
        None => bail!("Module path cannot be empty"),
        Some(Component::Prefix(_)) => bail!("Module path cannot be absolute"),
        Some(Component::RootDir) => bail!("Module path cannot be absolute"),
        Some(Component::CurDir) => script_location.parent().context("Module path cannot visit parent")?,
        Some(Component::ParentDir) => script_location.parent().and_then(Path::parent).context("Module path cannot visit parent")?,
        Some(Component::Normal(name)) => {
            let name = name.to_str().context("Module path must be val5id UTF-8")?;
            if !name.starts_with("@") {
                bail!("Module paths must start with a valid prefix");
            }
            let name = &name[1..];
            luau_rc.aliases.get(name).with_context(|| format!("Alias {name} in module path has not been defined"))?
        }
    };
    let rest_of_path = parts.collect::<PathBuf>();
    let module_path = starting_directory.join(rest_of_path);
    Ok(module_path)
}

pub fn select_native_binary(
    name: &OsStr,
    parent: &Path
) -> PathBuf {
    parent.join(library_filename(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_empty_path() {
        let script_location = Path::new("root/ancestor/script");
        let result = resolve_module_path(
            &CanonicalLuauRc {
                aliases: [].into_iter().collect(),
            },
            script_location,
            Path::new("")
        );
        assert!(result.is_err(), "Empty paths should throw an error");
    }

    #[test]
    fn resolve_absolute_path_slash() {
        let script_location = Path::new("root/ancestor/script");
        let result = resolve_module_path(
            &CanonicalLuauRc {
                aliases: [].into_iter().collect(),
            },
            script_location,
            Path::new("/foo/bar/baz")
        );
        assert!(result.is_err(), "Paths starting with / are absolute and should throw an error");
    }

    #[test]
    fn resolve_absolute_path_prefix() {
        let script_location = Path::new("root/ancestor/script");
        let result = resolve_module_path(
            &CanonicalLuauRc {
                aliases: [].into_iter().collect(),
            },
            script_location,
            Path::new("C:/foo/bar/baz")
        );
        assert!(result.is_err(), "Paths starting with a prefix are absolute and should throw an error");
    }

    #[test]
    fn resolve_relative_path() {
        let script_location = Path::new("root/ancestor/script");
        let result = resolve_module_path(
            &CanonicalLuauRc {
                aliases: [].into_iter().collect(),
            },
            script_location,
            Path::new("./foo")
        ).expect("Relative paths should not error");
        assert_eq!(result, Path::new("root/ancestor/foo"));
    }

    #[test]
    fn resolve_parent_path() {
        let script_location = Path::new("root/ancestor/script");
        let result = resolve_module_path(
            &CanonicalLuauRc {
                aliases: [].into_iter().collect(),
            },
            script_location,
            Path::new("../foo")
        ).expect("Parent paths should not error");
        assert_eq!(result, Path::new("root/foo"));
    }

    #[test]
    fn resolve_alias() {
        let script_location = Path::new("root/ancestor/script");
        let result = resolve_module_path(
            &CanonicalLuauRc {
                aliases: [(String::from("hello"), PathBuf::from("different/path"))].into_iter().collect(),
            },
            script_location,
            Path::new("@hello")
        ).expect("Alias should not error");
        assert_eq!(result, Path::new("different/path"));
    }

    #[test]
    fn resolve_alias_nested() {
        let script_location = Path::new("root/ancestor/script");
        let result = resolve_module_path(
            &CanonicalLuauRc {
                aliases: [(String::from("hello"), PathBuf::from("different/path"))].into_iter().collect(),
            },
            script_location,
            Path::new("@hello/@world")
        ).expect("Alias should not error");
        assert_eq!(result, Path::new("different/path/@world"));
    }

    #[test]
    fn resolve_alias_path() {
        let script_location = Path::new("root/ancestor/script");
        let result = resolve_module_path(
            &CanonicalLuauRc {
                aliases: [(String::from("hello"), PathBuf::from("different/path"))].into_iter().collect(),
            },
            script_location,
            Path::new("@hello/foo/bar")
        ).expect("Alias should not error");
        assert_eq!(result, Path::new("different/path/foo/bar"));
    }
}