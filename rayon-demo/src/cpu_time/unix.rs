use libc::{getrusage, RUSAGE_SELF};
// MaybeUninit<T> 是一个 Rust 中的包装类型，用于表示可能未被初始化的数据。
// 它是一种安全的方式来处理未初始化的内存，因为 Rust 的默认行为是要求所有的变量在使用前都必须被初始化。
// 在某些情况下，你可能需要在变量完全初始化之前就创建它，
// 例如，当你需要先分配内存，然后在稍后的时间点填充这块内存时。在这种情况下，你可以使用 MaybeUninit<T>。
// 这个类型的主要优点是它不会自动调用 T 的析构函数，这意味着你可以安全地覆盖 MaybeUninit<T> 包含的数据，而不需要担心未初始化的数据被错误地释放。
use std::mem::MaybeUninit;

pub fn get_cpu_time() -> Option<u64> {
    let usage = unsafe {
        let mut usage = MaybeUninit::uninit();
        // getrusage 函数，将 RUSAGE_SELF 和 usage 的可变指针作为参数。
        // RUSAGE_SELF 是一个常量，表示我们想要获取当前进程的资源使用信息。u
        // sage.as_mut_ptr() 是获取 usage 的可变指针，这样 getrusage 函数就可以将结果写入这块内存。
        if getrusage(RUSAGE_SELF, usage.as_mut_ptr()) != 0 {
            return None;
        }
        // usage.assume_init() 来获取 usage 中的值。
        // 这个方法会移除 MaybeUninit 的包装，并返回初始化后的值。这个值被赋值给 useage 变量。
        usage.assume_init()
    };

    let user =
        1_000_000_000 * (usage.ru_utime.tv_sec as u64) + 1_000 * (usage.ru_utime.tv_usec as u64);
    let system =
        1_000_000_000 * (usage.ru_stime.tv_sec as u64) + 1_000 * (usage.ru_stime.tv_usec as u64);

    Some(user + system)
}
