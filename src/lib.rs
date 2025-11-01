#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod hooks;
pub mod android;
pub mod egl;

use std::collections::HashMap;
use std::ffi::{CStr};
use libc::{c_char, c_void};
use std::sync::Mutex;

lazy_static! {
    static ref REAL_EGL_GET_PROC_ADDRESS: Mutex<Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>> = Mutex::new(None);
    static ref INTERCEPT_MAP: Mutex<HashMap<&'static str, usize>> = {
        let mut m = HashMap::new();
        m.insert("eglGetDisplay", egl::wrappers::eglGetDisplay as usize);
        Mutex::new(m)
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn eglGetProcAddress(procname: *const c_char) -> *mut c_void {
    if procname.is_null() {
        return std::ptr::null_mut();
    }
    let procname_str = unsafe { CStr::from_ptr(procname).to_str().unwrap_or("") };

    let intercept_map = INTERCEPT_MAP.lock().unwrap();
    if let Some(intercepted_func_addr) = intercept_map.get(procname_str) {
        return *intercepted_func_addr as *mut c_void;
    }

    let mut real_egl_get_proc_address = REAL_EGL_GET_PROC_ADDRESS.lock().unwrap();
    if real_egl_get_proc_address.is_none() {
        if let Some(real_func_ptr) = android::linker::get_android_symbol("libEGL.so", "eglGetProcAddress") {
            if !real_func_ptr.is_null() {
                 unsafe {
                    *real_egl_get_proc_address = Some(std::mem::transmute(real_func_ptr));
                }
            }
        }
    }

    if let Some(real_fn) = *real_egl_get_proc_address {
        unsafe { real_fn(procname) }
    } else {
        std::ptr::null_mut()
    }
}
