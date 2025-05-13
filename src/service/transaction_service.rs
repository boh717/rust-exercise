use crate::domain::client::Client;
use crate::domain::client_id::ClientId;
use crate::domain::input_record::InputRecord;
use crate::domain::output_record::OutputRecord;
use crate::domain::transaction::Transaction;
use crate::domain::transaction_status::TransactionStatus;
use crate::domain::tx_id::TxId;
use crate::domain::tx_type::TxType;
use crate::error::ClientError;
use crate::repository::client_repository::ClientRepository;
use crate::repository::transaction_repository::TransactionRepository;

pub struct TransactionService<T, V>
where
    T: ClientRepository,
    V: TransactionRepository,
{
    client_repository: T,
    transaction_repository: V,
}

impl<T, V> TransactionService<T, V>
where
    T: ClientRepository,
    V: TransactionRepository,
{
    pub fn new(client_repository: T, transaction_repository: V) -> Self {
        Self {
            client_repository,
            transaction_repository,
        }
    }

    pub fn process_transaction(&mut self, record: &InputRecord) -> anyhow::Result<Client> {
        match record.tx_type {
            TxType::Dispute | TxType::Resolve | TxType::Chargeback => {
                self.process_existing_transaction(record.tx_type, record.tx, record.client)
            }
            TxType::Deposit | TxType::Withdrawal => {
                let transaction = Transaction::try_from(record)?;
                self.process_new_transaction(transaction)
            }
        }
    }

    pub fn get_all_clients(&self) -> impl Iterator<Item = OutputRecord> {
        self.client_repository
            .get_all_clients()
            .map(OutputRecord::from)
    }

    fn process_new_transaction(&mut self, transaction: Transaction) -> anyhow::Result<Client> {
        let client = match transaction.tx_type {
            TxType::Deposit => {
                let mut client = self.get_or_create_client(&transaction.client_id)?;
                client.deposit(transaction.amount)?;
                client
            }
            TxType::Withdrawal => {
                let mut client = self.client_repository.get_client(&transaction.client_id)?;
                client.withdraw(transaction.amount)?;
                client
            }
            _ => anyhow::bail!(
                "Invalid transaction type for new transaction: {:?}",
                transaction
            ),
        };

        self.client_repository.update_client(&client)?;

        self.transaction_repository
            .create_transaction(&transaction)?;

        Ok(client)
    }

    fn process_existing_transaction(
        &mut self,
        tx_type: TxType,
        tx_id: TxId,
        client_id: ClientId,
    ) -> anyhow::Result<Client> {
        let mut client = self.client_repository.get_client(&client_id)?;

        match tx_type {
            TxType::Dispute => {
                let transaction = self.transaction_repository.get_transaction(&tx_id)?;
                client.dispute(transaction.amount)?;

                let mut updated_tx = transaction.clone();
                updated_tx.status = TransactionStatus::Disputed;
                self.transaction_repository
                    .update_transaction(&updated_tx)?;
            }
            TxType::Resolve => {
                let transaction = self
                    .transaction_repository
                    .get_transaction_under_dispute(&tx_id)?;
                client.resolve(transaction.amount)?;
            }
            TxType::Chargeback => {
                let transaction = self
                    .transaction_repository
                    .get_transaction_under_dispute(&tx_id)?;
                client.chargeback(transaction.amount)?;
            }
            _ => anyhow::bail!(
                "Invalid transaction type for existing transaction: {:?}",
                tx_type
            ),
        }

        self.client_repository.update_client(&client)?;
        Ok(client)
    }

    fn get_or_create_client(&mut self, id: &ClientId) -> anyhow::Result<Client, ClientError> {
        match self.client_repository.get_client(id) {
            Ok(client) => Ok(client),
            Err(ClientError::NotFound { .. }) => {
                let client = Client::new(id.clone());
                self.client_repository.create_client(&client)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ClientStatus;
    use crate::service::stubs::*;

    #[test]
    fn test_process_deposit_transaction() {
        let client_id = ClientId::try_from("1".to_string()).unwrap();
        let tx_id = TxId::try_from("100".to_string()).unwrap();
        let amount = 5.0;

        let client_repo = TestClientRepository::new();
        let transaction_repo = TestTransactionRepository {};
        let mut service = TransactionService::new(client_repo, transaction_repo);

        let input_record = InputRecord {
            client: client_id.clone(),
            tx: tx_id,
            tx_type: TxType::Deposit,
            amount: Some(amount),
        };

        let client = service.process_transaction(&input_record).unwrap();

        assert_eq!(client.id, client_id);
        assert_eq!(client.available.get(), amount);
        assert_eq!(client.total.get(), amount);
        assert_eq!(client.held.get(), 0.0);
        assert_eq!(client.status, ClientStatus::Active);
    }

    #[test]
    fn test_process_withdrawal_transaction() {
        let client_id = ClientId::try_from("1".to_string()).unwrap();
        let tx_id = TxId::try_from("100".to_string()).unwrap();
        let initial_amount = 10.0;
        let withdrawal_amount = 5.0;

        let mut initial_client = Client::new(client_id.clone());
        initial_client.deposit(initial_amount).unwrap();

        let client_repo = TestClientRepository::with_client(initial_client);
        let transaction_repo = TestTransactionRepository {};
        let mut service = TransactionService::new(client_repo, transaction_repo);

        let input_record = InputRecord {
            client: client_id,
            tx: tx_id,
            tx_type: TxType::Withdrawal,
            amount: Some(withdrawal_amount),
        };

        let client = service.process_transaction(&input_record).unwrap();

        assert_eq!(client.id, client_id);
        assert_eq!(client.available.get(), 5.0);
        assert_eq!(client.total.get(), 5.0);
        assert_eq!(client.held.get(), 0.0);
        assert_eq!(client.status, ClientStatus::Active);
    }

    #[test]
    fn test_process_withdrawal_transaction_without_client() {
        let client_id = ClientId::try_from("1".to_string()).unwrap();
        let tx_id = TxId::try_from("100".to_string()).unwrap();
        let withdrawal_amount = 5.0;

        let client_repo = TestClientRepository::new();
        let transaction_repo = TestTransactionRepository {};
        let mut service = TransactionService::new(client_repo, transaction_repo);

        let input_record = InputRecord {
            client: client_id,
            tx: tx_id,
            tx_type: TxType::Withdrawal,
            amount: Some(withdrawal_amount),
        };

        let err = service.process_transaction(&input_record).unwrap_err();
        assert!(err.is::<ClientError>());
    }

    #[test]
    fn test_process_dispute_transaction() {
        let client_id = ClientId::try_from("1".to_string()).unwrap();
        let tx_id = TxId::try_from("100".to_string()).unwrap();
        let amount = 10.0;

        let mut initial_client = Client::new(client_id.clone());
        initial_client.deposit(amount).unwrap();

        let original_tx = Transaction {
            id: tx_id,
            client_id: client_id,
            tx_type: TxType::Deposit,
            amount: amount,
            status: TransactionStatus::Confirmed,
        };

        let transaction_repo = DisputeTransactionRepository {
            transaction: original_tx,
        };
        let mut service = TransactionService::new(
            TestClientRepository::with_client(initial_client),
            transaction_repo,
        );

        let dispute_record = InputRecord {
            client: client_id,
            tx: tx_id,
            tx_type: TxType::Dispute,
            amount: None,
        };

        let client = service.process_transaction(&dispute_record).unwrap();

        assert_eq!(client.id, client_id);
        assert_eq!(client.available.get(), 0.0);
        assert_eq!(client.total.get(), amount);
        assert_eq!(client.held.get(), amount);
        assert_eq!(client.status, ClientStatus::Active);
    }

    #[test]
    fn test_process_chargeback_transaction() {
        let client_id = ClientId::try_from("1".to_string()).unwrap();
        let tx_id = TxId::try_from("100".to_string()).unwrap();
        let amount = 10.0;

        let mut initial_client = Client::new(client_id.clone());
        initial_client.deposit(amount).unwrap();
        initial_client.dispute(amount).unwrap();

        let disputed_tx = Transaction {
            id: tx_id.clone(),
            client_id: client_id.clone(),
            tx_type: TxType::Deposit,
            amount: amount,
            status: TransactionStatus::Disputed,
        };

        let transaction_repo = ChargebackTransactionRepository {
            transaction: disputed_tx,
        };
        let mut service = TransactionService::new(
            TestClientRepository::with_client(initial_client),
            transaction_repo,
        );

        let chargeback_record = InputRecord {
            client: client_id.clone(),
            tx: tx_id.clone(),
            tx_type: TxType::Chargeback,
            amount: None,
        };

        let client = service.process_transaction(&chargeback_record).unwrap();

        assert_eq!(client.id, client_id);
        assert_eq!(client.available.get(), 0.0);
        assert_eq!(client.total.get(), 0.0);
        assert_eq!(client.held.get(), 0.0);
        assert_eq!(client.status, ClientStatus::Locked);
    }

    #[test]
    fn test_process_dispute_with_nonexistent_client() {
        let client_id = ClientId::try_from("999".to_string()).unwrap();
        let tx_id = TxId::try_from("100".to_string()).unwrap();

        let client_repo = TestClientRepository::new();
        let transaction_repo = TestTransactionRepository {};
        let mut service = TransactionService::new(client_repo, transaction_repo);

        let dispute_record = InputRecord {
            client: client_id.clone(),
            tx: tx_id,
            tx_type: TxType::Dispute,
            amount: None,
        };

        let err = service.process_transaction(&dispute_record).unwrap_err();
        assert!(err.is::<ClientError>());
    }
}
