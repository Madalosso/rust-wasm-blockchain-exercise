use crate::transaction::Transaction;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn load_transactions_from_csv<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));
    let mut transactions = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let origin = record.get(0).ok_or("Missing origin")?.to_string();
        let destination = record.get(1).ok_or("Missing destination")?.to_string();
        let value: f64 = record.get(2).ok_or("Missing value")?.parse()?;
        transactions.push(Transaction::new(origin, destination, value));
    }

    Ok(transactions)
}
