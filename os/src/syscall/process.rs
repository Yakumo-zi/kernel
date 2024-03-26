use crate::{
    tasks::{
        get_task_info, mark_current_exited_and_run_next, mark_current_suspended_and_run_next,
        TaskInfo,
    },
    timer::get_time_ms,
};

pub fn sys_exit(xstate: i32) -> ! {
    println!(
        "[kernel] Application exited with code {},Completion time {}",
        xstate,
        sys_get_time()
    );
    mark_current_exited_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yiled() -> isize {
    mark_current_suspended_and_run_next();
    0
}
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
pub fn sys_task_info(id: usize, ts: *mut TaskInfo) -> isize {
    let info = get_task_info(id);
    unsafe {
        (*ts).id = info.id;
        (*ts).status = info.status;
        (*ts).time = info.time;
        // (*ts).call = info.call.clone();
    }
    0
}
