use sam_blockchain::{BlockChain, Transaction};

fn main() {
    let mut bchain = BlockChain::new(3);

    let trs = vec![
        Transaction::new("Alice", "Bob", 10.5),
        Transaction::new("Bob", "Sam", 3.3),
    ];
    let start = std::time::Instant::now();
    bchain.add_block(trs);
    let elapsed = start.elapsed();

    println!("{}", bchain);
    println!("\nIs Valid BlockChain:  {}", bchain.is_valid());
    println!("\nAdd new block done in {} msec", elapsed.as_millis());
}

