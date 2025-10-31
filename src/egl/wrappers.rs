use crate::android::linker::get_android_symbol;
use libc::c_void;
use std::mem;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ferroxide_eglGetDisplay(display_id: *mut c_void) -> *mut c_void {
    println!("[ferroxide wrapper] eglGetDisplay called with display_id: {:?}", display_id);

    type RealEglGetDisplay = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

    let real_func_ptr = get_android_symbol("libEGL.so", "eglGetDisplay");

    if let Some(real_func_ptr) = real_func_ptr {
        if !real_func_ptr.is_null() {
            let real_eglGetDisplay: RealEglGetDisplay = unsafe { mem::transmute(real_func_ptr) };
            let result = unsafe { real_eglGetDisplay(display_id) };
            println!("[ferroxide wrapper] eglGetDisplay returned: {:?}", result);
            return result;
        }
    }

    println!("[ferroxide wrapper] Error: could not find real eglGetDisplay");
    std::ptr::null_mut()
}
