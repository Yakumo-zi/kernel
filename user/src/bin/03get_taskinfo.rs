#![no_std]
#![no_main]

use user_lib::get_taskinfo;
#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
   get_taskinfo();
    0
}
