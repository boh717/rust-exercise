use crate::domain::client_id::ClientId;
use crate::domain::tx_id::TxId;
use crate::domain::tx_type::TxType;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Clone, Deserialize)]
pub struct InputRecord {
    #[serde(rename = "type")]
    pub tx_type: TxType,
    pub client: ClientId,
    pub tx: TxId,
    pub amount: Option<f64>,
}

impl InputRecord {
    pub fn from_csv<R: Read>(reader: R) -> csv::DeserializeRecordsIntoIter<R, InputRecord> {
        let rdr = ReaderBuilder::new()
            .flexible(true)
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_reader(reader);

        rdr.into_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tx_type::TxType;
    use std::io::Cursor;

    #[test]
    fn test_read_csv_records() {
        let csv_data = "\
type,client,tx,amount
deposit,1,1,1.0
";
        let cursor = Cursor::new(csv_data);

        let mut records = InputRecord::from_csv(cursor);

        let record = records.next().unwrap().unwrap();
        assert_eq!(record.tx_type, TxType::Deposit);
        assert_eq!(record.client.to_string(), "1");
        assert_eq!(record.tx.to_string(), "1");
        assert_eq!(record.amount, Some(1.0));

        assert!(records.next().is_none());
    }

    #[test]
    fn test_decimals() {
        let csv_data = "\
type,client,tx,amount
deposit,1,1,1.1021
";
        let cursor = Cursor::new(csv_data);

        let mut records = InputRecord::from_csv(cursor);

        let record = records.next().unwrap().unwrap();
        assert_eq!(record.tx_type, TxType::Deposit);
        assert_eq!(record.client.to_string(), "1");
        assert_eq!(record.tx.to_string(), "1");
        assert_eq!(record.amount, Some(1.1021));

        assert!(records.next().is_none());
    }

    #[test]
    fn test_fail_on_wrong_type() {
        let csv_data = "\
type,client,tx,amount
withdrawalll,1,1,1.0
";
        let cursor = Cursor::new(csv_data);

        let mut records = InputRecord::from_csv(cursor);

        let result = records.next().unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn test_support_whitespace() {
        let csv_data = "\
type,client,tx,amount
deposit,  1 ,   1    ,    1.0
";
        let cursor = Cursor::new(csv_data);
        let mut records = InputRecord::from_csv(cursor);

        let record = records.next().unwrap().unwrap();
        assert_eq!(record.tx_type, TxType::Deposit);
        assert_eq!(record.client.to_string(), "1");
        assert_eq!(record.tx.to_string(), "1");
        assert_eq!(record.amount, Some(1.0));
    }
}
