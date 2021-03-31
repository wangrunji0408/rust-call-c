extern crate libc;

use libc::c_int;
use libc::size_t;
// use libc::c_void;
use libc::c_char;
use std::ffi::CString;

pub enum PMEMobjpool {}

mod ffi;
// #[link(name = "test")]
// extern "C"{
//     fn create_content(size: size_t, buf: *mut c_char, id: c_int)->c_int;
//     fn print_content(id: c_int)->c_void;
//     fn print_content_all()->c_void;
//     fn my_free(id: c_int)->c_void;
//     fn my_free_all()->c_void;
//     fn init(path:*const c_char)->*mut PMEMobjpool;
//     fn fin(pop:*mut PMEMobjpool);
// }

fn main() {
    let path = "array".to_string();
    let path = CString::new(path.as_str()).unwrap();
    // let pop = unsafe{ init(path.as_ptr())};
    let pop = ffi::my_init(path.as_ptr());
    let size1: size_t = 10;
    let size2: size_t = 12;
    let size3: size_t = 14;
    let buf1 = "hellowolrd".to_string();
    let buf2 = "sadjkjsaasd".to_string();
    let buf3 = "asdajosajsaa".to_string();
    let id1 = 1;
    let id2 = 2;
    let id3 = 3;
    // unsafe{
    //     create_content(size1, buf1.as_ptr() as *mut c_char, id1);
    //     create_content(size2, buf2.as_ptr() as *mut c_char, id2);
    //     create_content(size3, buf3.as_ptr() as *mut c_char, id3);
    //     print_content_all();
    //     my_free(id2);
    //     print_content_all();
    //     print_content(id1);
    //     my_free_all();
    //     print_content_all();
    // }
    ffi::my_create_content(pop, size1, buf1.as_ptr() as *mut c_char, id1);
    ffi::my_create_content(pop, size2, buf2.as_ptr() as *mut c_char, id2);
    ffi::my_create_content(pop, size3, buf3.as_ptr() as *mut c_char, id3);
    ffi::my_print_content_all(pop);
    ffi::my_free(pop, id2);
    ffi::my_print_content_all(pop);
    ffi::my_print_content(pop, id1);
    ffi::my_free_all(pop);
    ffi::my_print_content_all(pop);
    // unsafe{ fin(pop) };
    ffi::my_fin(pop);
}
