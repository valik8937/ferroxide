pub mod hooks;
pub mod android;
pub mod egl;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn eglGetDisplay(display_id: *mut libc::c_void) -> *mut libc::c_void {
    unsafe { egl::wrappers::ferroxide_eglGetDisplay(display_id) }
}
