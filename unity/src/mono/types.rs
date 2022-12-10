//! TODO

use std::{ffi::*, ptr::{addr_of_mut, addr_of}, error};

use thiserror::Error;

use crate::runtime::{Runtime, UnityRuntime};

/// the appdomain
#[derive(Debug)]
#[repr(C)]
pub struct MonoDomain {}

/// a thread
#[derive(Debug)]
#[repr(C)]
pub struct MonoThread {}

#[derive(Debug, Error)]
pub enum MonoMethodError {
    #[error("not running under mono")]
    NotMono,
    #[error("failed to get runtime")]
    GetRuntimeFailed,
    #[error("failed to get assembly")]
    GetImageFailed,
    #[error("Missing Function: {0}")]
    MissingFunction(String),
    #[error("Failed to create CString: {0}")]
    CStringCreationFailed(String),
    #[error("Method returned null: {0}")]
    NullMethod(String),
    #[error("Managed Exception: {0}")]
    ManagedException(String),
}

/// a method
#[derive(Debug)]
#[repr(C)]
pub struct MonoMethod {}

impl MonoMethod {
    pub fn invoke(this: *mut Self, obj: Option<*mut MonoObject>, params: Option<&mut Vec<*mut c_void>>) -> Result<Option<*mut MonoObject>, Box<dyn error::Error>> {
        let runtime = Runtime::new()?;

        match runtime.runtime {
            UnityRuntime::MonoRuntime(mono) => {
                if let Some(invoke) = mono.exports.mono_runtime_invoke {
                    let exc: *mut MonoObject = std::ptr::null_mut();
                    let object = match obj {
                        Some(obj) => obj,
                        None => std::ptr::null_mut(),
                    };

                    let params = match params {
                        Some(params) => addr_of_mut!(params[0]),
                        None => std::ptr::null_mut(),
                    };

                    let result = invoke(this, object, params, exc as *mut *mut MonoObject);

                    if exc.is_null() {
                        match result.is_null() {
                            false => Ok(Some(result)),
                            true => Ok(None),
                        }
                    } else {
                        Err(Box::new(MonoMethodError::ManagedException("TODO: Managed Stacktrace".to_string())))
                    }

                } else {
                    Err(Box::new(MonoMethodError::MissingFunction("mono_runtime_invoke".to_string())))
                }
            },
            UnityRuntime::Il2Cpp(_) => Err(Box::new(MonoMethodError::NotMono)),
        }
    }

    pub fn get_name(this: *mut Self) -> Result<String, MonoMethodError> {
        let runtime = Runtime::new().map_err(|_| MonoMethodError::GetRuntimeFailed)?;

        match runtime.runtime {
            UnityRuntime::MonoRuntime(mono) => {
                if let Some(get_name) = mono.exports.mono_method_get_name {
                    let name = get_name(this);
                    if name.is_null() {
                        Err(MonoMethodError::NullMethod("mono_method_get_name".to_string()))
                    } else {
                        let name = unsafe { CStr::from_ptr(name) };
                        Ok(name.to_string_lossy().to_string())
                    }
                } else {
                    Err(MonoMethodError::MissingFunction("mono_method_get_name".to_string()))
                }
            },
            UnityRuntime::Il2Cpp(_) => Err(MonoMethodError::NotMono),
        }
    }
}

#[derive(Debug, Error)]
pub enum MonoClassError {
    #[error("not running under mono")]
    NotMono,
    #[error("failed to get runtime")]
    GetRuntimeFailed,
    #[error("failed to get assembly")]
    GetImageFailed,
    #[error("Missing Function: {0}")]
    MissingFunction(String),
    #[error("Failed to create CString: {0}")]
    CStringCreationFailed(String),
    #[error("Method returned null: {0}")]
    NullMethod(String),
}

/// a class
#[derive(Debug)]
#[repr(C)]
pub struct MonoClass {}

impl MonoClass {
    pub fn get_method(this: *mut Self, name: impl Into<String>, param_count: i32) -> Result<*mut MonoMethod, MonoClassError> {
        let runtime = Runtime::new().map_err(|_| MonoClassError::GetRuntimeFailed)?;

        match runtime.runtime {
            UnityRuntime::MonoRuntime(mono) => {
                let name = CString::new(name.into()).map_err(|e| MonoClassError::CStringCreationFailed(e.to_string()))?;
                if let Some(get_method) = mono.exports.mono_class_get_method_from_name {
                    let method = get_method(this, name.as_ptr(), param_count);
                    if method.is_null() {
                        Err(MonoClassError::NullMethod("mono_class_get_method_from_name".to_string()))
                    } else {
                        Ok(method)
                    }
                } else {
                    Err(MonoClassError::MissingFunction("mono_class_get_method_from_name".to_string()))
                }
            },
            UnityRuntime::Il2Cpp(_) => Err(MonoClassError::NotMono),
        }
    }
}

