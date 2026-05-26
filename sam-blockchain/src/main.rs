
use sam_blockchain::{BlockChain, transaction::Transaction};


fn main() {
    let mut bchain = BlockChain::new(3);
    bchain.submit_transaction(Transaction::new("Alice", "Bob", 10.5));
    bchain.submit_transaction(Transaction::new("Bob", "Sam", 3.3));
    bchain.submit_transaction(Transaction::new("Dave", "Bob", 1.5));
    
    println!("Blockchian size: {}", bchain.mempool_size());
    let start = std::time::Instant::now();
    bchain.mine_pending_trans();
    let elapsed = start.elapsed();
    println!("Blockchian size: {}", bchain.mempool_size());

    println!("{}", bchain);
    println!("\nIs Valid BlockChain:  {}", bchain.is_valid());
    println!("\nAdd new block done in {} msec", elapsed.as_millis());
}

