mod elevator;
mod philosophers;

use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let queue = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));

    let queue_consumer = Arc::clone(&queue);
    thread::spawn(move || {
        let (lock, condvar) = &*queue_consumer;

        loop {
            let mut q = lock.lock().unwrap();

            // Esperar mientras la cola esté vacía
            while q.is_empty() {
                q = condvar.wait(q).unwrap();
            }

            // Hay elementos, consumir uno
            if let Some(item) = q.pop_front() {
                println!("Popped: {item}");
            }
        }
    });

    // Productor principal
    let (lock, condvar) = &*queue;
    for i in 0.. {
        let mut q = lock.lock().unwrap();
        q.push_back(i);
        println!("Pushed: {i}");

        condvar.notify_one(); // Avisar al consumidor
        drop(q); // No obligatorio, pero explícito

        thread::sleep(Duration::from_secs(1));
    }
}


// producer consumer