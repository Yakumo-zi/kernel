use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Interrupt}, sie, stval, stvec
};

use crate::{
    syscall::syscall, tasks::mark_current_suspended_and_run_next, timer::set_next_trigger,
};

use self::context::TrapContext;
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

pub fn enable_timer_interrupt(){
     unsafe { sie::set_stimer(); }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        scause::Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; // 定位到下一条指令
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
        scause::Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            mark_current_suspended_and_run_next()
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
