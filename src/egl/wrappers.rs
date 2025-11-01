use libc::c_void;

generate_wrapper!(
    "libEGL.so",
    eglGetDisplay,
    (display_id: *mut c_void) -> *mut c_void
);
