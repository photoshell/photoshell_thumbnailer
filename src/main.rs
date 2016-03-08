extern crate libc;

use libc::c_int;
use libc::intptr_t;
use std::env;
use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "raw")]
extern {
    // Technically libraw_init returns a pointer to a libraw_data_t struct, but
    // since we're only using the pointer to pass back into libraw functions we
    // can pretend it's an int pointer.
    fn libraw_init(flags: c_int) -> intptr_t;
    fn libraw_open_file(data: intptr_t, filename: *const c_char);
    fn libraw_unpack_thumb(data: intptr_t);
    fn libraw_dcraw_thumb_writer(data: intptr_t, filename: *const c_char);
}

fn main() {
    // TODO: handle this more gracefully
    let input = env::args().nth(1).unwrap();
    let output = env::args().nth(2).unwrap();

    let input_c_string = CString::new(input).unwrap();
    let output_c_string = CString::new(output).unwrap();

    unsafe {
        let d = libraw_init(0);
        libraw_open_file(d, input_c_string.as_ptr());
        libraw_unpack_thumb(d);
        libraw_dcraw_thumb_writer(d, output_c_string.as_ptr());
    }
}
