use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl TryFrom<String> for TxType {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(match s.as_str() {
            "deposit" => Self::Deposit,
            "withdrawal" => Self::Withdrawal,
            "dispute" => Self::Dispute,
            "resolve" => Self::Resolve,
            "chargeback" => Self::Chargeback,
            _ => return Err(anyhow::anyhow!("Invalid transaction type: {}", s)),
        })
    }
}
