#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod logger;
mod sbi;

use core::{arch::global_asm, usize};

use crate::logger::Logger;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    log::set_logger(&Logger)
        .map(|()| log::set_max_level(log::LevelFilter::Trace))
        .unwrap();
    clear_bss();
    panic!("Shutdown machine")
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn skernel();
        fn ekernel();
    }
    log::info!(".bss [{:#x}-{:#x}]", sbss as usize, ebss as usize);
    log::info!(".text [{:#x}-{:#x}]", stext as usize, etext as usize);
    log::info!(".data [{:#x}-{:#x}]", sdata as usize, edata as usize);
    log::info!(".rodata [{:#x}-{:#x}]", srodata as usize, erodata as usize);
    log::info!(
        ".load range [{:#x}-{:#x}]",
        skernel as usize,
        ekernel as usize,
    );
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
