use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use crate::importers::types::Transaction;

const HEADER_START: &str = "Date";
// Columns of transactions are delimited by at least two spaces
const COLUMN_DELIMETER: &str = "  ";

pub fn load_file<P: AsRef<Path>>(filename: P) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

pub fn parse_statement<P: AsRef<Path>>(filename: P) -> Result<Vec<Transaction>, std::io::Error> {
    let reader = load_file(filename)?;

    // TODO: We filter out lines that may have errors, we should handle them explicitly
    let line_iter = reader
        .lines()
        .map_while(|item| item.ok());

    let transactions: Vec<Transaction> = line_iter
        // Skip rows until you pass the header
        .skip_while(|item| !item.starts_with(HEADER_START))
        // Skip header and "Beginning balance..." line
        .skip(2)
        // TODO: We filter out lines that may have errors, we should handle them explicitly
        .filter_map(|transaction_line| {
            let transaction_parts: Vec<&str> = transaction_line
                .split(COLUMN_DELIMETER)
                .filter(|part| !part.is_empty())
                .map(|part| part.trim())
                .collect();
            
            if transaction_parts.len() < 3 {
                return None;
            }

            Some(Transaction {
                date: transaction_parts[0].to_string(),
                description: transaction_parts[1].to_string(),
                amount: transaction_parts[2].to_string()
            })
        })
        .collect();

    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_statement() -> Result<(), Box<dyn std::error::Error>> {
        const TRANSACTIONS_PATH: &str = "../data/bankofamericatransactions.txt";
        let transactions_path = PathBuf::from(TRANSACTIONS_PATH);
        let transactions_expected = vec![
            Transaction {
                date: "10/20/2025".to_string(),
                description: "MTA*NYCT PAYGO 10/17 MOBILE PURCHASE NEW YORK NY".to_string(),
                amount: "-2.90".to_string()
            },
            Transaction {
                date: "10/20/2025".to_string(),
                description: "MTA*NYCT PAYGO 10/17 MOBILE PURCHASE NEW YORK NY".to_string(),
                amount: "-2.90".to_string()
            },
            Transaction {
                date: "10/20/2025".to_string(),
                description: "PAYPAL *NJ TRANSIT 10/17 PURCHASE 123-456-7890 NJ".to_string(),
                amount: "-5.30".to_string()
            },
            Transaction {
                date: "10/22/2025".to_string(),
                description: "VENMO DES:PAYMENT ID:XXXXX INDN:John Doe CO ID:XXXXX WEB".to_string(),
                amount: "-15.00".to_string()
            },
            Transaction {
                date: "11/03/2025".to_string(),
                description: "Online Recurring transfer from CHK 1111 Confirmation# xxxxxxx; DOE, JANE".to_string(),
                amount: "1,000.00".to_string()
            },
        ];

        let transactions = parse_statement(&transactions_path)?;
        assert_eq!(transactions_expected, transactions);
        
        Ok(())
    }
}