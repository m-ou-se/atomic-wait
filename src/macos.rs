use core::{
    ffi::c_void,
    sync::atomic::{AtomicU32, Ordering::Relaxed},
};

// On macOS, atomic wait/wake functionality is not available through
// any public/stable C interface, but is available through libc++.
//
// The libc++ functions declared below are are not publicly documented,
// but they are part of the stable ABI.
//
// These functions are used to implement C++20's std::atomic<T>::{wait,
// notify*} which are defined in libc++'s headers, resulting in C++ binaries
// that dynamically link these symbols. So, it's safe to rely on these from
// Rust as well, as long as we also link libc++.
//
// These exist since macOS 11, iOS 14, and watchOS 7.

#[link(name = "c++")]
extern "C" {
    // std::__1::__libcpp_atomic_monitor(void const volatile*)
    #[link_name = "_ZNSt3__123__libcpp_atomic_monitorEPVKv"]
    fn __libcpp_atomic_monitor(ptr: *const c_void) -> i64;

    // std::__1::__libcpp_atomic_wait(void const volatile*, long long)
    #[link_name = "_ZNSt3__120__libcpp_atomic_waitEPVKvx"]
    fn __libcpp_atomic_wait(ptr: *const c_void, monitor: i64);

    // std::__1::__cxx_atomic_notify_one(void const volatile*)
    #[link_name = "_ZNSt3__123__cxx_atomic_notify_oneEPVKv"]
    fn __cxx_atomic_notify_one(ptr: *const c_void);

    // std::__1::__cxx_atomic_notify_all(void const volatile*)
    #[link_name = "_ZNSt3__123__cxx_atomic_notify_allEPVKv"]
    fn __cxx_atomic_notify_all(ptr: *const c_void);

    // Next to the four `void const volatile*` functions above, there are also
    // overloads for `__cxx_atomic_contention_t const volatile*` (where
    // `__cxx_atomic_contention_t` is basically `AtomicI64`), specifically for
    // 64-bit atomics. Those don't use a separate futex in a table, but instead
    // use the atomic itself as the futex, which can be more efficient.
    // However, because of the monitor+wait API design here, that can result in
    // missed wakeups, due to the ABA problem. So, we simply don't use those.
    // See https://reviews.llvm.org/D114119#3193088
}

#[inline]
pub fn wait(a: &AtomicU32, expected: u32) {
    let ptr: *const AtomicU32 = a;
    // The 'monitor' is just the notification counter associated
    // with the address of the atomic.
    let monitor = unsafe { __libcpp_atomic_monitor(ptr.cast()) };
    // Check again if we should still go to sleep.
    if a.load(Relaxed) != expected {
        return;
    }
    // Wait, but only if there's been no new notifications
    // since we acquired the monitor.
    unsafe { __libcpp_atomic_wait(ptr.cast(), monitor) };
}

#[inline]
pub fn wake_one(ptr: *const AtomicU32) {
    unsafe { __cxx_atomic_notify_one(ptr.cast()) };
}

#[inline]
pub fn wake_n(ptr: *const AtomicU32, wake_count: u32) {
    for _ in 0..wake_count {
        wake_one(ptr);
    }
}

#[inline]
pub fn wake_all(ptr: *const AtomicU32) {
    unsafe { __cxx_atomic_notify_all(ptr.cast()) };
}
