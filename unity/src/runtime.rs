//! TODO

use std::{error, ffi::CString, path::PathBuf};

use thiserror::Error;

use crate::{
    common::{domain::UnityDomain, thread::UnityThread},
    il2cpp::Il2Cpp,
    mono::Mono,
    utils,
};

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Not a unity process")]
    NotUnity,
    #[error("Failed to find Base Path")]
    BasePathNotFound,
    #[error("Data Path not found!")]
    DataPathNotFound,
    #[error("Missing version argument in mono_jit_init_version")]
    JitInitVersionArgMissing,
    #[error("Missing Function")]
    MissingFunc,
    #[error("Function Returned Null")]
    ReturnedNull,
    #[error("Failed to initialize Runtime")]
    FailedToInitRuntime,
    #[error("Failed to create C-String")]
    FailedToCreateCString,
    #[error("{0}")]
    Passthrough(String),
}

#[derive(Debug, Clone)]
pub enum UnityRuntime {
    MonoRuntime(Mono),
    Il2Cpp(Il2Cpp),
}

#[derive(Debug, Clone)]
pub struct Runtime {
    pub runtime: UnityRuntime,
}

impl Runtime {
    pub fn new() -> Result<Self, RuntimeError> {
        Ok(Self {
            runtime: detect_runtime()?,
        })
    }

    /// the equivalent of `mono_jit_init_version` or `il2cpp_init`
    ///
    /// `mono_jit_init_version` requires a name, and a version
    ///
    /// `il2cpp_init` just requires a name, it'll ignore the second parameter
    pub fn init(
        &self,
        name: impl Into<String>,
        version: Option<impl Into<String>>,
    ) -> Result<UnityDomain, RuntimeError> {
        let name = name.into();
        let name = CString::new(name).map_err(|_| RuntimeError::FailedToCreateCString)?;
        match self.runtime.clone() {
            UnityRuntime::MonoRuntime(mono) => {
                if version.is_none() {
                    return Err(RuntimeError::JitInitVersionArgMissing);
                }

                let version = version.ok_or_else(|| RuntimeError::JitInitVersionArgMissing)?;

                let version = version.into();
                let version =
                    CString::new(version).map_err(|_| RuntimeError::FailedToCreateCString)?;

                let func = mono
                    .exports
                    .mono_jit_init_version
                    .ok_or_else(|| RuntimeError::MissingFunc)?;

                let res = func(name.as_ptr(), version.as_ptr());

                if res.is_null() {
                    Err(RuntimeError::ReturnedNull)
                } else {
                    Ok(UnityDomain { inner: res.cast() })
                }
            }

            UnityRuntime::Il2Cpp(il2cpp) => {
                let func = il2cpp
                    .exports
                    .il2cpp_init
                    .ok_or_else(|| RuntimeError::MissingFunc)?;

                let res = func(name.as_ptr());

                if res.is_null() {
                    Err(RuntimeError::ReturnedNull)
                } else {
                    Ok(UnityDomain { inner: res.cast() })
                }
            }
        }
    }

    pub fn get_current_thread(&self) -> Result<UnityThread, RuntimeError> {
        match self.clone().runtime {
            UnityRuntime::MonoRuntime(mono) => {
                let res = mono
                    .thread_current()
                    .map_err(|e| RuntimeError::Passthrough(e.to_string()))?;

                Ok(UnityThread { inner: res.cast() })
            }

            UnityRuntime::Il2Cpp(il2cpp) => {
                let res = il2cpp
                    .thread_current()
                    .map_err(|e| RuntimeError::Passthrough(e.to_string()))?;

                Ok(UnityThread { inner: res.cast() })
            }
        }
    }
}

/// looks up the runtime
fn detect_runtime() -> Result<UnityRuntime, RuntimeError> {
    let exe_path = std::env::current_exe().map_err(|_| RuntimeError::BasePathNotFound)?;
    if !is_unity(&exe_path)? {
        return Err(RuntimeError::NotUnity);
    }

    let base_path = exe_path
        .parent()
        .ok_or(RuntimeError::BasePathNotFound)?
        .to_path_buf();
    let data_path =
        utils::path::get_data_path(&exe_path).map_err(|_| RuntimeError::DataPathNotFound)?;

    let mono = utils::path::find_mono(&base_path, &data_path);

    if let Ok(mono_path) = mono {
        Ok(UnityRuntime::MonoRuntime(
            Mono::new(mono_path).map_err(|_| RuntimeError::FailedToInitRuntime)?,
        ))
    } else {
        Ok(UnityRuntime::Il2Cpp(
            Il2Cpp::new(base_path).map_err(|_| RuntimeError::FailedToInitRuntime)?,
        ))
    }
}

fn is_unity(file_path: &PathBuf) -> Result<bool, RuntimeError> {
    let file_name = file_path
        .file_stem()
        .ok_or_else(|| RuntimeError::BasePathNotFound)?
        .to_str()
        .ok_or_else(|| RuntimeError::BasePathNotFound)?;

    let base_folder = file_path
        .parent()
        .ok_or_else(|| RuntimeError::BasePathNotFound)?;

    let data_path = base_folder.join(format!("{}_Data", file_name));

    if !data_path.exists() {
        return Ok(false);
    }

    let global_game_managers = data_path.join("globalgamemanagers");
    let data_unity3d = data_path.join("data.unity3d");
    let main_data = data_path.join("mainData");

    if global_game_managers.exists() || data_unity3d.exists() || main_data.exists() {
        Ok(true)
    } else {
        Ok(false)
    }
}
