use std::mem::MaybeUninit;
use winapi::um::processthreadsapi::{GetCurrentProcess, GetProcessTimes};

pub fn get_cpu_time() -> Opition<u64> {
    let (kernel, user) = unsafe {
        let process = GetCurrentProcess();
        let mut _creation = MaybeUninit::uninit();
        let mut _exit = MaybeUninit::uninit();
        let mut kernel = MaybeUninit::uninit();
        let mut user = MaybeUninit::uninit();
        // 代码调用 GetProcessTimes 函数，传入进程句柄和四个变量的可变指针。如果 GetProcessTimes 函数返回 0，表示函数调用失败，此时代码返回 None。
        if GetProcessTimes(
            process,
            _creation.as_mut_ptr(),
            _exit.as_mut_ptr(),
            kernel.as_mut_ptr(),
            user.as_mut_ptr(),
        ) == 0
        {
            return None;
        }
        (kernel.assume_init(), user.assume_init())
    };
    // kernel 和 user 中的 dwHighDateTime 和 dwLowDateTime 字段组合成一个 u64 类型的值。
    // 这是通过将 dwHighDateTime 左移 32 位，然后与 dwLowDateTime 进行位或操作实现的。
    let kernel = (kernel.dwHighDateTime as u64) << 32 | kernel.dwLowDateTime as u64;
    let user = (user.dwHighDateTime as u64) << 32 | user.dwLowDateTime as u64;
    // 代码返回 Some(100 * (kernel + user))，这表示内核模式和用户模式下的 CPU 时间的总和，单位是百纳秒。
    Some(100 * (kernel + user))
}
