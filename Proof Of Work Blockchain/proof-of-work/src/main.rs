extern crate crypto;
extern crate num_bigint;
extern crate num_traits;
extern crate chrono;

use std::process;

mod mining_error;
mod block;
mod blockchain;

use mining_error::MiningError;
use blockchain::Blockchain;

fn main() {
    println!();
    println!("Welcome to proof-of-work blockchain!");

    run().
        unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1)
        })
}

fn run() -> Result<(), MiningError> {
    println!();
    println!("Mining genesis block with difficulty {}", block::DIFFICULTY);
    let mut chain = Blockchain::new()?;

    println!();
    println!("First block being mined!");
    let nonce_1 = chain.add_block("this is block one hello huhu")?;
    println!("First block successfully mined after {} tries", nonce_1);

    println!();
    println!("Second block being mined!");
    let nonce_2 = chain.add_block("this is block two gee")?;
    println!("Second block successfully mined after {} tries", nonce_2);

    println!();
    println!("Traversing blockchain:");
    println!();
    chain.traverse();

    Ok(())
}