#[derive(Debug, Error)]
pub enum MonoAssemblyError {
    #[error("not running under mono")]
    NotMono,
    #[error("failed to get runtime")]
    GetRuntimeFailed,
    #[error("failed to get assembly")]
    GetImageFailed,
    #[error("Missing Function: {0}")]
    MissingFunction(String),
    #[error("Failed to create C-String {0}")]
    CStringCreationFailed(String),
    #[error("Failed to get Domain")]
    GetDomainFailed,
}

/// an assembly
#[derive(Debug)]
#[repr(C)]
pub struct MonoAssembly {}

impl MonoAssembly {
    pub fn open(path: impl Into<String>) -> Result<*mut Self, MonoAssemblyError> {
        let path = CString::new(path.into()).map_err(|e| MonoAssemblyError::CStringCreationFailed(e.to_string()))?;
        let runtime = Runtime::new().map_err(|_| MonoAssemblyError::GetRuntimeFailed)?;

        match runtime.runtime {
            UnityRuntime::MonoRuntime(mono) => {
                let domain = mono.get_domain().map_err(|_| MonoAssemblyError::GetDomainFailed)?;
                if let Some(open) = mono.exports.mono_domain_assembly_open {
                    let assembly = open(domain, path.as_ptr());
                    if assembly.is_null() {
                        Err(MonoAssemblyError::GetImageFailed)
                    } else {
                        Ok(assembly)
                    }
                } else {
                    Err(MonoAssemblyError::MissingFunction("mono_domain_assembly_open".to_string()))
                }
            }
            _ => Err(MonoAssemblyError::NotMono),
        }
    }

    pub fn as_object(this: *mut Self) -> Result<*mut MonoObject, MonoAssemblyError> {
        let runtime = Runtime::new().map_err(|_| MonoAssemblyError::GetRuntimeFailed)?;

        match &runtime.runtime {
            UnityRuntime::MonoRuntime(mono) => {
                if let Some(as_object) = &mono.exports.mono_assembly_get_object {
                    let domain = mono.get_domain().map_err(|_| MonoAssemblyError::GetDomainFailed)?;
                    let object = as_object(domain, this);
                    if object.is_null() {
                        Err(MonoAssemblyError::GetImageFailed)
                    } else {
                        Ok(object)
                    }
                } else {
                    Err(MonoAssemblyError::MissingFunction("mono_assembly_get_object".to_string()))
                }
            }
            _ => Err(MonoAssemblyError::NotMono),
        }
    }
}

#[derive(Debug, Error)]
pub enum MonoImageError {
    #[error("not running under mono")]
    NotMono,
    #[error("failed to get runtime")]
    GetRuntimeFailed,
    #[error("failed to get image")]
    GetImageFailed,
    #[error("Missing Function: {0}")]
    MissingFunction(String),
    #[error("Failed to get Assembly: {0}")]
    GetAssemblyFailed(String),
}

/// a mono image
#[derive(Debug)]
#[repr(C)]
pub struct MonoImage {}

impl MonoImage {
    pub fn open(path: impl Into<String>) -> Result<*mut Self, MonoImageError> {
        let assembly = MonoAssembly::open(path).map_err(|e| MonoImageError::GetAssemblyFailed(e.to_string()))?;
        MonoImage::from_assembly(assembly)
    }

    pub fn from_assembly(assembly: *mut MonoAssembly) -> Result<*mut Self, MonoImageError> {
        let runtime = Runtime::new().map_err(|_| MonoImageError::GetRuntimeFailed)?;

        match runtime.runtime {
            UnityRuntime::MonoRuntime(mono) => {
                if let Some(get_image) = mono.exports.mono_assembly_get_image {
                    let image = get_image(assembly);

                    if image.is_null() {
                        Err(MonoImageError::GetImageFailed)
                    } else {
                        Ok(image)
                    }
                } else {
                    Err(MonoImageError::MissingFunction("mono_assembly_get_image".to_string()))
                }
            },
            _ => Err(MonoImageError::NotMono),
        }
    }

