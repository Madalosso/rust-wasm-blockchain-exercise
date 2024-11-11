use crate::blockchain::TX_PER_BLOCK;
use crate::transaction::Transaction;
use chrono::prelude::*;
use sha2::{Digest, Sha256};

use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct Block {
    id: u64,
    timestamp: DateTime<Local>,
    transactions: Vec<Transaction>, //set max tx
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(previous_hash: String) -> Block {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

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
        self.timestamp = now;

        for tx in &self.transactions {
            hasher.update(tx.id.to_string() + &tx.value.to_string() + &tx.origin + &tx.destination);
        }

        let hash = hasher.finalize();
        println!("hash: {:x}", hash);
        self.hash = format!("{:x}", hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    #[test]
    fn test_add_transaction() {
        let mut block = Block::new("previous_hash".to_string());

        // Add transactions until the block is full
        for i in 0..TX_PER_BLOCK {
            let transaction = Transaction::new(
                format!("origin_{}", i),
                format!("destination_{}", i),
                i as f64,
            );
            assert!(block.add_transaction(transaction).is_ok());
        }

        // Adding another transaction should fail
        let transaction = Transaction::new("origin".to_string(), "destination".to_string(), 10.0);
        assert!(block.add_transaction(transaction).is_err());
    }

    #[test]
    fn test_calculate_hash() {
        let mut block = Block::new("previous_hash".to_string());

        // Add a transaction
        let transaction = Transaction::new("origin".to_string(), "destination".to_string(), 10.0);
        block.add_transaction(transaction).unwrap();

        // Calculate the hash
        block.calculate_hash();

        // Check that the hash is not empty
        assert!(!block.hash.is_empty());
    }
}
