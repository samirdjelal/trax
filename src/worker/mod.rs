use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

pub async fn run() {
    let pool = ThreadPool::new(4);
    let (sender, receiver) = std::sync::mpsc::channel();

    for _ in 0..4 {
        let sender = sender.clone();
        pool.execute(move || {
            thread::sleep(Duration::from_secs(1));
            sender.send("Hello from thread pool").unwrap();
        });
    }

    loop {
        let msg = receiver.recv().unwrap();
        println!("{}", msg);
    }
	
	pool.join();
}
