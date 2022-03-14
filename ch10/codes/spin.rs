use std::{thread, time};

fn main() {
    for n in 1..=1000 {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();
        for _ in 0..n {
            let handle = thread::spawn(|| {
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);
                while start.elapsed() < pause {
                    thread::yield_now();
                }
            });
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop() {
            let _ = handle.join();
        }

        let finish = time::Instant::now();
        println!("{n}\t{:02?}", finish.duration_since(start));
    }
}
