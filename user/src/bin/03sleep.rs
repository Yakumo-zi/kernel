#![no_std]
#![no_main]



use user_lib::{get_time, yield_,sleep};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    sleep(3);
    println!("Test sleep OK!");
    0
}
