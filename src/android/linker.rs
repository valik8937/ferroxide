use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use libc::c_void;
use std::collections::HashMap;
use std::sync::Mutex;

const ANDROID_LIB_PATHS: &[&str] = &[
    "/system/lib64",
    "/vendor/lib64",
    "/vendor/lib64/egl",
];

lazy_static! {
    static ref LOADED_LIBS: Mutex<HashMap<String, Library>> = Mutex::new(HashMap::new());
}

pub fn get_android_symbol(lib_name: &str, symbol_name: &str) -> Option<*mut c_void> {
    let mut libs = LOADED_LIBS.lock().unwrap();

    if !libs.contains_key(lib_name) {
        println!("Cache miss for library: {}", lib_name);
        let mut found_lib: Option<Library> = None;
        for path in ANDROID_LIB_PATHS {
            let full_path = format!("{}/{}", path, lib_name);
            println!("Attempting to load library from: {}", full_path);
            match unsafe { Library::new(&full_path) } {
                Ok(lib) => {
                    println!("Successfully loaded library from: {}", full_path);
                    found_lib = Some(lib);
                    break;
                }
                Err(e) => {
                    println!("Failed to load library from: {}: {}", full_path, e);
                }
            }
        }

        if let Some(lib) = found_lib {
            libs.insert(lib_name.to_string(), lib);
        } else {
            println!("Library not found: {}", lib_name);
            return None;
        }
    } else {
        println!("Cache hit for library: {}", lib_name);
    }

    if let Some(lib) = libs.get(lib_name) {
        let symbol_name_bytes = symbol_name.as_bytes();
        unsafe {
            match lib.get(symbol_name_bytes) {
                Ok(symbol) => {
                    println!("Successfully found symbol '{}' in '{}'", symbol_name, lib_name);
                    let symbol: Symbol<unsafe extern "C" fn() -> ()> = symbol;
                    Some(symbol.into_raw().into_raw() as *mut c_void)
                }
                Err(e) => {
                    println!("Symbol '{}' not found in '{}': {}", symbol_name, lib_name, e);
                    None
                }
            }
        }
    } else {
        None
    }
}
