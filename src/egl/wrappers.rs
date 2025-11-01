generate_wrapper!(
    "libEGL.so",
    eglGetDisplay,
    (display_id: *mut libc::c_void) -> *mut libc::c_void
);
