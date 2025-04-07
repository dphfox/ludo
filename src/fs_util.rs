use anyhow::Result;
use std::fs::File;
use std::io;
use std::path::Path;

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