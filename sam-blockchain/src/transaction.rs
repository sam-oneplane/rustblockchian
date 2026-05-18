use std::{fmt::{self}};

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

    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.sender, self.recipient, self.amount)
    }

}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "{}->{}: {:.2}", self.sender, self.recipient, self.amount)?;
        Ok(())
    }
}