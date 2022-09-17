mod core;

fn main() {
    let wallet = core::Wallet::random().unwrap();
    dbg!(wallet);
}
