use std::os::raw::c_char;
use std::ffi::CString;
use std::ptr::null;

#[no_mangle]
pub extern "C" fn string_destroy(string: *mut CString) {
    unsafe {
        if !string.is_null() {
            Box::from_raw(string);
        }
    }
}

#[no_mangle]
pub extern "C" fn string_as_cstr(string: *mut CString) -> *const c_char {
    if !string.is_null() {
        unsafe {
            string
                .as_ref()
                .map(|v| v.as_ptr())
                .unwrap_or(null())
        }
    } else {
        null()
    }
}
