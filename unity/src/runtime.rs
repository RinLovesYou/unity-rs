use std::{error, ffi::CString, path::PathBuf};

use thiserror::Error;

use crate::{common::domain::UnityDomain, il2cpp::Il2Cpp, mono::Mono, utils};

#[derive(Debug, Error)]
enum RuntimeError {
    #[error("Not a unity process")]
    NotUnity,
    #[error("Failed to find Base Path")]
    BasePathNotFound,
    #[error("Missing version argument in mono_jit_init_version")]
    JitInitVersionArgMissing,
    #[error("Missing Function")]
    MissingFunc,
    #[error("Function Returned Null")]
    ReturnedNull,
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
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            runtime: detect_runtime()?,
        })
    }

    pub fn init(
        &self,
        name: impl Into<String>,
        version: Option<impl Into<String>>,
    ) -> Result<UnityDomain, Box<dyn error::Error>> {
        let name = name.into();
        let name = CString::new(name)?;
        match self.runtime.clone() {
            UnityRuntime::MonoRuntime(mono) => {
                if version.is_none() {
                    return Err(Box::new(RuntimeError::JitInitVersionArgMissing));
                }

                let version =
                    version.ok_or_else(|| Box::new(RuntimeError::JitInitVersionArgMissing))?;

                let version = version.into();
                let version = CString::new(version)?;

                let func = mono
                    .exports
                    .mono_jit_init_version
                    .ok_or_else(|| Box::new(RuntimeError::MissingFunc))?;

                let res = func(name.as_ptr(), version.as_ptr());

                if res.is_null() {
                    Err(Box::new(RuntimeError::ReturnedNull))
                } else {
                    Ok(UnityDomain { inner: res.cast() })
                }
            }

            UnityRuntime::Il2Cpp(il2cpp) => {
                let func = il2cpp
                    .exports
                    .il2cpp_init
                    .ok_or_else(|| Box::new(RuntimeError::MissingFunc))?;

                let res = func(name.as_ptr());

                if res.is_null() {
                    Err(Box::new(RuntimeError::ReturnedNull))
                } else {
                    Ok(UnityDomain { inner: res.cast() })
                }
            }
        }
    }
}

/// looks up the runtime
fn detect_runtime() -> Result<UnityRuntime, Box<dyn error::Error>> {
    let exe_path = std::env::current_exe()?;
    if !is_unity(&exe_path)? {
        return Err(Box::new(RuntimeError::NotUnity));
    }

    let base_path = exe_path
        .parent()
        .ok_or(RuntimeError::BasePathNotFound)?
        .to_path_buf();
    let data_path = utils::path::get_data_path(&exe_path)?;

    let mono = utils::path::find_mono(&base_path, &data_path);

    if let Ok(mono_path) = mono {
        Ok(UnityRuntime::MonoRuntime(Mono::new(mono_path)?))
    } else {
        Ok(UnityRuntime::Il2Cpp(Il2Cpp::new(base_path)?))
    }
}

fn is_unity(file_path: &PathBuf) -> Result<bool, Box<dyn error::Error>> {
    let file_name = file_path
        .file_stem()
        .ok_or_else(|| RuntimeError::BasePathNotFound)?
        .to_str()
        .ok_or_else(|| RuntimeError::BasePathNotFound)?
        .to_lowercase();

    let a: char = 'a';
    println!("{a}\n");

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
