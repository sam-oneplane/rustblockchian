
use sam_blockchain::{blockchain::BlockChain, transaction::Transaction, wallet::{Wallet, verify}};

fn main() {
    let bob = Wallet::new();
    let alice = Wallet::new(); 
    let dave = Wallet::new();

    let mut bchain = BlockChain::new(3);

    let mut tr = Transaction::new(&alice.public_key, &bob.public_key, 20.5);
    alice.sign(& mut tr);
    bchain.submit_transaction(tr);


    tr = Transaction::new(&bob.public_key, &dave.public_key, 13.3);
    bob.sign(& mut tr);
    bchain.submit_transaction(tr);

    tr = Transaction::new(&dave.public_key, &alice.public_key, 10.5);
    dave.sign(& mut tr);
    print!("Verify Dave Transaction: {}", verify(&tr));
    bchain.submit_transaction(tr);
    
    println!("Blockchian size: {}\n", bchain.mempool_size());
    

    let start = std::time::Instant::now();
    bchain.mine_pending_trans(&alice.public_key);
    let elapsed = start.elapsed();

    println!("Blockchian size: {}", bchain.mempool_size());
    print!("Balance Alice's Walllet : {}", alice.balance(&bchain));

    println!("{}", bchain);
    println!("\nIs Valid BlockChain:  {}", bchain.is_valid());
    println!("\nAdd new block done in {} msec", elapsed.as_millis());
}

