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
        for _i in 0..20 {
            let mut old = clone.load(Ordering::SeqCst);
            let mut new = old + 1;
            while clone.compare_and_swap(old, new, Ordering::SeqCst) != old {
                old = clone.load(Ordering::SeqCst);
                new = old + 1;
                thread::sleep(Duration::from_millis(1));
            };
        }
    })
}