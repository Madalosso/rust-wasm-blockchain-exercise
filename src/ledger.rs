use crate::transaction::Transaction;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ledger {
    // consider using an enum to represent accounts
    pub accounts: HashMap<String, f64>,
}
impl Ledger {
    pub fn new() -> Ledger {
        let mut accounts = HashMap::new();

        // Initialize with 3 accounts: Alice, Bob, and Charlie, each with a balance of 100.
        accounts.insert("Alice".to_string(), 100.0);
        accounts.insert("Bob".to_string(), 100.0);
        accounts.insert("Charlie".to_string(), 100.0);

        Ledger { accounts }
    }

    pub fn get_balance(&self, account_name: &str) -> Option<f64> {
        self.accounts.get(account_name).copied()
    }

    pub fn process_transaction(&mut self, transaction: &Transaction) -> Result<(), &str> {
        // Validate transaction.
        if let Some(origin_balance) = self.accounts.get(&transaction.origin) {
            if *origin_balance < transaction.value {
                return Err("Saldo insuficiente para a transação.");
            }
        } else {
            return Err("Conta de origem não existe.");
        }
        if self.accounts.get(&transaction.destination).is_none() {
            return Err("Conta de destino não existe.");
        }
        // Update balances after checking
        if let Some(origin_balance) = self.accounts.get_mut(&transaction.origin) {
            *origin_balance -= transaction.value;
        }
        if let Some(destination_balance) = self.accounts.get_mut(&transaction.destination) {
            *destination_balance += transaction.value;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    #[test]
    fn test_get_balance() {
        let ledger = Ledger::new();
        assert_eq!(ledger.get_balance("Alice"), Some(100.0));
        assert_eq!(ledger.get_balance("Bob"), Some(100.0));
        assert_eq!(ledger.get_balance("Charlie"), Some(100.0));
        assert_eq!(ledger.get_balance("NonExistent"), None);
    }

    #[test]
    fn test_process_transaction_success() {
        let mut ledger = Ledger::new();
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
        assert!(ledger.process_transaction(&transaction).is_ok());
        assert_eq!(ledger.get_balance("Alice"), Some(50.0));
        assert_eq!(ledger.get_balance("Bob"), Some(150.0));
    }

    #[test]
    fn test_process_transaction_insufficient_balance() {
        let mut ledger = Ledger::new();
        let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 150.0);
        assert_eq!(
            ledger.process_transaction(&transaction),
            Err("Saldo insuficiente para a transação.")
        );
        assert_eq!(ledger.get_balance("Alice"), Some(100.0));
        assert_eq!(ledger.get_balance("Bob"), Some(100.0));
    }

    #[test]
    fn test_process_transaction_nonexistent_origin() {
        let mut ledger = Ledger::new();
        let transaction = Transaction::new("NonExistent".to_string(), "Bob".to_string(), 50.0);
        assert_eq!(
            ledger.process_transaction(&transaction),
            Err("Conta de origem não existe.")
        );
        assert_eq!(ledger.get_balance("Bob"), Some(100.0));
    }

    #[test]
    fn test_process_transaction_nonexistent_destination() {
        let mut ledger = Ledger::new();
        let transaction = Transaction::new("Alice".to_string(), "NonExistent".to_string(), 50.0);
        assert_eq!(
            ledger.process_transaction(&transaction),
            Err("Conta de destino não existe.")
        );
        assert_eq!(ledger.get_balance("Alice"), Some(100.0));
    }
}
