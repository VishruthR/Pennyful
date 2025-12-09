use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Transaction {
    pub date: String,
    pub description: String,
    pub amount: String,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: {}, {}, {}", self.date, self.description, self.amount)
    }
}