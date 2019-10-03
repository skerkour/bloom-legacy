use libc::c_char;
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern "C" fn double_input(input: u64) -> u64 {
    input * 2
}

#[no_mangle]
pub extern "C" fn core_handle_message(message_char: *const c_char) -> *const c_char {
    let c_str: &CStr = unsafe { CStr::from_ptr(message_char) };
    let message_in: bloom_core::messages::Message = serde_json::from_str(c_str.to_str().expect("lol1")).expect("lol2");
    let message_out = bloom_core::handle_message(message_in);

    let message_out = serde_json::to_string(&message_out).expect("lol3");
    let ret = CString::new(message_out).expect("lol4");

    let ret_prt = ret.as_ptr();
    std::mem::forget(ret);
    return ret_prt;
}

#[no_mangle]
pub extern "C" fn core_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
