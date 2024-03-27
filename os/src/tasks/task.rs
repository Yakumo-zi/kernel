use crate::{config::MAX_SYSCALL_NUM, syscall::SyscallInfo};

use super::TaskContext;

#[derive(Clone, Copy, PartialEq,Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Clone, Copy,Debug)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub task_info: TaskInfo,
    pub task_call_size:usize,
    pub kernel_time:usize,
    pub user_time:usize,
}

#[derive(Clone, Copy,Debug)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
}

impl TaskInfo {
    pub fn zero_init() -> Self {
        Self {
            id: 0,
            status: TaskStatus::UnInit,
            call: [SyscallInfo { id: 0, times: 0 }; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}
