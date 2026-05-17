use std::{fmt::{self}, time::{SystemTime, UNIX_EPOCH}};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct BlockChain {
    chain: Vec<Block>,
    pub difficulty: usize,
}

impl BlockChain {
    pub fn new(difficulty : usize) -> Self {
        let genesis = Block::new(0, Vec::new(), "0".repeat(64).as_str());
        Self {
            chain: vec![genesis],
            difficulty, 
        }
    }

    pub fn add_block(&mut self, trs: Vec<Transaction>) {

        let last_block = self.last_block();
        let idx = last_block.index + 1;
        let prev_hash = &last_block.hash.clone();
        let mut new_block = Block::new(idx, trs, prev_hash);
        
        new_block.mine(self.difficulty);

        self.chain.push(new_block);
    }

    pub fn last_block(&mut self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn is_valid(&self) -> bool {
        /*
            1. is hash == calc_hash()
            2. does prev_hash == hash of prev block
         */

        for window in self.chain.windows(2) {
            let prev = &window[0];
            let curr = &window[1];

            if curr.hash != curr.calc_hash() {
                return false;
            }

            if curr.prev_hash != prev.hash {
                return false;
            }
        }
        return true;
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

/* ################################# */

#[derive(Debug)]
pub struct Block {
    index : u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    prev_hash: String,
    hash: String,
    nonce: u64,  
}

impl Block {
    fn new(index: u64, trs: Vec<Transaction>, prev_hash: &str) -> Block {
        let ts  = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let mut block = Block { 
            index,
            timestamp: ts, 
            transactions: trs, 
            prev_hash: prev_hash.to_string(), 
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calc_hash();
        block
    }

    fn calc_hash(&self) -> String {
        let trans_to_str : String = self.transactions.iter()
                                    .map(|tx| {
                                        format!("{}{}{}",tx.sender, tx.recipient, tx.amount)
                                    }).collect();

        let word = format!("{}{}{}{}{}",
                            self.index.to_string(),
                            self.timestamp.to_string(),
                            self.prev_hash,
                            trans_to_str, 
                            self.nonce);
        hex::encode(Sha256::digest(word.as_bytes()))
    }

    fn mine(&mut self, difficulty: usize) {

        let zeros = "0".repeat(difficulty);

        while !self.hash.starts_with(&zeros) {
            self.nonce += 1;
            self.hash = self.calc_hash();
        }
    }

    fn total_amount(&self) -> f64 {
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

/* ################################# */

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64, 
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: f64) -> Self {
        Self { 
            sender: sender.to_string(), 
            recipient: recipient.to_string(), 
            amount,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "{}->{}: {:.2}", self.sender, self.recipient, self.amount)?;
        Ok(())
    }
}




/* ################ Tests ################# */

#[cfg(test)]
mod tests {
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
        bchain.add_block(trs);

        let lblock = bchain.last_block();
        assert_eq!(lblock.index, 1);
        assert_eq!(lblock.transactions[0].sender, String::from("Alice"));
        let lblock = &bchain.chain[1];
        let mblock  = &bchain.chain[0];
        assert_eq!(lblock.prev_hash, mblock.hash);
        assert_ne!(lblock.nonce, 0);
    }
}