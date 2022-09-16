use atomic_wait::AtomicWait;
use std::{
    sync::atomic::{AtomicU32, Ordering::Relaxed},
    thread::sleep,
    time::{Duration, Instant},
};

#[test]
fn wake_nothing() {
    let a = AtomicU32::new(0);
    a.wake_one();
    a.wake_all();
}

#[test]
fn wait_unexpected() {
    let t = Instant::now();
    let a = AtomicU32::new(0);
    a.wait(1);
    assert!(t.elapsed().as_millis() < 100);
}

#[test]
fn wait_wake() {
    let t = Instant::now();
    let a = AtomicU32::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            sleep(Duration::from_millis(100));
            a.store(1, Relaxed);
            a.wake_one();
        });
        while a.load(Relaxed) == 0 {
            a.wait(0);
        }
        assert_eq!(a.load(Relaxed), 1);
        assert!((90..400).contains(&t.elapsed().as_millis()));
    });
}
