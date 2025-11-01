#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod hooks;
pub mod android;
pub mod egl;
pub mod registry;

use libc::{c_char, c_void};
use std::ffi::CStr;
use std::sync::Mutex;

lazy_static! {
    static ref REAL_EGL_GET_PROC_ADDRESS:
        Mutex<Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>>
        = Mutex::new(None);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn eglGetProcAddress(procname: *const c_char) -> *mut c_void {
    if procname.is_null() {
        return core::ptr::null_mut();
    }

    // Працюємо з байтами (не ламаємось на не-UTF-8)
    let name_bytes = unsafe { CStr::from_ptr(procname).to_bytes() };

    // 1) Якщо ми перехоплюємо цей символ — повертаємо нашу обгортку
    if let Some(ptr) = crate::registry::get(name_bytes) {
        return ptr;
    }

    // 2) Інакше — дзвонимо в оригінальний eglGetProcAddress
    let mut guard = match REAL_EGL_GET_PROC_ADDRESS.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };

    if guard.is_none() {
        if let Some(raw) = android::linker::get_android_symbol("libEGL.so", "eglGetProcAddress") {
            if !raw.is_null() {
                // Безпечно зберігаємо як fn-покажчик
                unsafe {
                    *guard = Some(core::mem::transmute(raw));
                }
            }
        }
    }

    if let Some(real_fn) = *guard {
        unsafe { real_fn(procname) }
    } else {
        core::ptr::null_mut()
    }
}
