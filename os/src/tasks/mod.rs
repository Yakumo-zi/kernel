mod context;
mod switch;
mod task;

pub use context::*;
pub use task::*;

use lazy_static::lazy_static;

use crate::{
    config::MAX_APP_NUM,
    loader::{get_num_app, init_app_cx},
    sync::up::UPSafeCell,
    timer::get_time_ms,
};

use self::switch::__switch;

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            task_info: TaskInfo::zero_init(),
            task_call_size: 0,
            kernel_time: 0,
            user_time: 0,
            wake_up_time: 0,
        }; MAX_APP_NUM];

        for i in 0..num_app {
            tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
            tasks[i].task_status = TaskStatus::Ready;
            tasks[i].task_info.id = i;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                    stop_watch: 0,
                })
            },
        }
    };
}
pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
    stop_watch: usize,
}

pub fn mark_current_suspended_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn mark_current_exited_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn mark_current_sleep_and_run_next(wake_up_time: usize) {
    mark_current_sleep(wake_up_time);
    run_next_task();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task()
}
pub fn record_task_info(syscall_id: usize) {
    TASK_MANAGER.record_task_info(syscall_id)
}

#[allow(unused)]
pub fn get_current_task_info() -> TaskInfo {
    get_task_info(TASK_MANAGER.inner.exclusive_access().current_task)
}
pub fn get_task_info(id: usize) -> TaskInfo {
    TASK_MANAGER.get_task_info(id)
}

pub fn get_current_task() -> usize {
    TASK_MANAGER.get_current_task()
}
pub fn user_time_start() {
    TASK_MANAGER.user_time_start()
}
pub fn user_time_end() {
    TASK_MANAGER.user_time_end()
}
impl TaskManagerInner {
    fn refresh_stop_watch(&mut self) -> usize {
        let start = self.stop_watch;
        self.stop_watch = get_time_ms();
        self.stop_watch - start
    }
}
impl TaskManager {
    fn user_time_start(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
    }
    fn user_time_end(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].user_time += inner.refresh_stop_watch();
    }
    fn get_current_task(&self) -> usize {
        self.inner.exclusive_access().current_task
    }
    fn get_task_info(&self, id: usize) -> TaskInfo {
        self.inner.exclusive_access().tasks[id].task_info.clone()
    }
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
    }
    fn mark_current_sleep(&self, wake_up_time: usize) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
        inner.tasks[current].wake_up_time = wake_up_time;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
    }
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_ptr = &task0.task_cx as *const TaskContext;
        inner.refresh_stop_watch();
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_ptr);
        }
        panic!("unreachable in run_first_task!");
    }
    fn record_task_info(&self, syscall_id: usize) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let current_task = &mut inner.tasks[current];
        let task_call_size = current_task.task_call_size;
        let task_info = &mut current_task.task_info;

        for i in 0..=task_call_size {
            if task_info.call[i].id == 0 {
                task_info.call[i].times = task_info.call[i].times + 1;
                task_info.call[i].id = syscall_id;
                task_info.status = current_task.task_status;
                task_info.time = current_task.user_time;
                current_task.task_call_size = task_call_size + 1;
                break;
            } else if syscall_id == task_info.call[i].id {
                task_info.call[i].times = task_info.call[i].times + 1;
                task_info.status = current_task.task_status;
                task_info.time = current_task.user_time;
                break;
            }
        }
    }
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.tasks[next].wake_up_time = 0;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr: *const TaskContext =
                &mut inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            panic!("All applications completed!");
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let select_one = |i: usize| {
            (current + i..current + self.num_app + i)
                .map(|id| id % self.num_app)
                .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
        };

        let mut i = 1;
        while let Some(task) = select_one(i) {
            if get_time_ms() >= inner.tasks[task].wake_up_time {
                return Some(task);
            } else {
                i += 1;
                continue;
            }
        }
        None
    }
}
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn mark_current_sleep(wake_up_time: usize) {
    TASK_MANAGER.mark_current_sleep(wake_up_time)
}
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}
