use lazy_static::lazy_static;
use libc::c_void;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    // Ключ — байтове ім'я функції (без \0), значення — адреса обгортки
    static ref MAP: Mutex<HashMap<&'static [u8], usize>> =
        Mutex::new(HashMap::new());
}

/// Безпечна реєстрація: ігноруємо poison, ніколи не панікуємо.
pub fn register(name: &'static [u8], ptr: *mut c_void) {
    let addr = ptr as usize;
    match MAP.lock() {
        Ok(mut m) => { m.insert(name, addr); }
        Err(e) => {
            // Ігноруємо poison і все одно пробуємо оновити мапу
            let mut m = e.into_inner();
            m.insert(name, addr);
        }
    }
}

/// Пошук за байтовою назвою (працює і з не-UTF-8)
pub fn get(name: &[u8]) -> Option<*mut c_void> {
    match MAP.lock() {
        Ok(m) => m.get(name).map(|addr| *addr as *mut c_void),
        Err(e) => e.into_inner().get(name).map(|addr| *addr as *mut c_void),
    }
}
