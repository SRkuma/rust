use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};

fn main() {

    let some_ptr = Arc::new(AtomicIsize::new(0));

    let cl1 = some_ptr.clone();
    let cl2 = some_ptr.clone();

    let handle1 = other_thread(cl1);
    let handle2 = other_thread(cl2);
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{:?}", some_ptr);
}

fn other_thread(clone: Arc<AtomicIsize>) -> JoinHandle<()> {
    return thread::spawn(move || {
        for _i in 1..20 {
            let tmp1_ptr = clone.load(Ordering::Relaxed);
            let new = tmp1_ptr + 1;
            let old = clone.load(Ordering::Relaxed);
            let _value = clone.compare_and_swap(old, new, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(1));
        }
    })
}