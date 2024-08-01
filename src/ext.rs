
use std::ffi::{c_char, CStr, CString};
use crate::{get_bytes_for_pdf, print_html_to_pdf};
use std::default::Default;
use std::ptr::null;

#[repr(C)]
#[no_mangle]
pub struct Buffer {
    data: *mut u8,
    len: usize,
    err: i8
}

impl Default for Buffer {
    fn default() -> Self {
        unsafe { Self { data: *null(), len: 0, err: 0 } }
    }
}
#[no_mangle]
pub extern "C" fn c_print_pdf_to_file(path_from: *mut c_char,path_to: *mut c_char) -> c_char{
    // let rust_string: String = get_hello_world();
    unsafe {
        if path_from.is_null() || path_to.is_null() {
            return 21
        }
        let path_from = CStr::from_ptr(path_from).to_string_lossy().into_owned();
        let path_to = CStr::from_ptr(path_to).to_string_lossy().into_owned();
        let res = print_html_to_pdf(path_from, path_to);


        return res
    }
}
#[no_mangle]
pub extern "C" fn c_get_pdf_bytes(path_from: *mut c_char) -> Buffer{
    // let rust_string: String = get_hello_world();
    unsafe {
        if path_from.is_null()  {
            return Buffer{err: 31, ..Default::default()}
        }
        let path_from = CStr::from_ptr(path_from).to_string_lossy().into_owned();
        let res = get_bytes_for_pdf(path_from);
        match res {
            Ok(arr) => {
                let mut buf = arr.into_boxed_slice();
                let data = buf.as_mut_ptr();
                let len = buf.len();
                let k = Buffer { data, len , err: 0 };
                k
            }
            Err(e) => {
                println!("{e}");
                Buffer{err: 32, ..Default::default()}
            }
        }
    }
}