use crate::domain::amounts::{AvailableAmount, HeldAmount, TotalAmount};
use crate::domain::client::Client;
use crate::domain::client_id::ClientId;

#[derive(Debug, Clone, Copy)]
pub struct OutputRecord {
    pub client: ClientId,
    pub available: AvailableAmount,
    pub held: HeldAmount,
    pub total: TotalAmount,
    pub locked: bool,
}

impl From<&Client> for OutputRecord {
    fn from(client: &Client) -> Self {
        Self {
            client: client.id,
            available: client.available,
            held: client.held,
            total: client.total,
            locked: client.is_locked(),
        }
    }
}
