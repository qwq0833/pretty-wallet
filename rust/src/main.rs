mod core;
mod producer;

use crate::producer::Producer;

fn main() {
    let destination = "../out/wallets.jsonl";
    let pretty_prefix = "0x000";
    let duration = 60 * 1000;

    Producer::new(duration, pretty_prefix.to_string(), destination.to_string()).start();
}
