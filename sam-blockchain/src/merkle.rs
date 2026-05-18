use sha2::{Sha256, Digest};
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct MerkleTree ;

/* per block transactions hash algo */
impl MerkleTree {
    pub fn merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return String::from("0".repeat(64));
        }

        // Step 1: Hash all leaf transactions
        let mut current_layer: Vec<String> = transactions
            .iter()
            .map(|tx| {
                let mut hasher = Sha256::new();
                hasher.update(tx.to_string().as_bytes());
                format!("{:?}", hasher.finalize())
            })    
            .collect();

        // Step 2: Iteratively build parent layers until one root remains
        while current_layer.len() > 1 {
            let mut next_layer = Vec::new();

            for chunk in current_layer.chunks(2) {
                match chunk {
                    // Two children: hash their concatenation
                    [left, right] => {
                        let s : String = Self::hash_nodes(left, right);
                        next_layer.push(s);
                    }
                    // Odd node out: duplicate it and hash
                    [left] => {
                        let s = Self::hash_nodes(left, left);
                        next_layer.push(s);
                    }
                    _ => unreachable!(),
                }
            }
            current_layer = next_layer;
        }
        current_layer.remove(0)
    }


    fn hash_nodes(left: &str, right: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(left.as_bytes());
        hasher.update(right.as_bytes());
        format!("{:?}", hasher.finalize())
    }
}


