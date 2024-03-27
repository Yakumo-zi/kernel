use core::arch::asm;
use core::fmt::Display;
pub const MAX_SYSCALL_NUM: usize = 256;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASKINFO: usize = 410;
const SYSCALL_SLEEP: usize = 13;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm! {
            "ecall",
            inlateout("x10") args[0]=>ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        }
    }
    ret
}
pub fn sys_sleep(sec:usize)->isize{
    syscall(SYSCALL_SLEEP, [sec,0,0])
}
pub fn sys_task_info(ts: *mut TaskInfo) -> isize {
    syscall(SYSCALL_TASKINFO, [ts as usize, 0, 0])
}
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    let ret = syscall(SYSCALL_EXIT, [xstate as usize, 0, 0]);
    println!("[user] Task completion time {}", sys_get_time());
    ret
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

#[derive(Clone, Copy, Debug)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
}
impl Display for TaskInfo {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        println!(
            "ID:{} Status:{:?},RunningTime:{}",
            self.id, self.status, self.time
        );
        println!("[");
        for i in 0..MAX_SYSCALL_NUM {
            if self.call[i].id != 0 {
                println!("\t{:?}", self.call[i]);
            }else{
                break;
            }
        }
        println!("]");
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
