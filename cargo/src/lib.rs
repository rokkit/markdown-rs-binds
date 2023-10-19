use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern fn render_html(tpl: *const c_char) -> *mut c_char {
    let c_str: &CStr = unsafe { CStr::from_ptr(tpl) };
    let recipient = match c_str.to_str() {
        Err(_) => "Bad template",
        Ok(string) => string,
    };
    return CString::new(markdown::to_html(c_str.to_str().unwrap())).unwrap().into_raw()
}