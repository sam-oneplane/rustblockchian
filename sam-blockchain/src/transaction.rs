use std::{fmt::{self}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub signature: String,
    pub public_key: String,
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: f64) -> Self {
        Self { 
            sender: sender.to_string(), 
            recipient: recipient.to_string(), 
            amount,
            signature: String::new(),
            public_key: String::new()
        }
    }

    pub fn sig_string(&self) -> String {
        format!("{}{}{}", self.sender, self.recipient, self.amount)
    }

}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "{}->{}: {:.2}", self.sender, self.recipient, self.amount)?;
        Ok(())
    }
}