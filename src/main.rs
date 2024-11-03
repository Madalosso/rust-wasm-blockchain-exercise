pub mod block;
pub mod blockchain;
pub mod ledger;
pub mod transaction;
pub mod wasm_interface;

// const vs static?

fn main() {
    println!("Hello, world!");

    let mut bc = blockchain::Blockchain::new();

    let tx1 = transaction::Transaction::new(1, "Alice".to_string(), "Bob".to_string(), 10.0);
    let tx2 = transaction::Transaction::new(2, "Bob".to_string(), "Charlie".to_string(), 5.0);
    let tx3 = transaction::Transaction::new(3, "Charlie".to_string(), "Alice".to_string(), 15.0);

    let tx1Receipt = bc.add_transaction(tx1); //.unwrap();
    let tx2Receipt = bc.add_transaction(tx2); //.unwrap();
    let tx3Receipt = bc.add_transaction(tx3); //.unwrap();
}
