//! TODO

use std::{error, path::PathBuf, ptr::addr_of, fmt::{Display, self}};

use thiserror::Error;

use crate::{
    common::thread::UnityThread,
    libs::{self, NativeLibrary},
};

use self::{exports::MonoExports, types::{MonoThread, MonoDomain}};

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
    MissingFunction(String),
    #[error("{0}")]
    Passthrough(String),
}

/// assembly hook types
#[derive(Debug, Clone, Copy)]
pub enum AssemblyHookType {
    /// called when an assembly is loaded
    Preload,
    /// called when an assembly is unloaded
    Load,
    /// called when an assembly is searched
    Search,
}

impl Display for AssemblyHookType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyHookType::Preload => write!(f, "preload"),
            AssemblyHookType::Load => write!(f, "load"),
            AssemblyHookType::Search => write!(f, "search"),
        }
    }
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
            None => Err(MonoError::MissingFunction("mono_thread_current".to_string())),
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
            None => Err(MonoError::MissingFunction("mono_thread_set_main".to_string())),
            Some(mono_thread_set_main) => {
                mono_thread_set_main(thread.inner.cast());
                Ok(())
            }
        }
    }

    pub fn add_internal_call(&self, name: impl Into<String>, func: usize) -> Result<(), MonoError> {
        match &self.exports.mono_add_internal_call {
            None => Err(MonoError::MissingFunction("mono_add_internal_call".to_string())),
            Some(mono_add_internal_call) => {
                let name = name.into();
                let name = std::ffi::CString::new(name).map_err(|e| MonoError::Passthrough(e.to_string()))?;
                mono_add_internal_call(name.as_ptr(), func as *mut std::ffi::c_void, std::ptr::null_mut());
                Ok(())
            }
        }
    }

    pub fn install_assembly_hook(&self, hook_type: AssemblyHookType, address: usize) -> Result<(), MonoError> {
        let hook_name = format!("mono_assembly_{}_hook", hook_type);

        let hook_func = match hook_type {
            AssemblyHookType::Preload => &self.exports.mono_install_assembly_preload_hook,
            AssemblyHookType::Load => &self.exports.mono_install_assembly_load_hook,
            AssemblyHookType::Search => &self.exports.mono_install_assembly_search_hook,
        };

        match hook_func {
            None => Err(MonoError::MissingFunction(hook_name)),
            Some(hook_func) => {
                hook_func(address as *mut std::ffi::c_void, std::ptr::null_mut());
                Ok(())
            }
        }
    }

    pub fn get_domain(&self) -> Result<*mut MonoDomain, MonoError> {
        match &self.exports.mono_get_root_domain {
            None => Err(MonoError::MissingFunction("mono_get_root_domain".to_string())),
            Some(mono_get_root_domain) => {
                let res = mono_get_root_domain();
                match res.is_null() {
                    true => Err(MonoError::ReturnedNull("mono_get_root_domain")),
                    false => Ok(res),
                }
            }
        }
    }
}
