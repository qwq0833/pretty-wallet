mod core;
mod producer;

use crate::producer::Producer;
use clap::Parser;
use std::thread;
use std::time::Duration;

/// 🍒 Pretty ethereum wallet generator
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Number of workers (default: number of logical cores)
    #[clap(short, long, value_parser, default_value_t = num_cpus::get())]
    workers: usize,

    /// Prefix of the wallet address
    #[clap(short, long, value_parser, default_value = "0x000000")]
    prefix: String,

    /// Number of execution duration in seconds (0 for infinite)
    #[clap(short, long, value_parser, default_value_t = 60)]
    duration: u128,

    /// Path of wallet keystore file
    #[clap(
        short,
        long,
        value_parser,
        default_value = "/tmp/pretty_keystore.jsonl"
    )]
    output: String,
}

fn main() {
    let args = Args::parse();

    let destination = &args.output;
    let prefix = &args.prefix;
    let duration = args.duration;
    let workers = args.workers;

    println!(
        "🚀 Pretty Wallet 即将开启 {} 个工作线程, {}",
        workers,
        if duration == 0 {
            "持续运行直到按下 Ctrl+C".to_string()
        } else {
            format!("运行 {} s 后停止", duration)
        }
    );
    println!("🚀 匹配前缀 {}, 结果将输出到 {}", prefix, destination);

    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..workers {
        let producer = Producer::new(duration, prefix, destination);
        let handle = thread::spawn(move || {
            producer.start();
        });
        handles.push(handle);
        thread::sleep(Duration::from_millis(1));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("🚀 Pretty Wallet 运行结束");
}
