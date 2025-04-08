use std::fmt::{Display, Formatter};
use base64ct::{Base64, Encoding};
use sha3::digest::Update;
use sha3::{Digest, Sha3_256};
use std::path::{PathBuf};
use thiserror::Error;
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
        let mut hash = Sha3_256::new();
        Digest::update(&mut hash, title.as_bytes());
        Digest::update(&mut hash, path.as_os_str().as_encoded_bytes());
        Digest::update(&mut hash, bytes);
        let hash = Base64::encode_string(hash.finalize().as_slice());
        Self { title, path, hash }
    }
}

#[derive(Error, Debug)]
pub struct NotBlessedError {
    not_blessed: Vec<BlessInfo>
}

impl Display for NotBlessedError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{} modules were not blessed", self.not_blessed.len())
    }
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
            todo!();
        }
    }

    visit(main_context);

    if not_blessed.is_empty() {
        Ok(())
    } else {
        Err(NotBlessedError { not_blessed })
    }
}