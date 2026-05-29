use std::{fmt::{self}, time::{SystemTime, UNIX_EPOCH}};
use sha2::{Sha256, Digest};
use crate::merkle::MerkleTree;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub index : u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,  
}

impl Block {
    pub fn new(index: u64, trs: Vec<Transaction>, prev_hash: &str) -> Block {
        let ts  = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let mut block = Block { 
            index,
            timestamp: ts, 
            transactions: trs, 
            prev_hash: prev_hash.to_string(), 
            hash: String::new(),
            nonce: 0,
        };
        block.hash = MerkleTree::merkle_root(&block.transactions);
        block
    }

    pub fn generate_hash(&self) -> String {
        /* direct map transactions to string 
        let trans_to_str : String = self.transactions.iter()
                                    .map(|tx| {
                                        format!("{}{}{}",tx.sender, tx.recipient, tx.amount)
                                    }).collect();
        */

        let word = format!("{}{}{}{}{}",
                            self.index.to_string(),
                            self.timestamp.to_string(),
                            self.prev_hash,
                            MerkleTree::merkle_root(&self.transactions), 
                            self.nonce);
        hex::encode(Sha256::digest(word.as_bytes()))
    }

    pub fn mine(&mut self, difficulty: usize) {

        let zeros = "0".repeat(difficulty);

        while !self.hash.starts_with(&zeros) {
            self.nonce += 1;
            self.hash = self.generate_hash();
        }
    }

    pub fn total_amount(&self) -> f64 {
        let mut total: f64 = 0.0;
        for trs in &self.transactions {
            total += trs.amount;
        }
        total
    }

}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "Mining Block #:{}\nTimeStamp:\t {}\nPrevHash: \t{}\nHash:\t {}\nNonce:\t {}\nTotal amount\t {}\n", 
            self.index,
            self.timestamp,
            self.prev_hash,
            if self.hash.is_empty() {"\t(empty)"} else {&self.hash},
            self.nonce,
            self.total_amount()
        )?;
        writeln!(f, "Transaction:")?;
        for trs in &self.transactions {
            writeln!(f, "\t{}", trs)?;
        }
        Ok(())
    }
}

