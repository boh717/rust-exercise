use crate::domain::client::Client;
use crate::domain::client_id::ClientId;
use crate::error::ClientError;
use std::collections::HashMap;

pub trait ClientRepository {
    fn get_client(&self, id: &ClientId) -> anyhow::Result<Client, ClientError>;
    fn create_client(&mut self, client: &Client) -> anyhow::Result<Client, ClientError>;
    fn update_client(&mut self, client: &Client) -> anyhow::Result<Client, ClientError>;

    fn get_all_clients(&self) -> impl Iterator<Item = &Client>;
}

#[derive(Debug, Clone)]
pub struct ClientRepositoryImpl {
    clients: HashMap<ClientId, Client>,
}

impl ClientRepositoryImpl {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}

impl ClientRepository for ClientRepositoryImpl {
    fn get_client(&self, id: &ClientId) -> anyhow::Result<Client, ClientError> {
        self.clients
            .get(id)
            .cloned()
            .ok_or(ClientError::NotFound { id: id.clone() })
    }

    fn create_client(&mut self, client: &Client) -> anyhow::Result<Client, ClientError> {
        self.clients.insert(client.id.clone(), client.clone());
        Ok(client.clone())
    }

    fn update_client(&mut self, client: &Client) -> anyhow::Result<Client, ClientError> {
        self.clients.insert(client.id.clone(), client.clone());
        Ok(client.clone())
    }

    fn get_all_clients(&self) -> impl Iterator<Item = &Client> {
        self.clients.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::client::Client;
    use crate::domain::client_id::ClientId;

    fn create_test_client(id: &str) -> Client {
        Client::new(ClientId::try_from(id.to_string()).unwrap())
    }

    fn assert_clients_equal(client1: &Client, client2: &Client) {
        assert_eq!(client1.id, client2.id);
        assert_eq!(client1.available.get(), client2.available.get());
        assert_eq!(client1.held.get(), client2.held.get());
        assert_eq!(client1.total.get(), client2.total.get());
        assert_eq!(client1.status, client2.status);
    }

    #[test]
    fn test_create_client() {
        let mut repo = ClientRepositoryImpl::new();
        let client = create_test_client("1");

        let result = repo.create_client(&client).unwrap();

        assert_clients_equal(&result, &client);
        assert!(repo.clients.contains_key(&client.id));
    }

    #[test]
    fn test_update_client() {
        let mut repo = ClientRepositoryImpl::new();
        let client = create_test_client("1");
        repo.clients.insert(client.id.clone(), client.clone());

        let mut updated_client = client.clone();
        updated_client.deposit(100.0).unwrap();

        let result = repo.update_client(&updated_client).unwrap();

        assert_clients_equal(&result, &updated_client);
    }

    #[test]
    fn test_get_client_success() {
        let mut repo = ClientRepositoryImpl::new();
        let client = create_test_client("1");
        repo.clients.insert(client.id.clone(), client.clone());

        let result = repo.get_client(&client.id).unwrap();

        assert_clients_equal(&result, &client);
    }

    #[test]
    fn test_get_client_not_found() {
        let repo = ClientRepositoryImpl::new();
        let id = ClientId::try_from("999".to_string()).unwrap();

        let result = repo.get_client(&id);

        assert!(matches!(result, Err(ClientError::NotFound { id: _ })));
    }

    #[test]
    fn test_get_all_clients() {
        let mut repo = ClientRepositoryImpl::new();
        let client1 = create_test_client("1");
        let client2 = create_test_client("2");

        repo.clients.insert(client1.id.clone(), client1.clone());
        repo.clients.insert(client2.id.clone(), client2.clone());

        let clients: Vec<&Client> = repo.get_all_clients().collect();

        assert_eq!(clients.len(), 2);
        assert!(clients.iter().any(|c| c.id == client1.id));
        assert!(clients.iter().any(|c| c.id == client2.id));
    }
}
