use libc::c_void;
use std::any::Any;

#[allow(non_camel_case_types)]
pub enum ApiCall {
    eglGetDisplay {
        display_id: *mut c_void,
    },
}

pub fn pre_call_hook(call: &ApiCall) {
    match call {
        ApiCall::eglGetDisplay { display_id } => {
            eprintln!("[ferroxide] PRE: Calling eglGetDisplay with id: {:?}", display_id);
        }
    }
}

pub fn post_call_hook(call: &ApiCall, result: &dyn Any) {
    match call {
        ApiCall::eglGetDisplay { .. } => {
            if let Some(res) = result.downcast_ref::<*mut c_void>() {
                eprintln!("[ferroxide] POST: eglGetDisplay returned: {:?}", res);
            }
        }
    }
}
