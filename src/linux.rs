use super::AtomicWaitInternal;

struct FutexWaitV {
    val: u64,
    uaddr: u64,
    flags: i32,
    _reserved: u32,
}

#[inline]
pub(crate) fn wait<T: AtomicWaitInternal>(a: &T, expected: T::Value) -> T::Value
where
    T::Value: Copy + Into<u64>,
{
    let size = core::mem::size_of::<T>();
    let ptr: *const _ = a;
    loop {
        if let Err(current) = a.check_value(expected) {
            return current;
        }
        if size == 4 {
            unsafe {
                libc::syscall(
                    libc::SYS_futex,
                    ptr,
                    libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG,
                    expected,
                    core::ptr::null::<libc::timespec>(),
                );
            };
        } else {
            let waiter = FutexWaitV {
                val: expected.into(),
                uaddr: ptr as u64,
                flags: libc::FUTEX_PRIVATE_FLAG
                    | match size {
                        1 => libc::FUTEX_8,
                        2 => libc::FUTEX_16,
                        8 => libc::FUTEX_64,
                        _ => unreachable!(),
                    },
                _reserved: 0,
            };
            unsafe {
                libc::syscall(
                    libc::SYS_futex_waitv,
                    &waiter,
                    1, // 1 futex to wait for
                    0, // no flags
                    core::ptr::null::<libc::timespec>(),
                    libc::CLOCK_MONOTONIC,
                );
            }
        }
    }
}

#[inline]
pub(crate) fn wake_one(ptr: *const ()) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
            1i32,
        );
    };
}

#[inline]
pub(crate) fn wake_all(ptr: *const ()) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
            i32::MAX,
        );
    };
}
