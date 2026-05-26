use ed25519_dalek::{SigningKey, Signature, Signer, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Wallet {
    pub signing_key : SigningKey,
    pub public_key : String,
}

impl Wallet {
    fn new() -> Self {
        let signing_key : SigningKey = SigningKey::generate(& mut OsRng);
        let public_key: String = hex::encode(signing_key.verifying_key().as_bytes());
        Wallet {
            public_key,
            signing_key, 
        }
    }

    fn sign(&self, tr: & mut Transaction) {

    }
}