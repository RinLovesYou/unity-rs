//! TODO

use std::{error, path::PathBuf};

use thiserror::Error;

use crate::{
    common::thread::UnityThread,
    libs::{self, NativeLibrary},
};

use self::{exports::MonoExports, types::MonoThread};

pub mod exports;
pub mod types;

#[derive(Debug, Error)]
pub enum MonoError {
    #[error("Failed to get Mono Lib Name")]
    MonoLibName,
    #[error("Failed to get Mono Lib Path")]
    MonoLibPath,
    #[error("Function `{0}` returned Null")]
    ReturnedNull(&'static str),
    #[error("Function '{0}' not found")]
    MissingFunction(&'static str),
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

    pub fn thread_current(&self) -> Result<*mut MonoThread, MonoError> {
        match &self.exports.mono_thread_current {
            None => Err(MonoError::MissingFunction("mono_thread_current")),
            Some(mono_thread_current) => {
                let res = mono_thread_current();
                match res.is_null() {
                    true => Err(MonoError::ReturnedNull("mono_thread_current")),
                    false => Ok(res),
                }
            }
        }
    }

    pub fn thread_set_main(&self, thread: UnityThread) -> Result<(), MonoError> {
        match &self.exports.mono_thread_set_main {
            None => Err(MonoError::MissingFunction("mono_thread_set_main")),
            Some(mono_thread_set_main) => {
                mono_thread_set_main(thread.inner.cast());
                Ok(())
            }
        }
    }
}
