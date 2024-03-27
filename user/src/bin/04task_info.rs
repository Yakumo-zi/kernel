#![no_std]
#![no_main]

use user_lib::{
    get_taskinfo, get_time, sleep, yield_, SyscallInfo, TaskInfo, TaskStatus, MAX_SYSCALL_NUM,
};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    sleep(1);
    let ts = &mut TaskInfo {
        id: 0,
        status: TaskStatus::UnInit,
        time: 0,
        call: [SyscallInfo { id: 0, times: 0 }; MAX_SYSCALL_NUM],
    };
    let ret = get_taskinfo(ts as *mut TaskInfo);
    println!("TaskInfo {:}", ts);
    ret as i32
}
