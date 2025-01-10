use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OperatorsResponse {
    operators: Option<Vec<Operator>>,

    pagination: Option<Pagination>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Operator {
    id: Option<i64>,

    id_str: Option<String>,

    declared_fee: Option<String>,

    previous_fee: Option<String>,

    fee: Option<String>,

    public_key: Option<String>,

    owner_address: Option<String>,

    address_whitelist: Option<String>,

    is_private: Option<bool>,

    whitelisting_contract: Option<String>,

    location: Option<String>,

    setup_provider: Option<String>,

    eth1_node_client: Option<String>,

    eth2_node_client: Option<String>,

    mev_relays: Option<String>,

    description: Option<String>,

    website_url: Option<String>,

    twitter_url: Option<String>,

    linkedin_url: Option<String>,

    dkg_address: Option<String>,

    logo: Option<String>,

    #[serde(rename = "type")]
    operator_type: Option<String>,

    name: Option<String>,

    performance: Option<HashMap<String, i64>>,

    is_valid: Option<bool>,

    is_deleted: Option<bool>,

    is_active: Option<i64>,

    status: Option<String>,

    validators_count: Option<i64>,

    version: Option<String>,

    network: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    total: Option<i64>,

    pages: Option<i64>,

    per_page: Option<i64>,

    page: Option<i64>,

    current_first: Option<i64>,

    current_last: Option<i64>,
}
