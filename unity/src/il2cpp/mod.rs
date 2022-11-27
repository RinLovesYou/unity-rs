use std::{error, path::PathBuf};

use thiserror::Error;

use crate::{
    join_dll_path,
    libs::{self, NativeLibrary},
};

use self::exports::Il2CppExports;

pub mod exports;
pub mod types;

#[derive(Debug, Error)]
pub enum Il2CppError {
    #[error("Failed to find GameAssembly")]
    GameAssemblyNotFound,
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
}
