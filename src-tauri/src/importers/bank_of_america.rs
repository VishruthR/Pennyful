use std::path::Path;
use crate::importers::types::TransactionImport;
use csv::ReaderBuilder;

const HEADER_START: &str = "Date";

pub fn parse_csv_statement<P: AsRef<Path>>(filename: P) -> Result<Vec<TransactionImport>, std::io::Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(filename)?;

    // TODO: We filter out lines that may have errors, we should handle them explicitly, line 16, 22
    let transactions = reader
        .records()
        .filter_map(|item| item.ok())
        // Skip rows until you pass the header
        .skip_while(|item| !item.as_slice().starts_with(HEADER_START))
        // Skip header and "Beginning balance..." line
        .skip(2)
        .filter_map(|transaction_record| transaction_record.deserialize(None).ok())
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
        const TRANSACTIONS_PATH: &str = "../data/bankofamericatransactions.csv";
        let transactions_path = PathBuf::from(TRANSACTIONS_PATH);
        let transactions_expected = vec![
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 10).unwrap(),
                name: "Zelle payment from JOHN DOE for food\"; Conf# 11bu1u1\"".to_string(),
                amount: dec!(5),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 10).unwrap(),
                name: "VENMO DES:PAYMENT ID:XXXXX INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-10),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 10).unwrap(),
                name: "VENMO DES:PAYMENT ID:XXXXX INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-7.20),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 10).unwrap(),
                name: "VENMO DES:PAYMENT ID:XXXXX INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-0.60),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 12).unwrap(),
                name: "COSTCO WHSE#1111 11/11 PURCHASE CITY ST".to_string(),
                amount: dec!(-141.02),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 12).unwrap(),
                name: "WELLS FARGO CARD DES:CRD ID:XXXXXXXXXX INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-481.86),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 17).unwrap(),
                name: "VENMO DES:CASHOUT ID:XXXXX INDN:JOHN DOE CO ID:XXXXX PPD".to_string(),
                amount: dec!(906.30),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 17).unwrap(),
                name: "COSTCO WHSE#1111 11/14 PURCHASE CITY ST".to_string(),
                amount: dec!(-4.44),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 21).unwrap(),
                name: "SimpleBills Prod DES:WEB PMTS ID:AAAA INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-31.94),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 25).unwrap(),
                name: "WELLS FARGO CARD DES:CRD ID:XXXXXXXXXX INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-474.74),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 11, 28).unwrap(),
                name: "VENMO DES:PAYMENT ID:XXXXX INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-49.50),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(),
                name: "Online Recurring transfer from CHK 1111 Confirmation# aaaaaaa; DOE, JANE".to_string(),
                amount: dec!(1000),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(),
                name: "PAYPAL *NEWBALANCEA 11/27 PURCHASE 111-111-11111 MO".to_string(),
                amount: dec!(-93.51),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 4).unwrap(),
                name: "BILTPYMTS DES:RENT PMT ID:XXXXXXXXXX INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-939.00),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 8).unwrap(),
                name: "PAYPAL DES:INST XFER ID:AMAZON INDN:JOHN DOE CO ID:PAYPALS WEB".to_string(),
                amount: dec!(-53.84),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 8).unwrap(),
                name: "PAYPAL DES:INST XFER ID:EBAY INDN:JOHN DOE CO ID:PAYPALS WEB".to_string(),
                amount: dec!(-46.13),
            },
            TransactionImport {
                date: NaiveDate::from_ymd_opt(2025, 12, 9).unwrap(),
                name: "WELLS FARGO CARD DES:CRD ID:XXXXXXXXXX INDN:JOHN DOE CO ID:XXXXX WEB".to_string(),
                amount: dec!(-397.57),
            },
        ];

        let transactions = parse_csv_statement(&transactions_path)?;

        assert_eq!(transactions, transactions_expected);
        Ok(())
    }
}
