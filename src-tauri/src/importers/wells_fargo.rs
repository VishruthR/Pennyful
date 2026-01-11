use std::path::Path;
use crate::importers::types::TransactionImport;
use csv::ReaderBuilder;

pub fn parse_csv_statement<P: AsRef<Path>>(filename: P) -> Result<Vec<TransactionImport>, std::io::Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;

    let headers = csv::StringRecord::from(vec!["date", "amount", "not_used_1", "not_used_2", "name"]);
    // TODO: We filter out lines that may have errors, we should handle them explicitly, line 14
    let transactions = reader
        .records()
        .filter_map(|transaction_record| transaction_record.ok()?.deserialize(Some(&headers)).ok())
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
        const TRANSACTIONS_PATH: &str = "../data/wellsfargotransactions.csv";
        let transactions_path = PathBuf::from(TRANSACTIONS_PATH);
        let transactions_expected = vec![
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 15).unwrap(),
                amount: dec!(-5.77),
                name: "SQ *ESPRESSO HOUSE CITY STATE".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 14).unwrap(),
                amount: dec!(-25.24),
                name: "SIGNATURE FOOD CITY STATE".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 14).unwrap(),
                amount: dec!(-15.25),
                name: "MCDONALD'S F1111 CITY STATE".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 12).unwrap(),
                amount: dec!(-10.14),
                name: "TST*PIZZA CITY STATE".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
                amount: dec!(-19.26),
                name: "FOOD MARKET #111 CITY STATE".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
                amount: dec!(-7.25),
                name: "FOOD MARKET #111 CITY STATE".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
                amount: dec!(-27.75),
                name: "SQ *THE FOOD COMPANY".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 10).unwrap(),
                amount: dec!(-4.52),
                name: "FOOD MARKET #111 CITY STATE".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 10).unwrap(),
                amount: dec!(-17.04),
                name: "SQ *FOOD GRILL CITY STATE".to_string(),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 10).unwrap(),
                amount: dec!(-3.89),
                name: "SQ *ESPRESSO HOUSE URBANA IL".to_string(),
            },
        ];

        let transactions = parse_csv_statement(&transactions_path)?;

        assert_eq!(transactions, transactions_expected);
        Ok(())
    }
}
