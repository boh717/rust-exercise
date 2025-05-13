use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl FromStr for TxType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "deposit" => Self::Deposit,
            "withdrawal" => Self::Withdrawal,
            "dispute" => Self::Dispute,
            "resolve" => Self::Resolve,
            "chargeback" => Self::Chargeback,
            _ => return Err(anyhow::anyhow!("Invalid transaction type: {}", s)),
        })
    }
}

impl TryFrom<String> for TxType {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}
