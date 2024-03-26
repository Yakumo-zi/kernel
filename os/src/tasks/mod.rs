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
                    last_switch_time: get_time_ms(),
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
    last_switch_time: usize,
}

pub fn mark_current_suspended_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn mark_current_exited_and_run_next() {
    mark_current_exited();
    run_next_task();
}
pub fn run_first_task() {
    TASK_MANAGER.run_first_task()
}
pub fn record_task_info(syscall_id: usize) {
    TASK_MANAGER.record_task_info(syscall_id)
}
pub fn get_current_task_info() -> TaskInfo {
    get_task_info(TASK_MANAGER.inner.exclusive_access().current_task)
}
pub fn get_task_info(id: usize) -> TaskInfo {
    TASK_MANAGER.get_task_info(id)
}

pub fn get_current_task() -> usize {
    TASK_MANAGER.get_current_task()
}
impl TaskManager {
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
    }
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_ptr = &task0.task_cx as *const TaskContext;
        inner.last_switch_time = get_time_ms();
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_ptr);
        }
        panic!("unreachable in run_first_task!");
    }
    fn record_task_info(&self, syscall_id: usize) {
        let inner = self.inner.exclusive_access();
        let mut current_task = inner.tasks[inner.current_task];
        let task_call_size = current_task.task_call_size;
        let task_info = &mut current_task.task_info;
        for i in 0..=current_task.task_call_size {
            if i == task_info.call[i].id {
                task_info.call[i].times = task_info.call[i].times + 1;
                task_info.status = current_task.task_status;
                task_info.time = task_info.time + get_time_ms() - inner.last_switch_time;
                return;
            }
        }
        task_info.time = task_info.time + get_time_ms() - inner.last_switch_time;
        task_info.call[0].id = syscall_id;
        task_info.id=inner.current_task;
        task_info.status = current_task.task_status;
        current_task.task_call_size = task_call_size + 1;
    }
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr: *const TaskContext =
                &mut inner.tasks[next].task_cx as *const TaskContext;
            inner.last_switch_time = get_time_ms();
            drop(inner);
            // let start = get_time_ms();
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // let end = get_time_ms();

            // println!(
            //     "[kernel] From app {} switch to app {} spend time {}",
            //     current,
            //     next,
            //     end - start
            // );
        } else {
            panic!("All applications completed!");
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }
}
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}
