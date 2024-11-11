pub mod block;
pub mod blockchain;
pub mod ledger;
pub mod loadcsv;
pub mod transaction;

fn main() {
    let mut bc = blockchain::Blockchain::new();

    let transactions = loadcsv::load_transactions_from_csv("transactions.csv")
        .expect("Failed to load transactions");
    println!("{:?}", transactions);

    // let transactions = vec![
    //     transaction::Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0),
    //     transaction::Transaction::new("Bob".to_string(), "Alice".to_string(), 5.0),
    //     transaction::Transaction::new("Charlie".to_string(), "Alice".to_string(), 15.0),
    //     transaction::Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0),
    //     transaction::Transaction::new("Bob".to_string(), "Bob".to_string(), 5.0),
    //     transaction::Transaction::new("Charlie".to_string(), "Alice".to_string(), 15.0),
    // ];

    for tx in transactions {
        bc.add_transaction(tx).expect("Failed to add transaction");
    }

    println!("Blockchain: {:?}", bc.ledger);
    println!("Blockchain: {:?}", bc.blocks);

    bc.print_all_hashes();
}
