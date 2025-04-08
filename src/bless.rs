use crate::fs_util::{resolve_module_path, select_native_binary};
use crate::ludorc::Native;
use crate::run::ScriptContext;
use anyhow::{Context, Result};
use base64ct::{Base64, Encoding};
use sha3::{Digest, Sha3_256};
use std::collections::VecDeque;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

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

    pub fn new_from_fs(
        native: &Native
    ) -> Result<Self>{
        let path = select_native_binary(&native.name, &native.parent);
        let bytes = fs::read(&path).with_context(|| format!("Could not read native binary {}", native.name.to_string_lossy()))?;
        Ok(Self::new(native.name.to_string_lossy().to_string(), path, &bytes))
    }
}

pub struct TransitiveNative {
    pub context: ScriptContext,
    pub native: Native,
    pub bless: BlessInfo
}

impl TransitiveNative {
    pub fn is_blessed(&self) -> bool {
        self.context.user_rc.is_blessed(&self.bless)
    }
}

pub fn collect_transitive_natives(
    main_context: &ScriptContext
) -> Result<Vec<TransitiveNative>> {
    let mut transitive_natives = vec![];
    let mut queue = VecDeque::from([main_context.clone()]);
    while let Some(context) = queue.pop_front() {
        if let Some(native) = &context.workspace_rc.native {
            transitive_natives.push(TransitiveNative {
                context: context.clone(),
                native: native.clone(),
                bless: BlessInfo::new_from_fs(native)?
            });
        }
        for (alias, permissions) in context.workspace_rc.permissions.iter() {
            if !permissions.native { continue }
            let module_location = resolve_module_path(&context.luau_rc, &context.script_location, Path::new(alias))?;
            let sub_context = ScriptContext::new_from_fs(main_context.user_rc.clone(), module_location)?;
            queue.push_back(sub_context);
        }
    }
    Ok(transitive_natives)
}