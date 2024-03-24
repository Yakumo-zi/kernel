use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception},
    stval, stvec,
};

use crate::syscall::syscall;

use self::context::TrapContext;
use crate::tasks;
pub mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        scause::Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        scause::Trap::Exception(Exception::StoreFault)
        | scause::Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it."); 
            panic!("[kernel] Cannot continue!");
        }
        scause::Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            panic!("[kernel] Cannot continue!");
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
