// Base API template
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;


/// OperatorMetadataDto model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OperatorMetadataDto {
    
    /// operatorName field
    pub operatorName: Option<String>,
    
    /// description field
    pub description: Option<String>,
    
    /// location field
    pub location: Option<String>,
    
    /// setupProvider field
    pub setupProvider: Option<String>,
    
    /// eth1NodeClient field
    pub eth1NodeClient: Option<String>,
    
    /// eth2NodeClient field
    pub eth2NodeClient: Option<String>,
    
    /// mevRelays field
    pub mevRelays: Option<String>,
    
    /// websiteUrl field
    pub websiteUrl: Option<String>,
    
    /// twitterUrl field
    pub twitterUrl: Option<String>,
    
    /// linkedinUrl field
    pub linkedinUrl: Option<String>,
    
    /// dkgAddress field
    pub dkgAddress: Option<String>,
    
    /// logo field
    pub logo: Option<String>,
    
    /// signature field
    pub signature: String,
    
}

/// DkgHealthCheckDto model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DkgHealthCheckDto {
    
    /// dkgAddress field
    pub dkgAddress: String,
    
}


// Base API template
use axum::{
    Router,
    routing::{get, post, put, delete},
};
use super::handlers::*;

pub fn create_router() -> Router {
    Router::new()
    .route("/api/v4/{network}/accounts", get(handle_get_api_v4_network_accounts))
    .route("/api/v4/{network}/accounts/{ownerAddress}", get(handle_get_api_v4_network_accounts_ownerAddress))
    .route("/api/v4/{network}/accounts/counts/{ownerAddress}", get(handle_get_api_v4_network_accounts_counts_ownerAddress))
    .route("/api/v4/{network}/clusters/count", get(handle_get_api_v4_network_clusters_count))
    .route("/api/v4/{network}/clusters", get(handle_get_api_v4_network_clusters))
    .route("/api/v4/{network}/clusters/updates", get(handle_get_api_v4_network_clusters_updates))
    .route("/api/v4/{network}/clusters/{id}", get(handle_get_api_v4_network_clusters_id))
    .route("/api/v4/{network}/clusters/owner/{owner}/operators/{operators}", get(handle_get_api_v4_network_clusters_owner_owner_operators_operators))
    .route("/api/v4/{network}/clusters/owner/{owner}", get(handle_get_api_v4_network_clusters_owner_owner))
    .route("/api/v4/{network}/clusters/hash/{clusterHash}", get(handle_get_api_v4_network_clusters_hash_clusterHash))
    .route("/api/v4/{network}/duties/{validator}", get(handle_get_api_v4_network_duties_validator))
    .route("/api/v4/{network}/events/{txHash}", get(handle_get_api_v4_network_events_txHash))
    .route("/api/v4/{network}/faucet", get(handle_get_api_v4_network_faucet))
    .route("/api/v4/{network}/faucet", post(handle_post_api_v4_network_faucet))
    .route("/api/v4/{network}/faucet/config", get(handle_get_api_v4_network_faucet_config))
    .route("/api/finance/currency/convert/{symbol}/{quote}", get(handle_get_api_finance_currency_convert_symbol_quote))
    .route("/api/v4/{network}/health", get(handle_get_api_v4_network_health))
    .route("/api/v4/{network}/incentivization/merkle-tree", get(handle_get_api_v4_network_incentivization_merkle_tree))
    .route("/api/v4/{network}/operators/graph", get(handle_get_api_v4_network_operators_graph))
    .route("/api/v4/{network}/operators/owned_by/{ownerAddress}", get(handle_get_api_v4_network_operators_owned_by_ownerAddress))
    .route("/api/v4/{network}/operators/incentivized/{operator}", get(handle_get_api_v4_network_operators_incentivized_operator))
    .route("/api/v4/{network}/operators/{operator}", get(handle_get_api_v4_network_operators_operator))
    .route("/api/v4/{network}/operators/dkg_health_check", post(handle_post_api_v4_network_operators_dkg_health_check))
    .route("/api/v4/{network}/operators/public_key/{public_key}", get(handle_get_api_v4_network_operators_public_key_public_key))
    .route("/api/v4/{network}/operators", get(handle_get_api_v4_network_operators))
    .route("/api/v4/{network}/operators", post(handle_post_api_v4_network_operators))
    .route("/api/v4/{network}/operators/{operator}/metadata", put(handle_put_api_v4_network_operators_operator_metadata))
    .route("/api/v4/{network}/operators/nodes/{layer}", get(handle_get_api_v4_network_operators_nodes_layer))
    .route("/api/v4/{network}/operators/locations", get(handle_get_api_v4_network_operators_locations))
    .route("/api/v4/{network}/search", get(handle_get_api_v4_network_search))
    .route("/api/v4/{network}/validators/countActiveValidators", get(handle_get_api_v4_network_validators_countActiveValidators))
    .route("/api/v4/{network}/validators/owned_by/{ownerAddress}/cost", get(handle_get_api_v4_network_validators_owned_by_ownerAddress_cost))
    .route("/api/v4/{network}/validators/in_operator/{operator}", get(handle_get_api_v4_network_validators_in_operator_operator))
    .route("/api/v4/{network}/validators/incentivized/{validator}", get(handle_get_api_v4_network_validators_incentivized_validator))
    .route("/api/v4/{network}/validators/{validator}", get(handle_get_api_v4_network_validators_validator))
    .route("/api/v4/{network}/validators/isRegisteredValidator/{validator}", get(handle_get_api_v4_network_validators_isRegisteredValidator_validator))
    .route("/api/v4/{network}/validators/registeredByPublicKeys", post(handle_post_api_v4_network_validators_registeredByPublicKeys))
    .route("/api/v4/{network}/validators", get(handle_get_api_v4_network_validators))
    .route("/api/v4/{network}/validators/duty_counts/{from_epoch}/{to_epoch}", get(handle_get_api_v4_network_validators_duty_counts_from_epoch_to_epoch))
    .route("/api/v4/{network}/validators/validatorsByClusterHash/{clusterHash}", get(handle_get_api_v4_network_validators_validatorsByClusterHash_clusterHash))
    .route("/api/v4/{network}/validators/validatorsWithdrawCredentials", post(handle_post_api_v4_network_validators_validatorsWithdrawCredentials))
    
}

