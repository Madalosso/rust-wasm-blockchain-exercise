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
        let genesis_block = Block::new("0".to_string());

        Blockchain {
            blocks: vec![genesis_block],
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
        // if let Err(e) = self.ledger.validate_transaction(&transaction) {
        //     return Err(e);
        // }
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
