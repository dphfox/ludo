use base64ct::{Base64, Encoding};
use sha3::digest::Update;
use sha3::{Digest, Sha3_256};
use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::ludorc::WorkspaceRc;
use crate::run::RunContext;

#[derive(Debug, Clone)]
pub struct BlessInfo {
    pub title: String,
    pub path: PathBuf,
    pub hash: String
}

impl BlessInfo {
    pub fn new(
        title: String,
        path: PathBuf,
        bytes: &[u8]
    ) -> Self {
        let hash = Base64::encode_string(
            Sha3_256::new()
            .update(title.as_bytes())
            .update(path.as_os_str().as_ref())
            .update(bytes)
            .finalize()
        );
        Self { title, path, hash }
    }
}

#[derive(Error, Debug)]
pub struct NotBlessedError {
    not_blessed: Vec<BlessInfo>
}

pub fn ensure_blessed(
    main_context: RunContext
) -> Result<(), NotBlessedError> {
    let mut not_blessed = vec![];

    fn visit(
        context: RunContext
    ) {
        for relative_path in context.native_paths() {

            let sub_workspace = context.workspace.join(&relative_path);



            let sub_context = RunContext::new_from_fs()
            visit()
        }
    }

    visit(main_context);

    if not_blessed.is_empty() {
        Ok(())
    } else {
        Err(NotBlessedError { not_blessed })
    }
}