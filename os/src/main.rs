#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod config;
mod counter;
mod lang_items;
mod loader;
mod logger;
mod mm;
mod sbi;
mod sync;
mod syscall;
mod tasks;
mod timer;
mod trap;

use crate::logger::Logger;
use core::{arch::global_asm, usize};
extern crate alloc;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

static MY_LOGGER: Logger = Logger;
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
    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

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
    mm::init_heap();
    // mm::heap_test();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
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
