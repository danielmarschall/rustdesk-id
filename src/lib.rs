use hbb_common::config::Config;

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rd_get_id() -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        let id = Config::get_id();

        CString::new(id)
            .ok()
            .map(|s| s.into_raw())
            .unwrap_or(std::ptr::null_mut())
    });

    match result {
        Ok(ptr) => ptr,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn rd_free_string(
    s: *mut c_char
)
{
    if s.is_null() {
        return;
    }

    unsafe {

        let _ = CString::from_raw(s);

    }

}
