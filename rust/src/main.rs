mod core;

fn main() {
    let destination = "../out/wallets.jsonl";
    let pretty_prefix = "0x000";
    let duration = 60 * 1000;

    let now = std::time::Instant::now();
    let mut total_count = 0;
    let mut pretty_count = 0;

    // Generate wallets
    while now.elapsed().as_millis() < duration {
        let wallet = core::Wallet::random().unwrap();
        // Check if the wallet is pretty
        if wallet.address.starts_with(pretty_prefix) {
            wallet.append_to_file(destination).unwrap();
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

    println!("[执行结束]");
    println!(
        "统计: 总共生成 {} 个钱包, 符合条件的有 {} 个",
        total_count, pretty_count
    );
    println!("执行时长: {} ms", now.elapsed().as_millis());
    println!("速度: {} 钱包/s", total_count / now.elapsed().as_secs());
}
