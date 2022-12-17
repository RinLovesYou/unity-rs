//! TODO

use std::{ffi::*, ptr::{addr_of_mut, addr_of}, error};

use thiserror::Error;

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