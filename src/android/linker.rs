use lazy_static::lazy_static;
use libloading::Library;
use libc::c_void;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref LOADED_LIBS: Mutex<HashMap<String, Library>> =
        Mutex::new(HashMap::new());
}

pub fn get_android_symbol(lib_name: &str, symbol_name: &str) -> Option<*mut c_void> {
    // Лок без паніки
    let mut libs = match LOADED_LIBS.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };

    // Завантажуємо .so якщо ще не
    if !libs.contains_key(lib_name) {
        match unsafe { Library::new(lib_name) } {
            Ok(lib) => {
                libs.insert(lib_name.to_string(), lib);
            }
            Err(_) => return None,
        }
    }

    // Тепер дістаємо символ
    if let Some(lib) = libs.get(lib_name) {
        let name_bytes = symbol_name.as_bytes();
        unsafe {
            // Беремо як "будь-яку" C-функцію й трансмутимо в *mut c_void
            type AnyFn = unsafe extern "C" fn();
            match lib.get::<AnyFn>(name_bytes) {
                Ok(sym) => {
                    let f: AnyFn = *sym;
                    Some(core::mem::transmute::<AnyFn, *mut c_void>(f))
                }
                Err(_) => None,
            }
        }
    } else {
        None
    }
}
