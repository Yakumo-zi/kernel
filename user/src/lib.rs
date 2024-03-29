#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

use core::iter::Iterator;

#[macro_use]
pub mod console;

mod lang_items;
mod syscall;
pub use console::*;
pub use syscall::*;
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!")
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}


pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
pub fn get_taskinfo(ts:*mut TaskInfo) -> isize {
    sys_task_info(ts)
}
pub fn yield_() {
    sys_yield();
}

pub fn get_time()->isize{
    sys_get_time()
}
pub fn sleep(sec:usize){
    sys_sleep(sec);
}