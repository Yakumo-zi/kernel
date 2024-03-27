mod fs;
mod process;
use fs::*;
use process::*;

use crate::tasks::{get_current_task, record_task_info, TaskInfo};

pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_TASKINFO: usize = 410;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GET_TIME: usize = 169;
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    record_task_info(syscall_id);
    let current_task = get_current_task();
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_TASKINFO => sys_task_info(current_task, args[0] as *mut TaskInfo),
        SYSCALL_YIELD => sys_yiled(),
        SYSCALL_GET_TIME => sys_get_time(),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

#[derive(Clone, Copy,Debug)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize,
}
