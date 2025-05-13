use crate::domain::tx_id::TxId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("transaction with id {id} not found")]
    NotFound { id: TxId },
    #[error("transaction with id {id} not under dispute")]
    NotUnderDispute { id: TxId },
    #[error("transaction with id {id} is not a valid transaction")]
    InvalidTransaction { id: TxId },
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
