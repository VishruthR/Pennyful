use std::path::Path;
use crate::{importers::types::TransactionImport, types::Transaction};
use csv::ReaderBuilder;
use rust_decimal::dec;

pub fn parse_csv_statement<P: AsRef<Path>>(filename: P) -> Result<Vec<TransactionImport>, std::io::Error> {
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_path(filename)?;

    let headers = csv::StringRecord::from(vec!["date", "name", "not_used_1", "not_used_2", "amount"]);
    // TODO: We filter out lines that may have errors, we should handle them explicitly, line 16, 22
    let transactions: Vec<TransactionImport> = reader
        .records()
        .filter_map(|item| item.ok())
        .filter_map(|transaction_record| transaction_record.deserialize(Some(&headers)).ok())
        .map(|transaction: TransactionImport| TransactionImport { 
            date: transaction.date, 
            name: transaction.name, 
            amount: transaction.amount * dec!(-1) 
        })
        .collect();

    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use chrono::NaiveDate;
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn test_parse_statement() -> Result<(), Box<dyn std::error::Error>> {
        const TRANSACTIONS_PATH: &str = "../data/americanexpresstransactions.csv";
        let transactions_path = PathBuf::from(TRANSACTIONS_PATH);
        let transactions_expected = vec![
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 23).unwrap(),
                name: "UBERBV              CITY           ST".to_string(),
                amount: dec!(-6.71),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 22).unwrap(),
                name: "XXXX#FLIGHT.AIRLINE               FA".to_string(),
                amount: dec!(-2404.89),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 22).unwrap(),
                name: "AMEX Airline Fee Reimbursement".to_string(),
                amount: dec!(27.60),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 22).unwrap(),
                name: "AplPay UMM PLAZA GIFCITY             ST".to_string(),
                amount: dec!(-6.62),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 21).unwrap(),
                name: "SOME AIRLINES   XXX-XXX-XXXX        ST".to_string(),
                amount: dec!(-198.48),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 21).unwrap(),
                name: "AMEX CLEAR PLUS CREDIT".to_string(),
                amount: dec!(209.00),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 21).unwrap(),
                name: "CLEAR *CLEARME.COM  CITY            ST".to_string(),
                amount: dec!(-209.00),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 21).unwrap(),
                name: "HALF PRICE THINGS    CITY            ST".to_string(),
                amount: dec!(-4.41),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 21).unwrap(),
                name: "REGISTRATION FEE       CITY".to_string(),
                amount: dec!(-355.00),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 20).unwrap(),
                name: "AplPay AIRLINE AIRLINCITY            ST".to_string(),
                amount: dec!(-35.00),
            },
        ];

        let transactions = parse_csv_statement(&transactions_path)?;

        // assert_eq!(transactions, transactions_expected);

        use std::iter::zip;
        for (transaction, expected) in zip(transactions, transactions_expected) {
            assert_eq!(transaction, expected);
        }
        Ok(())
    }
}
