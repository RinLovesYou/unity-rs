//! TODO

use std::ffi::c_char;

use crate::libs::{LibError, NativeLibrary, NativeMethod};

use super::types::{Il2CppDomain, Il2CppThread};

/// Various methods exported by il2cpp
///
/// These are direct function pointers, and as such can be hooked
#[derive(Debug, Clone)]
pub struct Il2CppExports {
    /// initializes an il2cpp domain, this is called by unity itself
    pub il2cpp_init: Option<NativeMethod<fn(*const c_char) -> *mut Il2CppDomain>>,
    /// returns the current thread
    pub il2cpp_thread_current: Option<NativeMethod<fn() -> *mut Il2CppThread>>,
}

impl Il2CppExports {
    /// looks up and returns all methods from il2cpp
    pub fn new(lib: &NativeLibrary) -> Result<Il2CppExports, LibError> {
        Ok(Il2CppExports {
            il2cpp_init: Some(lib.sym("il2cpp_init")?),
            il2cpp_thread_current: Some(lib.sym("il2cpp_thread_current")?),
        })
    }
}
