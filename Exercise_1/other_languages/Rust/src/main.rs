use std::thread;
use std::sync::{Arc, Mutex};

fn main() {

    let i: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let i_incrementing = i.clone();
    let i_decrementing = i.clone();

    let join_incrementing = thread::spawn(move || {
        for _ in 0..1_000_000 {
            let mut i_incrementing = i_incrementing.lock().unwrap();
            *i_incrementing += 1;
        }
    });
    
    let join_decrementing = thread::spawn(move || {
        for _ in 0..999_900 {
            let mut i_decrementing = i_decrementing.lock().unwrap();
            *i_decrementing -= 1;
        }

    });

    join_incrementing.join().unwrap();
    join_decrementing.join().unwrap();

    let mut i = i.lock().unwrap();
    println!("The number is: {}", *i);
}

