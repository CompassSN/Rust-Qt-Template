use std::{env, ffi::CString};

fn main() {
    let args: Vec<_> = env::args().collect();
    unsafe{
        ffi::init(args.len() as i32, make_c_argv(args));
    }
}

unsafe fn make_c_argv(args: Vec<String>) -> usize{
    let argvvec: Vec<_> = args
        .into_iter()
        .map(|arg| {
                let cstr = CString::new(arg).unwrap();
                cstr.as_ptr()
            }).collect();
    argvvec.as_ptr() as usize
}

#[cxx::bridge]
mod ffi{
    unsafe extern "C++"{
        include!("qt/cpp/lib.h");
        unsafe fn init(argc: i32, argv: usize) -> i32;
    }
}
