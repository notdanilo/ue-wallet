use std::ffi::c_void;
use std::ptr::null_mut;

#[no_mangle]
pub extern "C" fn vector_length(vector: *mut Vec<*mut c_void>) -> i32 {
    unsafe {
        vector
            .as_ref()
            .map(|v| v.len() as i32)
            .unwrap_or_default()
    }
}

#[no_mangle]
pub extern "C" fn vector_destroy(vector: *mut Vec<*mut c_void>) {
    unsafe {
        if !vector.is_null() {
            Box::from_raw(vector);
        }
    }
}

#[no_mangle]
pub extern "C" fn vector_pointer(vector: *mut Vec<*mut c_void>) -> *mut *mut c_void {
    unsafe {
        vector
            .as_mut()
            .map(|v| v.as_mut_ptr())
            .unwrap_or(null_mut())
    }
}
