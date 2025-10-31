use libc::{c_char, c_void, dlsym as libc_dlsym, RTLD_NEXT};
use std::ffi::CStr;
use std::mem;
use std::sync::Once;

type DlsymFunc = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;

static mut REAL_DLSYM: *mut c_void = std::ptr::null_mut();
static INIT: Once = Once::new();

fn get_real_dlsym() -> Option<DlsymFunc> {
    INIT.call_once(|| {
        unsafe {
            REAL_DLSYM = libc_dlsym(RTLD_NEXT, "dlsym\0".as_ptr() as *const c_char);
        }
    });
    unsafe {
        if REAL_DLSYM.is_null() {
            None
        } else {
            Some(mem::transmute(REAL_DLSYM))
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void {
    // Загортаємо ВЕСЬ код в один unsafe блок, щоб задовольнити компілятор
    unsafe {
        if symbol.is_null() {
            if let Some(real_dlsym) = get_real_dlsym() {
                return real_dlsym(handle, symbol);
            } else {
                return std::ptr::null_mut();
            }
        }

        let symbol_str = CStr::from_ptr(symbol).to_string_lossy();
        
        if symbol_str == "dlsym" {
            return dlsym as *mut c_void;
        }
        
        if let Some(real_dlsym) = get_real_dlsym() {
            match symbol_str.as_ref() {
                "eglGetDisplay" => {
                    println!("[ferroxide dlsym] Intercepting 'eglGetDisplay', returning our wrapper.");
                    crate::egl::wrappers::ferroxide_eglGetDisplay as *mut c_void
                }
                _ => {
                    real_dlsym(handle, symbol)
                }
            }
        } else {
            std::ptr::null_mut()
        }
    }
}