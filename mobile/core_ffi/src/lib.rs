use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn double_input(input: u64) -> u64 {
    input * 2
}

#[no_mangle]
pub unsafe extern "C" fn core_handle_message(message_char: *const c_char) -> *const c_char {
    use log::Level;
    use log::{debug, error};
    android_logger::init_once(android_logger::Config::default().with_min_level(Level::Trace));
    debug!("Rust core_handle_message");

    let c_str: &CStr = CStr::from_ptr(message_char);

    let message_out: bloom_core::messages::Message =
        match serde_json::from_str(c_str.to_str().expect("lol1")) {
            Ok(res) => bloom_core::handle_message(res),
            Err(err) => {
                error!("error deserialazing json: {}", err);
                let err: bloom_core::error::BloomError = err.into();
                let err: bloom_core::messages::kernel::Error = err.into();
                err.into()
            }
        };
    let message_out = serde_json::to_string(&message_out).expect("lol3");

    let ret = CString::new(message_out).expect("lol4");

    return ret.into_raw();
}

#[no_mangle]
pub unsafe extern "C" fn core_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}