// Base API template
use axum::{ extract::{Path, Query, Json}, response::Json as JsonResponse, http::StatusCode };
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;


#[utoipa::path(GET, path = "/api/v4/{network}/accounts",  tag = "Accounts",   params(("network", String),("page", String),("perPage", String),), 
    responses( (status = 200, description = "Accounts found and returned in response", body = ()), (status = 404, description = "Requested page number does not exists", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_accounts(
        network: String, 
        page: Option<String>, 
        perPage: Option<String>, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_accounts handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/accounts/{ownerAddress}",  tag = "Accounts",   params(("network", String),("ownerAddress", String),), 
    responses( (status = 200, description = "Account found and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_accounts_ownerAddress(
        network: String, 
        ownerAddress: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_accounts_ownerAddress handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/accounts/counts/{ownerAddress}",  tag = "Accounts",  
    responses( (status = 200, description = "Counts found and returned in response", body = ()), (status = 400, description = "Owner address has wrong format", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_accounts_counts_ownerAddress(
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_accounts_counts_ownerAddress handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/clusters/count",  tag = "Clusters",   params(("network", String),), 
    responses( (status = 200, description = "Clusters counted and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_clusters_count(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_clusters_count handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/clusters",  tag = "Clusters",   params(("network", String),("from", String),("limit", i32),("operatorDetails", String),), 
    responses( (status = 200, description = "Clusters found and returned in response", body = ()), (status = 404, description = "Requested page number does not exists", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_clusters(
        network: String, 
        from: String, 
        limit: i32, 
        operatorDetails: Option<String>, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_clusters handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/clusters/updates",  tag = "Clusters",   params(("network", String),("fromBlock", String),), 
    responses( (status = 200, description = "Cluster updates found and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_clusters_updates(
        network: String, 
        fromBlock: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_clusters_updates handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/clusters/{id}",  tag = "Clusters",   params(("network", String),("id", String),("operatorDetails", String),), 
    responses( (status = 200, description = "Cluster found and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_clusters_id(
        network: String, 
        id: String, 
        operatorDetails: Option<String>, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_clusters_id handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/clusters/owner/{owner}/operators/{operators}",  tag = "Clusters",   params(("network", String),("owner", String),("operators", String),("operatorDetails", String),), 
    responses( (status = 200, description = "Cluster found and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_clusters_owner_owner_operators_operators(
        network: String, 
        owner: String, 
        operators: String, 
        operatorDetails: Option<String>, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_clusters_owner_owner_operators_operators handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/clusters/owner/{owner}",  tag = "Clusters",   params(("network", String),("page", String),("perPage", String),("ordering", String),("operatorDetails", String),("owner", String),), 
    responses( (status = 200, description = "Clusters found and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_clusters_owner_owner(
        network: String, 
        page: Option<String>, 
        perPage: Option<String>, 
        ordering: Option<String>, 
        operatorDetails: Option<String>, 
        owner: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_clusters_owner_owner handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/clusters/hash/{clusterHash}",  tag = "Clusters",   params(("network", String),("page", String),("perPage", String),("clusterHash", String),), 
    responses( (status = 200, description = "Cluster info found and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_clusters_hash_clusterHash(
        network: String, 
        page: Option<String>, 
        perPage: Option<String>, 
        clusterHash: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_clusters_hash_clusterHash handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/duties/{validator}",  tag = "Duties",   params(("page", String),("perPage", String),("network", String),("validator", String),), 
    responses( (status = 200, description = "Validator found and duties returned in response", body = ()), (status = 400, description = "Validator address has wrong format", body = ()), (status = 404, description = "Validator not found", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_duties_validator(
        page: Option<String>, 
        perPage: Option<String>, 
        network: String, 
        validator: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_duties_validator handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/events/{txHash}",  tag = "Events",  
    responses( (status = 200, description = "Event found and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_events_txHash(
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_events_txHash handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/faucet",  tag = "Faucet",   params(("network", String),), 
    responses( (status = 200, description = "OK", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_faucet(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_faucet handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(POST, path = "/api/v4/{network}/faucet",  tag = "Faucet",   params(("network", String),), 
    responses( (status = 200, description = "OK", body = ()), (status = 400, description = "One or more parameters are invalid", body = ()), (status = 406, description = "Reached max transactions per day or faucet depleted", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_post_api_v4_network_faucet(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_post_api_v4_network_faucet handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/faucet/config",  tag = "Faucet",   params(("network", String),), 
    responses( (status = 200, description = "OK", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_faucet_config(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_faucet_config handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/finance/currency/convert/{symbol}/{quote}",  tag = "Finance",   params(("symbol", String),("quote", String),), 
    responses( (status = 200, description = "Both currencies are valid and has been converted", body = Pet), (status = 404, description = "Currency can not be found or is not valid", body = Pet), (status = 429, description = "Rate limit exceeded", body = Pet), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_finance_currency_convert_symbol_quote(
        symbol: String, 
        quote: String, 
    
) -> Result<JsonResponse<Pet>, StatusCode> {
    // TODO: Implement handle_get_api_finance_currency_convert_symbol_quote handler logic
    Ok(JsonResponse(Pet::default()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/health",  tag = "Health",   params(("network", String),), 
    responses( (status = 200, description = "Health check for all endpoints", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_health(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_health handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/incentivization/merkle-tree",  tag = "Incentivized",   params(("network", String),), 
    responses( (status = 200, description = "Merkle tree found for this version and network", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_incentivization_merkle_tree(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_incentivization_merkle_tree handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/operators/graph",  tag = "Operators",   params(("page", String),("perPage", String),("randomize", String),("network", String),), 
    responses( (status = 200, description = "Operators found and returned in response", body = ()), (status = 404, description = "Requested page number does not exists", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_operators_graph(
        page: Option<String>, 
        perPage: Option<String>, 
        randomize: Option<String>, 
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_operators_graph handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/operators/owned_by/{ownerAddress}",  tag = "Operators",   params(("page", String),("perPage", String),("ordering", String),("network", String),("ownerAddress", String),), 
    responses( (status = 200, description = "Operators found and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_operators_owned_by_ownerAddress(
        page: Option<String>, 
        perPage: Option<String>, 
        ordering: Option<String>, 
        network: String, 
        ownerAddress: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_operators_owned_by_ownerAddress handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/operators/incentivized/{operator}",  tag = "Operators",   params(("epochFrom", String),("epochsPerRound", String),("rounds", String),("network", String),("operator", String),), 
    responses( (status = 200, description = "Operator found and returned in response", body = ()), (status = 400, description = "Operator address has wrong format", body = ()), (status = 404, description = "Operator not found", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_operators_incentivized_operator(
        epochFrom: String, 
        epochsPerRound: String, 
        rounds: Option<String>, 
        network: String, 
        operator: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_operators_incentivized_operator handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/operators/{operator}",  tag = "Operators",   params(("network", String),("operator", String),), 
    responses( (status = 200, description = "Operator found and returned in response", body = ()), (status = 400, description = "Operator id/address has wrong format", body = ()), (status = 404, description = "Operator not found", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_operators_operator(
        network: String, 
        operator: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_operators_operator handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(POST, path = "/api/v4/{network}/operators/dkg_health_check",  tag = "Operators",   params(("network", String),), 
    responses( (status = 200, description = "true if dkg endpoint is active, false else", body = ()), (status = 201, description = "true if dkg endpoint is active, false else", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_post_api_v4_network_operators_dkg_health_check(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_post_api_v4_network_operators_dkg_health_check handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/operators/public_key/{public_key}",  tag = "Operators",   params(("network", String),("public_key", String),), 
    responses( (status = 200, description = "Operator found and returned in response", body = ()), (status = 400, description = "Operator public key has wrong format", body = ()), (status = 404, description = "Operator not found", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_operators_public_key_public_key(
        network: String, 
        public_key: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_operators_public_key_public_key handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/operators",  tag = "Operators",   params(("operator_type", String),("page", String),("perPage", String),("ordering", String),("search", String),("has_dkg_address", String),("network", String),), 
    responses( (status = 200, description = "Operators found and returned in response", body = ()), (status = 404, description = "Requested page number does not exists", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_operators(
        operator_type: Option<String>, 
        page: Option<String>, 
        perPage: Option<String>, 
        ordering: Option<String>, 
        search: Option<String>, 
        has_dkg_address: Option<String>, 
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_operators handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(POST, path = "/api/v4/{network}/operators",  tag = "Operators",   params(("network", String),), 
    responses( (status = 200, description = "OK", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_post_api_v4_network_operators(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_post_api_v4_network_operators handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(PUT, path = "/api/v4/{network}/operators/{operator}/metadata",  tag = "Operators",   params(("network", String),("operator", String),), 
    responses( (status = 200, description = "Operator found and his metadata was updated. Final operator returned in response", body = ()), (status = 401, description = "Owner address is not authorized to perform this action", body = ()), (status = 404, description = "Operator does not exist", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_put_api_v4_network_operators_operator_metadata(
        network: String, 
        operator: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_put_api_v4_network_operators_operator_metadata handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/operators/nodes/{layer}",  tag = "Operators",   params(("network", String),("layer", String),), 
    responses( (status = 200, description = "List of available Eth node clients", body = ()), (status = 400, description = "Layer has wrong format or value", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_operators_nodes_layer(
        network: String, 
        layer: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_operators_nodes_layer handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/operators/locations",  tag = "Operators",   params(("network", String),), 
    responses( (status = 200, description = "List of available locations", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_operators_locations(
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_operators_locations handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/search",  tag = "Search",   params(("search", String),("searchFor", String),("operatorsLimit", String),("validatorsLimit", String),("network", String),), 
    responses( (status = 200, description = "Operators or validators found and returned in response", body = ()), (status = 400, description = "Search parameters provided with incorrect values", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_search(
        search: String, 
        searchFor: Option<String>, 
        operatorsLimit: Option<String>, 
        validatorsLimit: Option<String>, 
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_search handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators/countActiveValidators",  tag = "Validators",  
    responses( (status = 200, description = "Validators counted and returned in response", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators_countActiveValidators(
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators_countActiveValidators handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators/owned_by/{ownerAddress}/cost",  tag = "Validators",   params(("ownerAddress", String),("network", String),), 
    responses( (status = 200, description = "OK", body = ()), (status = 400, description = "Account address has wrong format", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators_owned_by_ownerAddress_cost(
        ownerAddress: String, 
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators_owned_by_ownerAddress_cost handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators/in_operator/{operator}",  tag = "Validators",   params(("page", String),("perPage", String),("network", String),("operator", String),), 
    responses( (status = 200, description = "Validators found and returned in response", body = ()), (status = 404, description = "No such operator or operator does not have validators", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators_in_operator_operator(
        page: Option<String>, 
        perPage: Option<String>, 
        network: String, 
        operator: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators_in_operator_operator handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators/incentivized/{validator}",  tag = "Validators",   params(("epochFrom", String),("epochsPerRound", String),("rounds", String),("network", String),("validator", String),), 
    responses( (status = 200, description = "Validator found and returned in response", body = ()), (status = 400, description = "Validator address has wrong format", body = ()), (status = 404, description = "Validator not found", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators_incentivized_validator(
        epochFrom: String, 
        epochsPerRound: String, 
        rounds: Option<String>, 
        network: String, 
        validator: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators_incentivized_validator handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators/{validator}",  tag = "Validators",   params(("network", String),("validator", String),), 
    responses( (status = 200, description = "Validator found and returned in response", body = ()), (status = 400, description = "Validator public_key has wrong format", body = ()), (status = 404, description = "Validator not found", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators_validator(
        network: String, 
        validator: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators_validator handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators/isRegisteredValidator/{validator}",  tag = "Validators",   params(("network", String),("validator", String),), 
    responses( (status = 200, description = "Validator found and returned in response", body = ()), (status = 400, description = "Validator public_key has wrong format", body = ()), (status = 404, description = "Validator not found", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators_isRegisteredValidator_validator(
        network: String, 
        validator: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators_isRegisteredValidator_validator handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(POST, path = "/api/v4/{network}/validators/registeredByPublicKeys",  tag = "Validators",  
    responses( (status = 200, description = "Validators found and returned in response", body = ()), (status = 400, description = "Public key has wrong format", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_post_api_v4_network_validators_registeredByPublicKeys(
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_post_api_v4_network_validators_registeredByPublicKeys handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators",  tag = "Validators",   params(("lastId", String),("pageDirection", String),("perPage", String),("ordering", String),("ownerAddress", String),("search", String),("network", String),), 
    responses( (status = 200, description = "Validators found and returned in response", body = ()), (status = 404, description = "Requested page number does not exists", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators(
        lastId: Option<String>, 
        pageDirection: Option<String>, 
        perPage: Option<String>, 
        ordering: Option<String>, 
        ownerAddress: Option<String>, 
        search: Option<String>, 
        network: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators/duty_counts/{from_epoch}/{to_epoch}",  tag = "Validators",   params(("network", String),("from_epoch", String),("to_epoch", String),), 
    responses( (status = 200, description = "Duty counts found and returned in response", body = ()), (status = 404, description = "Requested page number does not exists", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators_duty_counts_from_epoch_to_epoch(
        network: String, 
        from_epoch: String, 
        to_epoch: String, 
    
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators_duty_counts_from_epoch_to_epoch handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(GET, path = "/api/v4/{network}/validators/validatorsByClusterHash/{clusterHash}",  tag = "Validators",  
    responses( (status = 200, description = "Validators found and returned in response", body = ()), (status = 400, description = "Cluster hash has wrong format", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_get_api_v4_network_validators_validatorsByClusterHash_clusterHash(
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_get_api_v4_network_validators_validatorsByClusterHash_clusterHash handler logic
    Ok(JsonResponse(()))
}


#[utoipa::path(POST, path = "/api/v4/{network}/validators/validatorsWithdrawCredentials",  tag = "Validators",  
    responses( (status = 200, description = "Withdraw credentials found and returned in response", body = ()), (status = 400, description = "Public key has wrong format", body = ()), (status = 500, description = "Internal server error", body = ()),
    )
)]
pub async fn handle_post_api_v4_network_validators_validatorsWithdrawCredentials(
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_post_api_v4_network_validators_validatorsWithdrawCredentials handler logic
    Ok(JsonResponse(()))
}





#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_example_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/example")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": "example" }));
    }
}