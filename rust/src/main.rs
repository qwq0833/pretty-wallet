mod core;
mod producer;

use crate::producer::Producer;
use std::thread;

fn main() {
    let destination = "../out/wallets.jsonl";
    let pretty_prefix = "0x000";
    let duration = 60 * 1000;
    let workers = num_cpus::get();

    println!("🚀 Pretty Wallet 正在启动, 即将开启 {} 个工作线程", workers);

    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..workers {
        let producer = Producer::new(duration, pretty_prefix.to_string(), destination.to_string());
        let handle = thread::spawn(move || {
            producer.start();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("🚀 Pretty Wallet 运行结束");
}
