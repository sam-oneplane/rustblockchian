

use ed25519_dalek::{SigningKey, Signature, Signer, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use crate::transaction::Transaction;
use crate::blockchain::BlockChain;

#[derive(Debug)]
pub struct Wallet {
    signing_key : SigningKey,
    pub public_key : String,
}

impl Wallet {
    pub fn new() -> Self {
        let signing_key : SigningKey = SigningKey::generate(& mut OsRng);
        let public_key: String = hex::encode(signing_key.verifying_key().as_bytes());
        Wallet {
            public_key,
            signing_key, 
        }
    }

    pub fn sign(&self, tr: & mut Transaction) {
        let word = tr.sig_string();
        let signed = self.signing_key.sign(word.as_bytes());
        tr.signature = hex::encode(signed.to_bytes());
        tr.public_key = self.public_key.clone();
    }

    pub fn balance(&self, bc: &BlockChain) -> f64 {
        let mut balance = 0.0;

        for block in &bc.chain {
            for tr in &block.transactions {
                if tr.recipient == self.public_key {
                    balance += tr.amount;
                }
                if tr.sender == self.public_key {
                    balance -= tr.amount;
                }
            }
        }
        balance
    }

}

pub fn verify(tr: &Transaction) -> bool {
    /* decode verifying key from public key */
    if tr.signature.is_empty() || tr.public_key.is_empty() {
        return true;
    }
    let msg = tr.sig_string();

    let pub_vec = hex::decode(&tr.public_key).unwrap(); //.as_slice().try_into().unwrap();
    let pub_bytes = pub_vec.try_into().unwrap();
    let verifying_key = VerifyingKey::from_bytes(&pub_bytes).unwrap();
    /* decode signature */
    let sig_vec = hex::decode(&tr.signature).unwrap(); //.as_slice().try_into().unwrap();
    let sig_bytes: [u8; 64] = sig_vec.try_into().unwrap();
    let sig = Signature::from_bytes(&sig_bytes);
    /* verify massge : &[u8] with Signature */
    verifying_key.verify(msg.as_bytes(), &sig).is_ok()
}

