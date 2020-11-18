use std::{str, ffi::CStr, os::raw::c_char};

#[no_mangle]
pub extern "C" fn int64_to_bytes(x: *mut [u8; 8], y: i64) -> i32 {
    if let true = x.is_null() { return -1 };
    let z = y.to_be_bytes();
    unsafe { std::ptr::write(x, z) };
    0
}
#[no_mangle]
pub extern "C" fn bytes_to_uint64(x: *const [u8; 8]) -> u64 {
    if let true = x.is_null() { panic!("Invalid pointer") };
    let z = unsafe { &*x };
    u64::from_be_bytes(*z)
}
#[no_mangle]
pub extern "C" fn bytes_to_int64(x: *const [u8; 8]) -> i64 {
    if let true = x.is_null() { panic!("Invalid pointer") };
    let z = unsafe { &*x };
    i64::from_be_bytes(*z)
}
#[no_mangle]
pub extern "C" fn validate_utf8(x: *const c_char) -> i32 {
    if let true = x.is_null() { return -1 };
    let y = unsafe { CStr::from_ptr(x) };
    let z = str::from_utf8(&y.to_bytes());
    if let true = z.is_err() { return -1 }
    0
}
#[no_mangle]
pub extern "C" fn utf8_to_bytes(x: *const c_char) -> *const c_char {
    if let true = x.is_null() { panic!("Invalid pointer"); };
    x
}
