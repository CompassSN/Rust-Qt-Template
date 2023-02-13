use tokio;
use std::{env, ffi::{CString, c_char}, os::unix::prelude::OsStrExt, ptr::null_mut};

#[tokio::main]
async fn main() {
    init_qapp();
}

fn init_qapp() {
    let args: Vec<CString> = env::args_os()
        .map(|arg|{
            let arg = arg.as_bytes();
            CString::new(arg).unwrap()
        }).collect();
    let argc = args.len();
    let mut argv = Vec::with_capacity(argc + 1);
    for arg in args{
        argv.push(arg.as_ptr() as *mut c_char)
    }
    argv.push(null_mut());
    unsafe{
        ffi::initqapp(argc as i32, argv.as_mut_ptr());
    }
}

#[cxx::bridge]
mod ffi{
    unsafe extern "C++"{
        include!("qt/cpp/lib.h");
        unsafe fn initqapp(argc: i32, argv: *mut *mut c_char);
    }
}
