#![feature(rustc_private)]
extern crate libc;
use std::process::Command;
use libc::{c_char, strlen};
use std::{slice, str};
use std::ffi::CStr;

#[link(name = "curl")]
#[link(name = "ssl")]
#[link(name = "crypto")]
#[link(name = "z")]

extern "C"
{
    fn curl_version() -> *const c_char;
}

fn main() {
    // Demo how to call system cmd
    // let output = Command::new("sh")
    //     .arg("-c")
    //     .arg("curl http://www.baidu.com/")
    //     .output()
    //     .expect("failed to execute process");
    //
    // let hello = output.stdout;
    // println!("{}", String::from_utf8_lossy(&hello));
    // =============================

    // Demo how to call native C function
    let c_buf: *const c_char = unsafe { curl_version() };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    // another way to convert c_char to &str
    // let s = unsafe {
    //     let c_s = curl_version();
    //     str::from_utf8_unchecked(slice::from_raw_parts(c_s as *const u8, strlen(c_s) + 1))
    // };
    println!("curl_version output: {}", str_slice);
    // ============================
}
