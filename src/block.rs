use crate::blockchain::TX_PER_BLOCK;
use crate::transaction::Transaction;
use chrono::prelude::*;
use sha2::{Digest, Sha256};

pub struct Block {
    id: u32,
    timestamp: DateTime<Local>,
    transactions: Vec<Transaction>, //set max tx
    previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(id: u32, previous_hash: String) -> Block {
        Block {
            id,
            timestamp: Local::now(),
            transactions: vec![],
            previous_hash,
            hash: String::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), &str> {
        if self.transactions.len() < TX_PER_BLOCK.into() {
            self.transactions.push(transaction);
            Ok(())
        } else {
            Err("O bloco já atingiu o limite de transações.")
            // need to Hash the block and add a next block
        }
    }

    pub fn calculate_hash(&mut self) {
        let mut hasher = Sha256::new();
        let now = Local::now();
        hasher.update(self.id.to_string() + &now.to_string() + &self.previous_hash);

        for tx in &self.transactions {
            hasher.update(tx.id.to_string() + &tx.value.to_string() + &tx.origin + &tx.destination);
        }

        let hash = hasher.finalize();
        println!("hash: {:x}", hash);
    }
}
