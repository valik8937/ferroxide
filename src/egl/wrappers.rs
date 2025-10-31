use crate::android::linker::get_android_symbol;
use libc::c_void;
use std::mem;

type RealEglGetDisplay = unsafe extern "C" fn(display_id: *mut c_void) -> *mut c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ferroxide_eglGetDisplay(display_id: *mut c_void) -> *mut c_void {
    println!("[ferroxide wrapper] ==> Entered ferroxide_eglGetDisplay.");
    println!("[ferroxide wrapper] --> Calling our linker to find real eglGetDisplay...");
    let real_func_ptr = get_android_symbol("libEGL.so", "eglGetDisplay");
    println!("[ferroxide wrapper] <-- Linker returned pointer: {:?}", real_func_ptr);

    if let Some(ptr) = real_func_ptr {
        if ptr.is_null() {
            println!("[ferroxide wrapper] ERROR: Linker returned a null pointer!");
            return std::ptr::null_mut();
        }
        
        println!("[ferroxide wrapper] --> Pointer is valid. Transmuting and calling real function...");
        
        // Додаємо unsafe блок, як просить компілятор
        unsafe {
            let real_egl_get_display: RealEglGetDisplay = mem::transmute(ptr);
            let result = real_egl_get_display(display_id);
            println!("[ferroxide wrapper] <-- Real function returned: {:?}", result);
            return result;
        }

    } else {
        println!("[ferroxide wrapper] ERROR: Linker returned None! Could not find symbol.");
        return std::ptr::null_mut();
    }
}