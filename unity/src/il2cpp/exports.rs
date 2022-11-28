use std::ffi::c_char;

use crate::libs::{LibError, NativeLibrary, NativeMethod};

use super::types::Il2CppDomain;

#[derive(Debug, Clone)]
pub struct Il2CppExports {
    pub il2cpp_init: Option<NativeMethod<fn(*const c_char) -> *mut Il2CppDomain>>,
}

impl Il2CppExports {
    pub fn new(lib: &NativeLibrary) -> Result<Il2CppExports, LibError> {
        Ok(Il2CppExports {
            il2cpp_init: Some(lib.sym("il2cpp_init")?),
        })
    }
}
