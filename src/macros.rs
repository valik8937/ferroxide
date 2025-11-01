#[macro_export]
macro_rules! generate_wrapper {
    ($lib_name:expr, $func_name:ident, ($($arg_name:ident: $arg_type:ty),*) -> $ret_type:ty) => {
        paste::paste! {
            // Власне обгортка
            #[allow(non_snake_case)]
            pub unsafe extern "C" fn $func_name($($arg_name: $arg_type),*) -> $ret_type {
                type RealFn = unsafe extern "C" fn($($arg_type),*) -> $ret_type;

                // Хуки — не повинні панікувати
                let call_info = $crate::hooks::ApiCall::$func_name { $($arg_name),* };
                $crate::hooks::pre_call_hook(&call_info);

                // Шукаємо справжній символ і викликаємо; якщо немає — повертаємо "нуль" типу
                let result: $ret_type = match $crate::android::linker::get_android_symbol($lib_name, stringify!($func_name)) {
                    Some(ptr) if !ptr.is_null() => {
                        unsafe {
                            let real_fn: RealFn = core::mem::transmute(ptr);
                            real_fn($($arg_name),*)
                        }
                    }
                    _ => unsafe { core::mem::zeroed() },
                };

                $crate::hooks::post_call_hook(&call_info, &result);
                result
            }

            // АВТОРЕЄСТРАЦІЯ: кладемо адресу обгортки в глобальну мапу на етапі завантаження .so
            #[allow(non_snake_case)]
            #[ctor::ctor]
            fn [<__ferroxide_register_ $func_name>] () {
                // ім'я як bytes (працює навіть якщо у виклику прилетить не-UTF-8)
                let name: &'static [u8] = stringify!($func_name).as_bytes();
                let ptr = $func_name as *mut ::libc::c_void;
                $crate::registry::register(name, ptr);
            }
        }
    };
}
