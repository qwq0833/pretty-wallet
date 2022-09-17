mod core;

use std::fs::{create_dir_all, File};
use std::path::Path;

fn main() {
    let destination = "../out/wallets.jsonl";
    let pretty_prefix = "0x000";

    // Create wallet storage file if not exists
    let path = Path::new(destination);
    if !path.exists() {
        create_dir_all(path.parent().unwrap()).unwrap();
        File::create(path).unwrap();
    }

    let now = std::time::Instant::now();

    // Generate wallets
    let wallet = core::Wallet::random().unwrap();
    // Check if the wallet is pretty
    if wallet.address.starts_with(pretty_prefix) {
        wallet.append_to_file(destination).unwrap();
    }

    println!("执行时长: {} ms", now.elapsed().as_millis());
}