    pub fn get_class(this: *mut Self, namespace: impl Into<String>, name: impl Into<String>) -> Result<*mut MonoClass, MonoImageError> {
        let runtime = Runtime::new().map_err(|_| MonoImageError::GetRuntimeFailed)?;

        match runtime.runtime {
            UnityRuntime::MonoRuntime(mono) => {
                if let Some(get_class) = mono.exports.mono_class_from_name {
                    let namespace = CString::new(namespace.into()).map_err(|e| MonoImageError::GetAssemblyFailed(e.to_string()))?;
                    let name = CString::new(name.into()).map_err(|e| MonoImageError::GetAssemblyFailed(e.to_string()))?;

                    let class = get_class(this, namespace.as_ptr(), name.as_ptr());

                    if class.is_null() {
                        Err(MonoImageError::GetImageFailed)
                    } else {
                        Ok(class)
                    }
                } else {
                    Err(MonoImageError::MissingFunction("mono_class_from_name".to_string()))
                }
            },
            _ => Err(MonoImageError::NotMono),
        }               
    }
}

/// a mono string
#[derive(Debug)]
#[repr(C)]
pub struct MonoString {}

/// a mono object
#[derive(Debug)]
#[repr(C)]
pub struct MonoObject {
    /// the vtable
    pub vtable: *mut c_void,
    /// the sync
    pub syncchronisation: *mut c_void,
}

/// a reflection assembly
#[derive(Debug)]
#[repr(C)]
pub struct MonoReflectionAssembly {
    /// the object
    pub object: MonoObject,
    /// the assembly
    pub assembly: *mut MonoAssembly,
    /// evidence
    pub evidence: *mut MonoObject,
}

/// an assembly name
#[derive(Debug)]
#[repr(C)]
pub struct AssemblyName {
    pub name: *mut c_char,
    pub culture: *mut c_char,
    pub hash_value: *mut c_char,
    pub public_key: *mut c_char,

    pub public_key_token: [c_char; 17],

    pub hash_alg: u32,
    pub hash_len: u32,

    pub flags: u32,
    pub major: c_ushort,
    pub minor: c_ushort,
    pub build: c_ushort,
    pub revision: c_ushort,
    pub arch: u32,
}

#[derive(Debug, Error)]
pub enum MonoStringError {
    #[error("not running under mono")]
    NotMono,
    #[error("mono_string_to_utf8 failed")]
    Utf8Failed,
    #[error("failed to create string")]
    CreateFailed,
    #[error("failed to get runtime")]
    GetRuntimeFailed,
    #[error("Failed to get Domain")]
    GetDomainFailed,
}

impl MonoString {
    pub fn new(string: impl Into<String>) -> Result<*mut Self, MonoStringError> {
        let string = string.into();
        let string = CString::new(string).unwrap();
        
        let runtime = Runtime::new().map_err(|_| MonoStringError::GetRuntimeFailed)?;

        match runtime.runtime {
            UnityRuntime::Il2Cpp(_) => Err(MonoStringError::NotMono),
            UnityRuntime::MonoRuntime(mono) => {
                let domain = mono.get_domain().map_err(|_| MonoStringError::GetDomainFailed)?;
                match mono.exports.mono_string_new {
                    Some(mono_string_new) => {
                        let managed_string = mono_string_new(domain, string.as_ptr());

                        match managed_string.is_null() {
                            true => Err(MonoStringError::CreateFailed),
                            false => Ok(managed_string),
                        }
                    }
                    None => Err(MonoStringError::CreateFailed),
                }

            }
        }
    }

    pub fn from_raw(raw: *mut c_char) -> Result<*mut Self, MonoStringError> {
        let runtime = Runtime::new().map_err(|_| MonoStringError::GetRuntimeFailed)?;

        match runtime.runtime {
            UnityRuntime::Il2Cpp(_) => Err(MonoStringError::NotMono),
            UnityRuntime::MonoRuntime(mono) => {
                let domain = mono.get_domain().map_err(|_| MonoStringError::GetDomainFailed)?;
                match mono.exports.mono_string_new {
                    Some(mono_string_new) => {
                        let managed_string = mono_string_new(domain, raw);

                        match managed_string.is_null() {
                            true => Err(MonoStringError::CreateFailed),
                            false => Ok(managed_string),
                        }
                    }
                    None => Err(MonoStringError::CreateFailed),
                }

            }
        }
    }
}