use crate::android::linker::get_android_symbol;
use libc::c_void;
use std::mem;

type RealEglGetDisplay = unsafe extern "C" fn(display_id: *mut c_void) -> *mut c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ferroxide_eglGetDisplay(display_id: *mut c_void) -> *mut c_void {
    let real_func_ptr = get_android_symbol("libEGL.so", "eglGetDisplay");

    if let Some(ptr) = real_func_ptr {
        if ptr.is_null() {
            return std::ptr::null_mut();
        }
        
        unsafe {
            let real_egl_get_display: RealEglGetDisplay = mem::transmute(ptr);
            return real_egl_get_display(display_id);
        }

    } else {
        return std::ptr::null_mut();
    }
}
