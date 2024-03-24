use crate::tasks::{mark_current_exited_and_run_next, mark_current_suspended_and_run_next};

pub fn sys_exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    mark_current_exited_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_get_taskinfo() -> isize {
    0
}

pub fn sys_yiled() -> isize {
    mark_current_suspended_and_run_next();
    0
}
