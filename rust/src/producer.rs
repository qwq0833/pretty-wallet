use crate::core::Wallet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

static INDEX: AtomicUsize = AtomicUsize::new(1);

pub struct Producer {
    key: String,
    duration: u128,
    prefix: String,
    destination: String,
}

impl Producer {
    pub fn new(duration: u128, prefix: &str, destination: &str) -> Producer {
        let index = INDEX.fetch_add(1, Ordering::SeqCst);
        Producer {
            key: index.to_string(),
            duration: duration * 1000,
            prefix: prefix.to_string(),
            destination: destination.to_string(),
        }
    }

    pub fn start(&self) {
        let now = Instant::now();
        println!("🌱 线程 {} 开始运行", self.key);

        let mut total_count = 0;
        let mut pretty_count = 0;

        // Continuous generate wallets
        while self.duration == 0 || now.elapsed().as_millis() < self.duration {
            let wallet = Wallet::random().unwrap();
            // Check if the wallet is pretty
            if wallet.address.starts_with(&self.prefix) {
                wallet.append_to_file(&self.destination).unwrap();
                pretty_count += 1;
            }
            total_count += 1;

            // Print stats every 5000 wallets
            if total_count % 5000 == 0 {
                println!(
                    "[线程 {}] 已生成 {} 个钱包, 符合条件的有 {} 个, 执行时长 {} ms, 速度 {} 钱包/s",
                    self.key,
                    total_count,
                    pretty_count,
                    now.elapsed().as_millis(),
                    total_count / now.elapsed().as_secs()
                );
            }
        }

        println!("============================================================");
        println!("[线程 {} 执行结束]", self.key);
        println!(
            "统计: 总共生成 {} 个钱包, 符合条件的有 {} 个",
            total_count, pretty_count
        );
        println!("执行时长: {} ms", now.elapsed().as_millis());
        println!("速度: {} 钱包/s", total_count / now.elapsed().as_secs());
        println!("============================================================");
    }
}
