//! TODO

use std::{error, path::PathBuf, ffi::CString};

use thiserror::Error;

use crate::{
    join_dll_path,
    libs::{self, NativeLibrary, NativeMethod}, runtime::{Runtime, RuntimeError, RuntimeType}, common::{thread::UnityThread, domain::UnityDomain, method::MethodPointer}, mono::AssemblyHookType,
};

use self::{exports::Il2CppExports, types::Il2CppThread};

pub mod exports;
pub mod types;

#[derive(Debug, Clone)]
pub struct Il2Cpp {
    pub game_assembly: NativeLibrary,
    pub exports: Il2CppExports,
}

impl Il2Cpp {
    pub fn new(base_path: PathBuf) -> Result<Self, RuntimeError> {
        let game_assembly_path = join_dll_path!(base_path, "GameAssembly");

        if !game_assembly_path.exists() {
            return Err(RuntimeError::GameAssemblyNotFound);
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

impl Runtime for Il2Cpp {
    fn get_type(&self) -> RuntimeType {
        RuntimeType::Il2Cpp(self)
    }

    fn get_export_ptr(&self, name: &str) -> Result<MethodPointer, RuntimeError> {
        let function: NativeMethod<fn()> = self.game_assembly.sym(name)?;

        if function.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("get_export_ptr"));
        }

        Ok(function.inner)
    }

    fn get_current_thread(&self) -> Result<UnityThread, RuntimeError> {
        let function = &self.exports.clone().il2cpp_thread_current.ok_or(RuntimeError::MissingFunction("il2cpp_thread_current"))?;
        let thread = function();

        if thread.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_thread_current"));
        }

        Ok(UnityThread { 
            inner: thread.cast() 
        })
    }

    /// this function doesn't exist in il2cpp, it just forwards to il2cpp_thread_attach
    fn set_main_thread(&self, thread: UnityThread) -> Result<(), RuntimeError> {
        let function = &self.exports.clone().il2cpp_thread_attach.ok_or(RuntimeError::MissingFunction("il2cpp_thread_attach"))?;

        if thread.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_thread_attach").into());
        }

        function(thread.inner.cast());

        Ok(())
    }

    fn attach_to_thread(&self, thread: UnityDomain) -> Result<UnityThread, RuntimeError> {
        let function = &self.exports.clone().il2cpp_thread_attach.ok_or(RuntimeError::MissingFunction("il2cpp_thread_attach"))?;

        if thread.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_thread_attach").into());
        }

        let thread = function(thread.inner.cast());

        if thread.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_thread_attach").into());
        }

        Ok(UnityThread {
            inner: thread.cast(),
        })
    }

    fn add_internal_call(&self, name: String, func: MethodPointer) -> Result<(), RuntimeError> {
        let function = &self.exports.clone().il2cpp_add_internal_call.ok_or(RuntimeError::MissingFunction("il2cpp_add_internal_call"))?;

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
        return Err(RuntimeError::NotImplemented("install_assembly_hook are mono only functions"));
    }
    
    fn get_domain(&self) -> Result<UnityDomain, RuntimeError> {
        let function = &self.exports.clone().il2cpp_domain_get.ok_or(RuntimeError::MissingFunction("il2cpp_domain_get"))?;

        let domain = function();

        if domain.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_domain_get"));
        }

        Ok(UnityDomain {
            inner: domain.cast(),
        })
    }
}
