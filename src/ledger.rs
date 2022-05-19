use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Ledger {
    pub transactions: Vec<Transaction>,
}

impl Ledger {

    pub fn new() -> Self {
        Self { transactions: Vec::new() }
    }
}