pub mod block;
pub mod blockchain;
pub mod ledger;
pub mod transaction;
pub mod wasm_interface;

// const vs static?

fn main() {
    println!("Hello, world!");

    let mut bc = blockchain::Blockchain::new();

    let tx1 = transaction::Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
    let tx2 = transaction::Transaction::new("Bob".to_string(), "Charlie".to_string(), 5.0);
    let tx3 = transaction::Transaction::new("Charlie".to_string(), "Alice".to_string(), 15.0);

    bc.add_transaction(tx1);
    bc.add_transaction(tx2);
    bc.add_transaction(tx3);

    println!("Blockchain: {:?}", bc.ledger);
}
