use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};

fn main() {

    let some_ptr  = Arc::new(AtomicIsize::new(0));

    let cl1 = some_ptr.clone();
    let cl2 = some_ptr.clone();

    let handle = thread::spawn(move || {
        for _i in 1..20 {
            let tmp1_ptr = cl1.load(Ordering::Relaxed);
            let new = tmp1_ptr + 1;
            let old = cl1.load(Ordering::Relaxed);
            let _value = cl1.compare_and_swap(old, new, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for _i in 1..20 {
        let tmp1_ptr = cl2.load(Ordering::Relaxed);
        let new = tmp1_ptr + 1;
        let old = cl2.load(Ordering::Relaxed);
        let _value = cl2.compare_and_swap(old, new, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    println!("{:?}", some_ptr);
}