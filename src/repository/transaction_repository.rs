use crate::domain::transaction::Transaction;
use crate::domain::tx_id::TxId;
use crate::error::transaction_error::TransactionError;
use std::collections::HashMap;

pub trait TransactionRepository {
    fn get_transaction(&self, id: &TxId) -> anyhow::Result<Transaction, TransactionError>;
    fn get_transaction_under_dispute(
        &self,
        id: &TxId,
    ) -> anyhow::Result<Transaction, TransactionError>;
    fn create_transaction(
        &mut self,
        tx: &Transaction,
    ) -> anyhow::Result<Transaction, TransactionError>;
    fn update_transaction(
        &mut self,
        tx: &Transaction,
    ) -> anyhow::Result<Transaction, TransactionError>;
}

pub struct TransactionRepositoryImpl {
    transactions: HashMap<TxId, Transaction>,
}

impl TransactionRepositoryImpl {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }
}

impl TransactionRepository for TransactionRepositoryImpl {
    fn get_transaction(&self, id: &TxId) -> anyhow::Result<Transaction, TransactionError> {
        self.transactions
            .get(id)
            .cloned()
            .ok_or(TransactionError::NotFound { id: id.clone() })
    }

    fn get_transaction_under_dispute(
        &self,
        id: &TxId,
    ) -> anyhow::Result<Transaction, TransactionError> {
        let tx = self
            .transactions
            .get(id)
            .ok_or(TransactionError::NotFound { id: id.clone() })?;

        tx.is_under_dispute()
            .then(|| tx.clone())
            .ok_or(TransactionError::NotUnderDispute { id: id.clone() })
    }

    fn create_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> anyhow::Result<Transaction, TransactionError> {
        self.transactions
            .insert(transaction.id.clone(), transaction.clone());
        Ok(transaction.clone())
    }

    fn update_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> anyhow::Result<Transaction, TransactionError> {
        self.transactions
            .insert(transaction.id.clone(), transaction.clone());
        Ok(transaction.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::client_id::ClientId;
    use crate::domain::transaction::Transaction;
    use crate::domain::transaction_status::TransactionStatus;
    use crate::domain::tx_id::TxId;
    use crate::domain::tx_type::TxType;

    fn create_test_transaction(id: u32, status: TransactionStatus) -> Transaction {
        let mut tx = Transaction::new(
            TxId::try_from(id.to_string()).unwrap(),
            ClientId::try_from("1".to_string()).unwrap(),
            100.0,
            TxType::Deposit,
        );
        tx.status = status;
        tx
    }

    fn assert_transactions_equal(tx1: &Transaction, tx2: &Transaction) {
        assert_eq!(tx1.id, tx2.id);
        assert_eq!(tx1.client_id, tx2.client_id);
        assert_eq!(tx1.amount, tx2.amount);
        assert_eq!(tx1.tx_type, tx2.tx_type);
        assert_eq!(tx1.status, tx2.status);
    }

    #[test]
    fn test_create_transaction() {
        let mut repo = TransactionRepositoryImpl::new();
        let tx = create_test_transaction(1, TransactionStatus::Confirmed);

        let result = repo.create_transaction(&tx).unwrap();

        assert_transactions_equal(&result, &tx);
        assert!(repo.transactions.contains_key(&tx.id));
    }

    #[test]
    fn test_update_transaction() {
        let mut repo = TransactionRepositoryImpl::new();
        let tx = create_test_transaction(1, TransactionStatus::Confirmed);
        repo.transactions.insert(tx.id.clone(), tx.clone());

        let mut updated_tx = tx.clone();
        updated_tx.status = TransactionStatus::Disputed;

        let result = repo.update_transaction(&updated_tx).unwrap();

        assert_transactions_equal(&result, &updated_tx);
    }

    #[test]
    fn test_get_transaction_success() {
        let mut repo = TransactionRepositoryImpl::new();
        let tx = create_test_transaction(1, TransactionStatus::Confirmed);
        repo.transactions.insert(tx.id.clone(), tx.clone());

        let result = repo.get_transaction(&tx.id).unwrap();

        assert_transactions_equal(&result, &tx);
    }

    #[test]
    fn test_get_transaction_not_found() {
        let repo = TransactionRepositoryImpl::new();
        let id = TxId::try_from("999".to_string()).unwrap();

        let result = repo.get_transaction(&id);

        assert!(matches!(result, Err(TransactionError::NotFound { id: _ })));
    }

    #[test]
    fn test_get_transaction_under_dispute_success() {
        let mut repo = TransactionRepositoryImpl::new();
        let tx = create_test_transaction(1, TransactionStatus::Disputed);
        repo.transactions.insert(tx.id.clone(), tx.clone());

        let result = repo.get_transaction_under_dispute(&tx.id).unwrap();

        assert_transactions_equal(&result, &tx);
    }

    #[test]
    fn test_get_transaction_under_dispute_not_found() {
        let repo = TransactionRepositoryImpl::new();
        let id = TxId::try_from("999".to_string()).unwrap();

        let result = repo.get_transaction_under_dispute(&id);

        assert!(matches!(result, Err(TransactionError::NotFound { id: _ })));
    }

    #[test]
    fn test_get_transaction_under_dispute_not_under_dispute() {
        let mut repo = TransactionRepositoryImpl::new();
        let tx = create_test_transaction(1, TransactionStatus::Confirmed);
        repo.transactions.insert(tx.id.clone(), tx.clone());

        let result = repo.get_transaction_under_dispute(&tx.id);

        assert!(matches!(
            result,
            Err(TransactionError::NotUnderDispute { id: _ })
        ));
    }
}
