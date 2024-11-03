#[derive(Clone)]
pub struct Transaction {
    pub id: u64,
    pub origin: String,
    pub destination: String,
    pub value: f64,
}

impl Transaction {
    pub fn new(id: u64, origin: String, destination: String, value: f64) -> Transaction {
        Transaction {
            id,
            origin,
            destination,
            value,
        }
    }
}
