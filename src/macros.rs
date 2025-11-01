#[macro_export]
macro_rules! generate_wrapper {
    ($lib_name:expr, $func_name:ident, ($($arg_name:ident: $arg_type:ty),*) -> $ret_type:ty) => {
        pub unsafe extern "C" fn $func_name($($arg_name: $arg_type),*) -> $ret_type {
            type RealFn = unsafe extern "C" fn($($arg_type),*) -> $ret_type;

            let call_info = $crate::hooks::ApiCall::$func_name {
                $($arg_name),*
            };
            $crate::hooks::pre_call_hook(&call_info);

            let result = match $crate::android::linker::get_android_symbol($lib_name, stringify!($func_name)) {
                Some(ptr) if !ptr.is_null() => {
                    let real_fn: RealFn = unsafe { std::mem::transmute(ptr) };
                    unsafe { real_fn($($arg_name),*) }
                }
                _ => {
                    // Symbol not found or is null, return a default value
                    unsafe { std::mem::zeroed() }
                }
            };

            $crate::hooks::post_call_hook(&call_info, &result);
            result
        }
    };
}
