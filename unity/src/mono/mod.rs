//! TODO

use std::{error, path::PathBuf, ptr::addr_of, fmt::{Display, self}, ffi::CString};

use thiserror::Error;

use crate::{
    common::{thread::UnityThread, domain::UnityDomain, method::{MethodPointer}},
    libs::{self, NativeLibrary, NativeMethod}, runtime::{Runtime, RuntimeError, RuntimeType},
};

use self::{exports::MonoExports, types::{MonoThread, MonoDomain}};

pub mod exports;
pub mod types;

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
            return Err(Box::new(RuntimeError::MonoLibPath));
        }

        let lib_name = mono_path
            .file_stem()
            .ok_or(RuntimeError::MonoLibName)?
            .to_str()
            .ok_or(RuntimeError::MonoLibName)?;

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

impl Runtime for Mono {
    fn get_type(&self) -> RuntimeType {
        RuntimeType::Mono(self)
    }

    fn get_export_ptr(&self, name: &str) -> Result<MethodPointer, RuntimeError> {
        let function: NativeMethod<fn()> = self.mono_lib.sym(name)?;

        if function.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("get_export_ptr"));
        }

        Ok(function.inner)
    }

    fn get_current_thread(&self) -> Result<UnityThread, RuntimeError> {
        let function = &self.exports.clone().mono_thread_current.ok_or(RuntimeError::MissingFunction("mono_thread_current"))?;
        let thread = function();

        if thread.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_thread_current").into());
        }

        Ok(UnityThread {
            inner: thread.cast(),
        })
    }

    fn set_main_thread(&self, thread: UnityThread) -> Result<(), RuntimeError> {
        let function = &self.exports.clone().mono_thread_set_main.ok_or(RuntimeError::MissingFunction("mono_thread_set_main"))?;

        if thread.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_thread_set_main"));
        }

        function(thread.inner.cast());
        Ok(())
    }

    fn attach_to_thread(&self, thread: UnityDomain) -> Result<UnityThread, RuntimeError> {
        let function = &self.exports.clone().mono_thread_attach.ok_or(RuntimeError::MissingFunction("mono_thread_attach"))?;

        if thread.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_thread_attach"));
        }

        let result = function(thread.inner.cast());

        if result.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_thread_attach"));
        }

        Ok(UnityThread {
            inner: result.cast(),
        })
    }

    fn add_internal_call(&self, name: String, func: MethodPointer) -> Result<(), RuntimeError> {
        let function = &self.exports.clone().mono_add_internal_call.ok_or(RuntimeError::MissingFunction("mono_add_internal_call"))?;

        if name.is_empty() {
            return Err(RuntimeError::EmptyString);
        }

        if func.is_null() {
            return Err(RuntimeError::NullPointer("func"));
        }

        let name = CString::new(name.as_str())?;

        function(name.as_ptr(), func);

        Ok(())
    }

    fn install_assembly_hook(&self, hook_type: AssemblyHookType, func: MethodPointer) -> Result<(), RuntimeError> {
        if func.is_null() {
            return Err(RuntimeError::NullPointer("func"));
        }

        let hook_func = match hook_type {
            AssemblyHookType::Preload => self.exports.clone().mono_install_assembly_preload_hook,
            AssemblyHookType::Load => self.exports.clone().mono_install_assembly_load_hook,
            AssemblyHookType::Search => self.exports.clone().mono_install_assembly_search_hook,
        }.ok_or(RuntimeError::MissingFunction("mono_install_assembly_hook"))?;

        hook_func(func, std::ptr::null_mut());

        Ok(())
    }

    fn get_domain(&self) -> Result<UnityDomain, RuntimeError> {
        let function = &self.exports.clone().mono_get_root_domain.ok_or(RuntimeError::MissingFunction("mono_get_root_domain"))?;

        let domain = function();

        if domain.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_get_root_domain"));
        }

        Ok(UnityDomain {
            inner: domain.cast(),
        })
    }
}
