use crate::core::Wallet;
use std::time::Instant;

pub struct Producer {
    duration: u128,
    pretty_prefix: String,
    destination: String,
}

impl Producer {
    pub fn new(duration: u128, pretty_prefix: String, destination: String) -> Producer {
        Producer {
            duration,
            pretty_prefix,
            destination,
        }
    }

    pub fn start(&self) {
        let now = Instant::now();

        let mut total_count = 0;
        let mut pretty_count = 0;

        // Continuous generate wallets
        while now.elapsed().as_millis() < self.duration {
            let wallet = Wallet::random().unwrap();
            // Check if the wallet is pretty
            if wallet.address.starts_with(&self.pretty_prefix) {
                wallet.append_to_file(&self.destination).unwrap();
                pretty_count += 1;
            }
            total_count += 1;

            // Print stats every 5000 wallets
            if total_count % 5000 == 0 {
                println!(
                    "已生成 {} 个钱包, 符合条件的有 {} 个, 执行时长 {} ms, 速度 {} 钱包/s",
                    total_count,
                    pretty_count,
                    now.elapsed().as_millis(),
                    total_count / now.elapsed().as_secs()
                );
            }
        }

        println!("============================================================");
        println!("[执行结束]");
        println!(
            "统计: 总共生成 {} 个钱包, 符合条件的有 {} 个",
            total_count, pretty_count
        );
        println!("执行时长: {} ms", now.elapsed().as_millis());
        println!("速度: {} 钱包/s", total_count / now.elapsed().as_secs());
        println!("============================================================");
    }
}
