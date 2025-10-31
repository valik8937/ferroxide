use crate::egl::wrappers::ferroxide_eglGetDisplay;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use libc::{c_char, c_void};
use std::ffi::CStr;
use std::sync::Mutex;

type DlsymFunc = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;

lazy_static! {
    static ref REAL_DLSYM: Mutex<Option<Symbol<'static, DlsymFunc>>> = Mutex::new(None);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void {
    let mut real_dlsym = REAL_DLSYM.lock().unwrap();

    if real_dlsym.is_none() {
        unsafe {
            let libdl = Box::leak(Box::new(Library::new("libdl.so.2").unwrap()));
            let dlsym_symbol: Symbol<DlsymFunc> = libdl.get(b"dlsym\0").unwrap();
            *real_dlsym = Some(dlsym_symbol);
        }
    }

    let symbol_str = unsafe { CStr::from_ptr(symbol).to_string_lossy() };

    match symbol_str.as_ref() {
        "eglGetDisplay" => {
            println!("[ferroxide dlsym] Intercepting 'eglGetDisplay', returning our wrapper.");
            return ferroxide_eglGetDisplay as *mut c_void;
        }
        _ => {
            println!("dlsym hook: {}", symbol_str);
        }
    }

    if let Some(ref dlsym_func) = *real_dlsym {
        unsafe { dlsym_func(handle, symbol) }
    } else {
        // This should not happen
        std::ptr::null_mut()
    }
}
