mod core;

use std::fs::{create_dir_all, File};
use std::path::Path;

fn main() {
    let destination = "../out/wallets.jsonl";

    // Create wallet storage file if not exists
    let path = Path::new(destination);
    if !path.exists() {
        create_dir_all(path.parent().unwrap()).unwrap();
        File::create(path).unwrap();
    }

    // Generate wallets
    let wallet = core::Wallet::random().unwrap();
    wallet.append_to_file(destination).unwrap();
    dbg!(wallet);
}
