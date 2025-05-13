use crate::domain::client_id::ClientId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("client with id {id} not found")]
    NotFound { id: ClientId },
    #[error("error updating funds for client {id} with tx type {tx_type}")]
    FundsUpdateError { id: ClientId, tx_type: String },
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
