#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod counter;
mod lang_items;
mod loader;
mod logger;
mod sbi;
mod sync;
mod syscall;
mod trap;
mod tasks;
mod config;

use core::{arch::global_asm, usize};

use crate::logger::Logger;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
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
    log::set_logger(&Logger)
        .map(|()| log::set_max_level(log::LevelFilter::Trace))
        .unwrap();

    log::info!(".bss [{:#x}-{:#x}]", sbss as usize, ebss as usize);
    log::info!(".text [{:#x}-{:#x}]", stext as usize, etext as usize);
    log::info!(".data [{:#x}-{:#x}]", sdata as usize, edata as usize);
    log::info!(".rodata [{:#x}-{:#x}]", srodata as usize, erodata as usize);
    log::info!(
        ".load range [{:#x}-{:#x}]",
        skernel as usize,
        ekernel as usize,
    );

    clear_bss();
    trap::init();
    loader::load_apps();
    tasks::run_first_task();
    panic!("Unreachable in rust_main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
