//! TODO

use std::{error, path::PathBuf, io};

use thiserror::Error;

use crate::{
    common::{domain::UnityDomain, thread::UnityThread, method::{MethodPointer}},
    il2cpp::Il2Cpp,
    mono::{Mono, AssemblyHookType},
    utils, libs::{self, NativeMethod},
};

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    Std(#[from] Box<dyn error::Error>),
    #[error(transparent)]
    Lib(#[from] libs::LibError),
    #[error(transparent)]
    Nul(#[from] std::ffi::NulError),

    #[error("Not a unity process")]
    NotUnity,
    #[error("Failed to find Base Path")]
    BasePathNotFound,
    #[error("Data Path not found!")]
    DataPathNotFound,
    #[error("Failed to find mono library path")]
    MonoLibPath,
    #[error("Failed to get mono lib name")]
    MonoLibName,
    #[error("Missing version argument in mono_jit_init_version")]
    JitInitVersionArgMissing,
    #[error("Function '{0}' not found")]
    MissingFunction(&'static str),
    #[error("Function Returned Null at {0}")]
    ReturnedNull(&'static str),
    #[error("Failed to get Game Assembly")]
    GameAssemblyNotFound,
    #[error("Failed to initialize Runtime")]
    FailedToInitRuntime,
    #[error("Failed to create C-String")]
    FailedToCreateCString,
    #[error("{0}")]
    Passthrough(String),
    #[error("String may not be empty!")]
    EmptyString,
    #[error("Argument {0} is a null pointer!")]
    NullPointer(&'static str),
    #[error("Not Implemented: {0}")]
    NotImplemented(&'static str),
}

pub enum RuntimeType<'a> {
    Mono(&'a Mono),
    Il2Cpp(&'a Il2Cpp)
}

pub trait Runtime {
    fn get_type(&self) -> RuntimeType;
    fn get_domain(&self) -> Result<UnityDomain, RuntimeError>;
    fn get_current_thread(&self) -> Result<UnityThread, RuntimeError>;
    fn set_main_thread(&self, thread: UnityThread) -> Result<(), RuntimeError>;
    fn attach_to_thread(&self, thread: UnityDomain) -> Result<UnityThread, RuntimeError>;
    fn add_internal_call(&self, name: String, func: MethodPointer) -> Result<(), RuntimeError>;
    fn install_assembly_hook(&self, hook_type: AssemblyHookType, func: MethodPointer) -> Result<(), RuntimeError>;
    fn get_export_ptr(&self, name: &str) -> Result<MethodPointer, RuntimeError>;
}


/// looks up the runtime
pub fn get_runtime() -> Result<Box<dyn Runtime>, RuntimeError> {
    let exe_path = std::env::current_exe()?;
    if !is_unity(&exe_path)? {
        return Err(RuntimeError::NotUnity);
    }

    let base_path = exe_path
        .parent()
        .ok_or(RuntimeError::BasePathNotFound)?
        .to_path_buf();
    let data_path =
        utils::path::get_data_path(&exe_path)?;

    let mono = utils::path::find_mono(&base_path, &data_path);

    if let Ok(mono_path) = mono {
        let mono = Mono::new(mono_path)?;
        Ok(Box::new(mono))
    } else {
        let il2cpp = Il2Cpp::new(base_path)?;
        Ok(Box::new(il2cpp))
    }
}

fn is_unity(file_path: &PathBuf) -> Result<bool, RuntimeError> {
    let file_name = file_path
        .file_stem()
        .ok_or(RuntimeError::BasePathNotFound)?
        .to_str()
        .ok_or(RuntimeError::BasePathNotFound)?;

    let base_folder = file_path
        .parent()
        .ok_or(RuntimeError::BasePathNotFound)?;

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
