use std::{fs};
use crate::blockchain::BlockChain;


pub fn save(blockchain: &BlockChain, path: &str) {
    let json = serde_json::to_string_pretty(blockchain).unwrap();
    fs::write(path, json).expect("couldn't save to file");
}

pub fn load(path: &str) -> BlockChain {
    match std::fs::read_to_string(path) {
        Ok(json) => serde_json::from_str(&json).unwrap(),
        Err(_)   => BlockChain::new(3),
    }
}

pub fn load_wallet(public_key: &str) -> Option<String> {
    let filename = format!("wallet_{}.json", &public_key[..8]);
    match std::fs::read_to_string(&filename) {
        Ok(json) => {
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            Some(v["private_key"].as_str().unwrap().to_string())
        }
        Err(_) => None
    }
}