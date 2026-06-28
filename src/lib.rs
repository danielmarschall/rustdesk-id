use hbb_common::config::Config;

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rd_get_id() -> *mut c_char {

    let id = Config::get_id();

    CString::new(id)
        .unwrap()
        .into_raw()

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
