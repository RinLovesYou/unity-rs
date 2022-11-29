//! TODO

use std::{error, path::PathBuf};

use thiserror::Error;

use crate::{
    join_dll_path,
    libs::{self, NativeLibrary},
};

use self::{exports::Il2CppExports, types::Il2CppThread};

pub mod exports;
pub mod types;

/// All errors associatedd with Il2cpp
#[derive(Debug, Error)]
pub enum Il2CppError {
    /// Failed to find the GameAssembly
    #[error("Failed to find GameAssembly")]
    GameAssemblyNotFound,
    /// A function returned a null pointer, the billion dollar mistake creeps back, thank you FFI!
    #[error("Function `{0}` returned Null")]
    ReturnedNull(&'static str),
    /// An exported function could not be found (please report!)
    #[error("Function '{0}' not found")]
    MissingFunction(&'static str),
}

#[derive(Debug, Clone)]
pub struct Il2Cpp {
    pub game_assembly: NativeLibrary,
    pub exports: Il2CppExports,
}

impl Il2Cpp {
    pub fn new(base_path: PathBuf) -> Result<Self, Box<dyn error::Error>> {
        let game_assembly_path = join_dll_path!(base_path, "GameAssembly");

        if !game_assembly_path.exists() {
            return Err(Box::new(Il2CppError::GameAssemblyNotFound));
        }

        let lib = libs::load_lib(&game_assembly_path)?;

        let exports = Il2CppExports::new(&lib)?;

        let il2cpp = Il2Cpp {
            game_assembly: lib,
            exports,
        };
        Ok(il2cpp)
    }

    pub fn thread_current(&self) -> Result<*mut Il2CppThread, Il2CppError> {
        match &self.exports.il2cpp_thread_current {
            None => Err(Il2CppError::MissingFunction("il2cpp_thread_current")),
            Some(il2cpp_thread_current) => {
                let res = il2cpp_thread_current();
                match res.is_null() {
                    true => Err(Il2CppError::ReturnedNull("il2cpp_thread_current")),
                    false => Ok(res),
                }
            }
        }
    }
}
