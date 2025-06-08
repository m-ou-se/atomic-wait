use core::sync::atomic::AtomicU32;

#[inline]
pub fn wait(a: &AtomicU32, expected: u32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            a,
            libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG,
            expected,
            core::ptr::null::<libc::timespec>(),
        );
    };
}

#[inline]
pub fn wake_one(ptr: *const AtomicU32) {
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
pub fn wake_n(ptr: *const AtomicU32, wake_count: u32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
            wake_count as i32,
        );
    };
}

#[inline]
pub fn wake_all(ptr: *const AtomicU32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
            i32::MAX,
        );
    };
}
