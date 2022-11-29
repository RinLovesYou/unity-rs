use std::ffi::c_void;

#[derive(Debug)]
pub struct UnityThread {
    pub inner: *mut c_void,
}

unsafe impl Send for UnityThread {}
unsafe impl Sync for UnityThread {}

impl Clone for UnityThread {
    fn clone(&self) -> UnityThread {
        UnityThread { ..*self }
    }
}
