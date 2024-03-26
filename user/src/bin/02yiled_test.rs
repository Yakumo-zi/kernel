
#![no_std]
#![no_main]

use user_lib::yield_;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    for i in 0..5{
        println!("yiled_test [{}/5]",i+1);
        yield_();
    }
    0
}
