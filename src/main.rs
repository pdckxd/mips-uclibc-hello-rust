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


use std::io::prelude::*;
use std::net::TcpStream;
fn main() -> std::io::Result<()> {
    let mut connection = TcpStream::connect("www.baidu.com:80")?;
    connection.write_all(b"GET / HTTP/1.0")?;
    connection.write_all(b"\r\n")?;
    connection.write_all(b"Host: www.baidu.com")?;
    connection.write_all(b"\r\n\r\n")?;
    std::io::copy(&mut connection, &mut std::io::stdout())?;
    Ok(())
}

/*
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    let builder = reqwest::Client::builder();
    let res = builder.danger_accept_invalid_certs(true)
        .build().unwrap().get("https://httpbin.org/ip")
        .send().await?;
    println!("{:#?}", res.text().await?);
    Ok(())
}
 */

/*
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
*/