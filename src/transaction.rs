use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: u64,
    pub origin: String,
    pub destination: String,
    pub value: f64,
}

impl Transaction {
    pub fn new(origin: String, destination: String, value: f64) -> Transaction {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        Transaction {
            id,
            origin,
            destination,
            value,
        }
    }
}
