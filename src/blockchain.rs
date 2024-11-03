use crate::block::Block;
use crate::ledger::Ledger;
use crate::transaction::Transaction;

pub struct Blockchain {
    blocks: Vec<Block>,
    ledger: Ledger,
    pending_transactions: Vec<Transaction>,
}
pub const TX_PER_BLOCK: u8 = 2;

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![],
            ledger: Ledger::new(),
            pending_transactions: vec![],
        }
    }

    pub fn validate(&self) -> bool {
        // validate the blockchain
        // return true if valid
        true
    }

    // user input for a transaction
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), &str> {
        if let Err(e) = self.ledger.validate_transaction(&transaction) {
            return Err(e);
        }
        self.pending_transactions.push(transaction);
        println!("Tx adicionada ao bloco pendente.");
        if self.pending_transactions.len() >= TX_PER_BLOCK.into() {
            println!("Bloco completo. Mineração...");
            let previous_hash = self.blocks.last().unwrap().hash.clone();
            let mut new_block = Block::new(self.blocks.len() as u32, previous_hash);
            for tx in &self.pending_transactions {
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
