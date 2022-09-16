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
}

pub fn wait(a: &AtomicU32, expected: u32) -> u32 {
    let ptr: *const AtomicU32 = a;
    loop {
        let current = a.load(Relaxed);
        if current != expected {
            return current;
        }
        let monitor = unsafe { __libcpp_atomic_monitor(ptr.cast()) };
        let current = a.load(Relaxed);
        if current != expected {
            return current;
        }
        unsafe { __libcpp_atomic_wait(ptr.cast(), monitor) };
    }
}

pub fn wake_one(a: &AtomicU32) {
    let ptr: *const AtomicU32 = a;
    unsafe { __cxx_atomic_notify_one(ptr.cast()) };
}

pub fn wake_all(a: &AtomicU32) {
    let ptr: *const AtomicU32 = a;
    unsafe { __cxx_atomic_notify_all(ptr.cast()) };
}