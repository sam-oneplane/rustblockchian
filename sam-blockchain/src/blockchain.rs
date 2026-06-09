
use serde::{Deserialize, Serialize};

use crate::block::Block;
use crate::transaction::Transaction;
use crate::wallet;
use std::{fmt::{self}};

const MINOR_SENDER : &str = "SYSTEM";

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    mempool: Vec<Transaction>,
    pub difficulty: usize,
}

impl BlockChain {
    pub fn new(difficulty : usize) -> Self {

        let genesis = Block::new(0, Vec::new(), "0".repeat(64).as_str());
        Self {
            chain: vec![genesis],
            mempool: vec![],
            difficulty,
        }
    }

    fn add_block(&mut self, minor: &str) {

        let last_block = self.last_block();
        let idx: u64 = last_block.index + 1;
        let prev_hash = &last_block.hash.clone();
        let mut transactions = std::mem::take(&mut self.mempool);
        transactions.insert(
            0, 
            Transaction::new(MINOR_SENDER, minor, 50.0)
        );

        let mut new_block = Block::new(idx, transactions, prev_hash);
        
        new_block.mine(self.difficulty);

        self.chain.push(new_block);
    }

    pub fn last_block(&mut self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn is_valid(&self) -> bool {
    
        for window in self.chain.windows(2) {
            let prev = &window[0];
            let curr = &window[1];
            if curr.hash != curr.generate_hash() {
                return false;
            }
            if curr.prev_hash != prev.hash {
                return false;
            }

            for tr in &curr.transactions {
                if !wallet::verify(tr) {
                    println!("Failed tx: {} -> {} : {}", tr.sender, tr.recipient, tr.amount);
                    println!("  public_key: '{}'", tr.public_key);
                    println!("  signature:  '{}'", tr.signature);
                    return false;
                }
            }
        }
        
        true
    }
}

impl BlockChain {
    pub fn submit_transaction(&mut self, tr: Transaction) {
        self.mempool.push(tr);
    }

    pub fn mine_pending_trans(& mut self, minor: &str) {
        if self.mempool.is_empty() {
            return;
        }
        self.add_block(minor);
        self.mempool.clear();

    }

    pub fn mempool_size(&self) -> usize {
        self.mempool.len()
    }

}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "=================BlockChain=================");
        for(i, block) in self.chain.iter().enumerate() {
            write!(f, "{}", block)?;
            if i < self.chain.len() - 1 {
                let _ = writeln!(f, "\n-------------------------------------");
            }
        }
        Ok(())
     }   
}

/* ################ Tests ################# */

#[cfg(test)]
mod tests {
    use crate::wallet::Wallet;

use super::* ;

    #[test]
    fn test_new() {
        let idx: u64 = 0 ;
        let trs = vec![
            Transaction::new("Alice", "Bob", 10.5),
        ];
        let prev_hash = "000000000";
        let blk = Block::new(idx, trs, prev_hash);

        assert_eq!(blk.index, 0);
        assert_eq!(blk.transactions[0].sender.as_str(), "Alice");
        assert_eq!(blk.nonce, 0);
    }

    #[test]
    fn test_chain() {
        let mut bchain = BlockChain::new(3);
        let trs = vec![
            Transaction::new("Alice", "Bob", 10.5),
        ];
        for tr in trs {
            bchain.submit_transaction(tr);
        }
    
        bchain.mine_pending_trans("Alice");

        let lblock = bchain.last_block();
        assert_eq!(lblock.index, 1);
        assert_eq!(lblock.transactions[0].sender, String::from("SYSTEM"));
        assert_eq!(lblock.transactions[1].recipient, String::from("Bob"));
        let lblock = &bchain.chain[1];
        let mblock  = &bchain.chain[0];
        assert_eq!(lblock.prev_hash, mblock.hash);
        assert_ne!(lblock.nonce, 0);
    }

    #[test] 
    fn test_wallet() {
        let alice = Wallet::new();
        let bob = Wallet::new();
        let mut bchain = BlockChain::new(3);
        
        let mut tr =  Transaction::new(&alice.public_key, &bob.public_key, 20.5);
       
        alice.sign(&mut tr);
        bchain.submit_transaction(tr);
            
        
       bchain.mine_pending_trans(&alice.public_key);
       assert_eq!(alice.balance(&bchain), 29.5);
       assert_eq!(bchain.is_valid(), true);
    }
}