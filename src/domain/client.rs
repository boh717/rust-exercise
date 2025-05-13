use crate::domain::amounts::{AvailableAmount, HeldAmount, TotalAmount};
use crate::domain::client_id::ClientId;
use crate::domain::client_status::ClientStatus;
use crate::error::client_error::ClientError;

#[derive(Debug, Clone)]
pub struct Client {
    pub id: ClientId,
    pub available: AvailableAmount,
    pub held: HeldAmount,
    pub total: TotalAmount,
    pub status: ClientStatus,
}

impl Client {
    pub fn new(id: ClientId) -> Self {
        Self {
            id,
            available: AvailableAmount::new(0.0),
            held: HeldAmount::new(0.0),
            total: TotalAmount::new(0.0),
            status: ClientStatus::Active,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.status == ClientStatus::Locked
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), ClientError> {
        self.available.add(amount);
        self.total.add(amount);
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), ClientError> {
        if self.available.get() < amount {
            return Err(ClientError::FundsUpdateError {
                id: self.id.clone(),
                tx_type: "withdraw".to_string(),
            });
        }

        self.available.add(-amount);
        self.total.add(-amount);
        Ok(())
    }

    pub fn dispute(&mut self, amount: f64) -> Result<(), ClientError> {
        if self.available.get() < amount {
            return Err(ClientError::FundsUpdateError {
                id: self.id.clone(),
                tx_type: "dispute".to_string(),
            });
        }

        self.available.add(-amount);
        self.held.add(amount);
        Ok(())
    }

    pub fn resolve(&mut self, amount: f64) -> Result<(), ClientError> {
        if self.held.get() < amount {
            return Err(ClientError::FundsUpdateError {
                id: self.id.clone(),
                tx_type: "resolve".to_string(),
            });
        }

        self.held.add(-amount);
        self.available.add(amount);
        Ok(())
    }

    pub fn chargeback(&mut self, amount: f64) -> Result<(), ClientError> {
        self.held.add(-amount);
        self.total.add(-amount);
        self.status = ClientStatus::Locked;
        Ok(())
    }
}

#[cfg(test)]
mod client_tests {
    use super::*;

    fn setup_client() -> Client {
        Client::new(ClientId::try_from("1".to_string()).unwrap())
    }

    #[test]
    fn test_new_client() {
        let client = setup_client();

        assert_eq!(client.available.get(), 0.0);
        assert_eq!(client.held.get(), 0.0);
        assert_eq!(client.total.get(), 0.0);
        assert_eq!(client.status, ClientStatus::Active);
    }

    #[test]
    fn test_deposit() {
        let mut client = setup_client();

        let result = client.deposit(100.0);
        assert!(result.is_ok());
        assert_eq!(client.available.get(), 100.0);
        assert_eq!(client.total.get(), 100.0);
        assert_eq!(client.held.get(), 0.0);
    }

    #[test]
    fn test_withdraw_success() {
        let mut client = setup_client();

        client.deposit(100.0).unwrap();
        let result = client.withdraw(50.0);

        assert!(result.is_ok());
        assert_eq!(client.available.get(), 50.0);
        assert_eq!(client.total.get(), 50.0);
        assert_eq!(client.held.get(), 0.0);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut client = setup_client();

        client.deposit(50.0).unwrap();
        let result = client.withdraw(100.0);

        assert!(matches!(result, Err(ClientError::FundsUpdateError { .. })));
        assert_eq!(client.available.get(), 50.0);
        assert_eq!(client.total.get(), 50.0);
    }

    #[test]
    fn test_dispute_success() {
        let mut client = setup_client();

        client.deposit(100.0).unwrap();
        let result = client.dispute(30.0);

        assert!(result.is_ok());
        assert_eq!(client.available.get(), 70.0);
        assert_eq!(client.held.get(), 30.0);
        assert_eq!(client.total.get(), 100.0);
    }

    #[test]
    fn test_dispute_insufficient_available() {
        let mut client = setup_client();

        client.deposit(20.0).unwrap();
        let result = client.dispute(30.0);

        assert!(matches!(result, Err(ClientError::FundsUpdateError { .. })));
        assert_eq!(client.available.get(), 20.0);
        assert_eq!(client.held.get(), 0.0);
        assert_eq!(client.total.get(), 20.0);
    }

    #[test]
    fn test_resolve_success() {
        let mut client = setup_client();

        client.deposit(100.0).unwrap();
        client.dispute(30.0).unwrap();
        let result = client.resolve(20.0);

        assert!(result.is_ok());
        assert_eq!(client.available.get(), 90.0);
        assert_eq!(client.held.get(), 10.0);
        assert_eq!(client.total.get(), 100.0);
    }

    #[test]
    fn test_resolve_insufficient_held() {
        let mut client = setup_client();

        client.deposit(100.0).unwrap();
        client.dispute(20.0).unwrap();
        let result = client.resolve(30.0);

        assert!(matches!(result, Err(ClientError::FundsUpdateError { .. })));
        assert_eq!(client.available.get(), 80.0);
        assert_eq!(client.held.get(), 20.0);
        assert_eq!(client.total.get(), 100.0);
    }

    #[test]
    fn test_chargeback() {
        let mut client = setup_client();

        client.deposit(100.0).unwrap();
        client.dispute(30.0).unwrap();
        let result = client.chargeback(30.0);

        assert!(result.is_ok());
        assert_eq!(client.available.get(), 70.0);
        assert_eq!(client.held.get(), 0.0);
        assert_eq!(client.total.get(), 70.0);
        assert_eq!(client.status, ClientStatus::Locked);
        assert!(client.is_locked());
    }

    #[test]
    fn test_is_locked() {
        let mut client = setup_client();

        assert!(!client.is_locked());

        client.deposit(100.0).unwrap();
        client.dispute(30.0).unwrap();
        client.chargeback(30.0).unwrap();

        assert!(client.is_locked());
    }
}
