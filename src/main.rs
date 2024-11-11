pub mod block;
pub mod blockchain;
pub mod ledger;
pub mod transaction;

fn main() {
    let mut bc = blockchain::Blockchain::new();

    let transactions = vec![
        transaction::Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0),
        transaction::Transaction::new("Bob".to_string(), "Alice".to_string(), 5.0),
        transaction::Transaction::new("Charlie".to_string(), "Alice".to_string(), 15.0),
        transaction::Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0),
        transaction::Transaction::new("Bob".to_string(), "Bob".to_string(), 5.0),
        transaction::Transaction::new("Charlie".to_string(), "Alice".to_string(), 15.0),
    ];

    for tx in transactions {
        bc.add_transaction(tx);
    }

    println!("Blockchain: {:?}", bc.ledger);
    println!("Blockchain: {:?}", bc.blocks);

    bc.print_all_hashes();
}
