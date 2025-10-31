use lazy_static::lazy_static;
use libloading::{Library};
use libc::c_void;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref LOADED_LIBS: Mutex<HashMap<String, Library>> = Mutex::new(HashMap::new());
}

pub fn get_android_symbol(lib_name: &str, symbol_name: &str) -> Option<*mut c_void> {
    let mut libs = LOADED_LIBS.lock().unwrap();

    if !libs.contains_key(lib_name) {
        if let Ok(lib) = unsafe { Library::new(lib_name) } {
            libs.insert(lib_name.to_string(), lib);
        } else {
            return None;
        }
    }

    if let Some(lib) = libs.get(lib_name) {
        let symbol_name_bytes = symbol_name.as_bytes();
        unsafe {
            if let Ok(symbol) = lib.get::<unsafe extern "C" fn()>(symbol_name_bytes) {
                let raw_symbol = symbol.into_raw();
                Some(raw_symbol.into_raw() as *mut c_void)
            } else {
                None
            }
        }
    } else {
        None
    }
}
