#[repr(C)]
#[derive(Clone, Copy,Debug)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> TaskContext {
        TaskContext {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_restore(kstack_ptr: usize) -> TaskContext {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,  // 通过 __switch 中的 ret 指令跳转到另一个Task中
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
