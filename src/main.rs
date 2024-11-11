use prettytable::{cell, row, Table};
use std::io;

pub mod block;
pub mod blockchain;
pub mod ledger;
pub mod loadcsv;
pub mod transaction;

fn main() {
    let mut input = String::new();
    println!("Press Enter to create new blockchain with genesis block.");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut bc = blockchain::Blockchain::new();
    println!("Blockchain created: {:?}", bc.ledger);

    println!("Enter to load transactions from CSV file...");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let transactions = loadcsv::load_transactions_from_csv("transactions.csv")
        .expect("Failed to load transactions");
    println!("Loaded the set of transactions:");

    // let transactions = vec![
    //     transaction::Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0),
    //     transaction::Transaction::new("Bob".to_string(), "Alice".to_string(), 5.0),
    //     transaction::Transaction::new("Charlie".to_string(), "Alice".to_string(), 15.0),
    //     transaction::Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0),
    //     transaction::Transaction::new("Bob".to_string(), "Bob".to_string(), 5.0),
    //     transaction::Transaction::new("Charlie".to_string(), "Alice".to_string(), 15.0),
    // ];

    let mut table = Table::new();
    table.add_row(row!["Origin", "Destination", "Value"]);
    for tx in &transactions {
        table.add_row(row![tx.origin, tx.destination, tx.value]);
    }
    table.printstd();

    println!("Press Enter to add transactions to the blockchain");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    for tx in transactions {
        bc.add_transaction(tx).expect("Failed to add transaction");
    }

    println!("Blockchain: {:?}", bc.ledger);
    println!("Blockchain: {:?}", bc.blocks);

    bc.print_all_hashes();
}
