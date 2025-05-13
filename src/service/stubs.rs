use crate::domain::{client::Client, client_id::ClientId, transaction::Transaction, tx_id::TxId};
use crate::error::{ClientError, TransactionError};
use crate::repository::{
    client_repository::ClientRepository, transaction_repository::TransactionRepository,
};

pub struct TestClientRepository {
    client: Option<Client>,
}

impl TestClientRepository {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub fn with_client(client: Client) -> Self {
        Self {
            client: Some(client),
        }
    }
}

impl ClientRepository for TestClientRepository {
    fn get_client(&self, id: &ClientId) -> anyhow::Result<Client, ClientError> {
        match &self.client {
            Some(client) if client.id == *id => Ok(client.clone()),
            _ => Err(ClientError::NotFound { id: id.clone() }),
        }
    }

    fn create_client(&mut self, client: &Client) -> anyhow::Result<Client, ClientError> {
        self.client = Some(client.clone());
        Ok(client.clone())
    }

    fn update_client(&mut self, client: &Client) -> anyhow::Result<Client, ClientError> {
        self.client = Some(client.clone());
        Ok(client.clone())
    }

    fn get_all_clients(&self) -> impl Iterator<Item = &Client> {
        Vec::<&Client>::new().into_iter()
    }
}

pub struct TestTransactionRepository {}

impl TransactionRepository for TestTransactionRepository {
    fn get_transaction(&self, id: &TxId) -> Result<Transaction, TransactionError> {
        Err(TransactionError::NotFound { id: id.clone() })
    }

    fn get_transaction_under_dispute(&self, id: &TxId) -> Result<Transaction, TransactionError> {
        Err(TransactionError::NotFound { id: id.clone() })
    }

    fn create_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Transaction, TransactionError> {
        Ok(transaction.clone())
    }

    fn update_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Transaction, TransactionError> {
        Ok(transaction.clone())
    }
}

pub struct DisputeTransactionRepository {
    pub transaction: Transaction,
}

impl TransactionRepository for DisputeTransactionRepository {
    fn get_transaction(&self, id: &TxId) -> Result<Transaction, TransactionError> {
        if self.transaction.id == *id {
            Ok(self.transaction.clone())
        } else {
            Err(TransactionError::NotFound { id: id.clone() })
        }
    }

    fn get_transaction_under_dispute(&self, id: &TxId) -> Result<Transaction, TransactionError> {
        Err(TransactionError::NotFound { id: id.clone() })
    }

    fn create_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Transaction, TransactionError> {
        Ok(transaction.clone())
    }

    fn update_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Transaction, TransactionError> {
        Ok(transaction.clone())
    }
}

pub struct ChargebackTransactionRepository {
    pub transaction: Transaction,
}

impl TransactionRepository for ChargebackTransactionRepository {
    fn get_transaction(&self, id: &TxId) -> Result<Transaction, TransactionError> {
        if self.transaction.id == *id {
            Ok(self.transaction.clone())
        } else {
            Err(TransactionError::NotFound { id: id.clone() })
        }
    }

    fn get_transaction_under_dispute(&self, id: &TxId) -> Result<Transaction, TransactionError> {
        if self.transaction.id == *id {
            Ok(self.transaction.clone())
        } else {
            Err(TransactionError::NotFound { id: id.clone() })
        }
    }

    fn create_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Transaction, TransactionError> {
        Ok(transaction.clone())
    }

    fn update_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Transaction, TransactionError> {
        Ok(transaction.clone())
    }
}
