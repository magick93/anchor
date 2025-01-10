use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountsResponse {
    #[serde(rename = "type")]
    accounts_response_type: String,

    filter: Filter,

    data: Vec<Account>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    id: i64,

    owner_address: String,

    recipient_address: Option<String>,

    network: Network,

    version: Version,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Network {
    Mainnet,
    Testnet,
}

// uses in query params
#[derive(Debug, Deserialize)]
pub struct NetworkParams {
    network: Network,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Version {
    V4,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    page: i64,

    per_page: i64,
}
