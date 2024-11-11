use crate::block::Block;
use crate::ledger::Ledger;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub ledger: Ledger,
    pub pending_transactions: Vec<Transaction>,
}
pub const TX_PER_BLOCK: u8 = 2;

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut genesis_block = Block::new("0".to_string());
        genesis_block.calculate_hash();
        Blockchain {
            blocks: vec![genesis_block],
            ledger: Ledger::new(),
            pending_transactions: vec![],
        }
    }

    pub fn validate(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn all_hashes(&self) -> Vec<String> {
        self.blocks.iter().map(|b| b.hash.clone()).collect()
    }

    pub fn print_all_hashes(&self) {
        for block in &self.blocks {
            println!("Hash: {}", block.hash);
        }
    }

    // user input for a transaction
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), &str> {
        println!("Tx {} adicionada ao bloco pendente.", transaction.id);
        self.pending_transactions.push(transaction);
        if self.pending_transactions.len() >= TX_PER_BLOCK.into() {
            println!("Bloco completo. Mineração...");
            let previous_hash = self.blocks.last().unwrap().hash.clone();
            let mut new_block = Block::new(previous_hash);
            for tx in &self.pending_transactions {
                // self.ledger.process_transaction(&transaction).unwrap();

                // this looks like poor usage of clone and borrow system
                self.ledger.process_transaction(&tx.clone()).unwrap();
                new_block.add_transaction(tx.clone());
            }
            new_block.calculate_hash();
            self.blocks.push(new_block);
            println!("Novo bloco adicionado a chain");
            self.pending_transactions.clear();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;
    use crate::transaction::Transaction;

    #[test]
    fn test_all_hashes() {
        let mut blockchain = Blockchain::new();
        let previous_hash = blockchain.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(previous_hash);
        blockchain.blocks.push(new_block);

        let hashes = blockchain.all_hashes();
        assert_eq!(hashes.len(), 2); // Genesis block + new block
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new();
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);

        let result = blockchain.add_transaction(transaction.clone());
        assert!(result.is_ok());
        assert_eq!(blockchain.pending_transactions.len(), 1);
        assert_eq!(blockchain.pending_transactions[0].id, transaction.id);

        // Add more transactions to trigger block mining
        for _ in 1..TX_PER_BLOCK {
            let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
            blockchain.add_transaction(transaction).unwrap();
        }

        assert_eq!(blockchain.pending_transactions.len(), 0); // Transactions should be cleared after mining
        assert_eq!(blockchain.blocks.len(), 2); // Genesis block + new block
    }

    #[test]
    fn test_validate() {
        let mut blockchain = Blockchain::new();

        // Create a valid block and add it to the blockchain
        let previous_hash = blockchain.blocks.last().unwrap().hash.clone();
        let mut valid_block = Block::new(previous_hash.clone());
        valid_block.calculate_hash();
        blockchain.blocks.push(valid_block);

        // Blockchain should be valid
        assert!(blockchain.validate());

        // Create an invalid block with incorrect previous_hash
        let mut invalid_block = Block::new("invalid_previous_hash".to_string());
        invalid_block.calculate_hash();
        blockchain.blocks.push(invalid_block);

        // Blockchain should be invalid
        assert!(!blockchain.validate());
    }
}
