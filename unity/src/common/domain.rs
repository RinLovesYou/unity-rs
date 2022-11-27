use std::ffi::c_void;

#[derive(Debug)]
pub struct UnityDomain {
    pub inner: *mut c_void,
}

unsafe impl Send for UnityDomain {}
unsafe impl Sync for UnityDomain {}

impl Clone for UnityDomain {
    fn clone(&self) -> UnityDomain {
        UnityDomain { ..*self }
    }
}
