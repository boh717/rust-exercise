use crate::domain::client_id::ClientId;
use crate::domain::input_record::InputRecord;
use crate::domain::transaction_status::TransactionStatus;
use crate::domain::tx_id::TxId;
use crate::domain::tx_type::TxType;
use crate::error::transaction_error::TransactionError;
#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: TxId,
    pub client_id: ClientId,
    pub amount: f64,
    pub tx_type: TxType,
    pub status: TransactionStatus,
}

impl Transaction {
    pub fn new(id: TxId, client_id: ClientId, amount: f64, tx_type: TxType) -> Self {
        Self {
            id,
            client_id,
            amount,
            tx_type,
            status: TransactionStatus::Confirmed,
        }
    }

    pub fn is_under_dispute(&self) -> bool {
        self.status == TransactionStatus::Disputed
    }
}

impl TryFrom<&InputRecord> for Transaction {
    type Error = TransactionError;

    fn try_from(record: &InputRecord) -> Result<Self, Self::Error> {
        match record.amount {
            Some(amount) => Ok(Transaction::new(
                record.tx,
                record.client,
                amount,
                record.tx_type,
            )),
            None => Err(TransactionError::InvalidTransaction { id: record.tx }),
        }
    }
}
