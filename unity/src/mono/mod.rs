use std::{error, path::PathBuf};

use thiserror::Error;

use crate::libs::{self, NativeLibrary};

use self::exports::MonoExports;

pub mod exports;
pub mod types;

#[derive(Debug, Error)]
enum MonoError {
    #[error("Failed to get Mono Lib Name")]
    MonoLibName,
    #[error("Failed to get Mono Lib Path")]
    MonoLibPath,
}

#[derive(Debug, Clone)]
pub struct Mono {
    pub is_old: bool,
    pub mono_lib: NativeLibrary,
    pub exports: MonoExports,
}

impl Mono {
    pub fn new(mono_path: PathBuf) -> Result<Self, Box<dyn error::Error>> {
        if !mono_path.exists() {
            return Err(Box::new(MonoError::MonoLibPath));
        }

        let lib_name = mono_path
            .file_stem()
            .ok_or(MonoError::MonoLibName)?
            .to_str()
            .ok_or(MonoError::MonoLibName)?;

        let is_old = lib_name == "mono" || lib_name == "libmono";

        let mono_lib = libs::load_lib(&mono_path)?;

        let exports = MonoExports::new(&mono_lib)?;

        let mono = Mono {
            is_old,
            mono_lib,
            exports,
        };

        Ok(mono)
    }
}
