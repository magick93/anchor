
//
// This file is generated from openapi specification. Please do not modify it.
// It should be .gitignored
//
#![allow(warnings)]
#![allow(clippy::all)]

use async_trait::async_trait;
use axum::{body::Body, http::{HeaderValue, Response, StatusCode}, response::IntoResponse, Json, http};
use garde::Validate;
use serde::{
    de::{self, IntoDeserializer},
    Deserialize,
};
use serde_qs::Error;
use std::fmt;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;





#[derive(Debug)]
pub struct FailedToDeserializeQuery(Error);

impl IntoResponse for FailedToDeserializeQuery {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::BAD_REQUEST, self.0.to_string()).into_response()
    }
}

/// GET /api/v4/{network}/accounts
/// path parameters
#[derive(Validate, Deserialize)]
pub struct AccountV4ControllerListPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/accounts
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct AccountV4ControllerListQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=100)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
}
























pub enum AccountV4ControllerListResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for AccountV4ControllerListResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<AccountV4ControllerListResponse> for Response<Body> {
    fn from(response: AccountV4ControllerListResponse) -> Response<Body> {
			match response {
				
                
				AccountV4ControllerListResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				AccountV4ControllerListResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				AccountV4ControllerListResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/accounts/{ownerAddress}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct AccountV4ControllerByIdPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "ownerAddress")]
	pub owner_address: AccountV4ControllerByIdOwnerAddressPathVariant,
	
}

















pub enum AccountV4ControllerByIdResponse {
	Status200,Status500,
}

impl IntoResponse for AccountV4ControllerByIdResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<AccountV4ControllerByIdResponse> for Response<Body> {
    fn from(response: AccountV4ControllerByIdResponse) -> Response<Body> {
			match response {
				
                
				AccountV4ControllerByIdResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				AccountV4ControllerByIdResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/accounts/counts/{ownerAddress}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ApiV4AccountsCountsGetPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(skip)]
	#[serde(rename = "ownerAddress")]
	pub owner_address: String,
	
}























pub enum ApiV4AccountsCountsGetResponse {
	Status200,Status400,Status500,
}

impl IntoResponse for ApiV4AccountsCountsGetResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ApiV4AccountsCountsGetResponse> for Response<Body> {
    fn from(response: ApiV4AccountsCountsGetResponse) -> Response<Body> {
			match response {
				
                
				ApiV4AccountsCountsGetResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ApiV4AccountsCountsGetResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				ApiV4AccountsCountsGetResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}






















/// GET /api/v4/{network}/clusters/count
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerCountPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}

















pub enum ClusterV4ControllerCountResponse {
	Status200,Status500,
}

impl IntoResponse for ClusterV4ControllerCountResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ClusterV4ControllerCountResponse> for Response<Body> {
    fn from(response: ClusterV4ControllerCountResponse) -> Response<Body> {
			match response {
				
                
				ClusterV4ControllerCountResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ClusterV4ControllerCountResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/clusters
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerListPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/clusters
/// query parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerListQuery{
    #[garde(range(min=1))]
	#[serde(rename = "from")]
	pub from: f64,
	
    #[garde(range(min=1))]
	#[serde(rename = "limit")]
	pub limit: f64,
	
    #[garde(dive)]
	#[serde(rename = "operatorDetails")]
	pub operator_details: Option<super::model::ClusterV4ControllerListOperatorDetailsQueryVariant>,
	
}
























pub enum ClusterV4ControllerListResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for ClusterV4ControllerListResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ClusterV4ControllerListResponse> for Response<Body> {
    fn from(response: ClusterV4ControllerListResponse) -> Response<Body> {
			match response {
				
                
				ClusterV4ControllerListResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ClusterV4ControllerListResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				ClusterV4ControllerListResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/clusters/updates
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerUpdatesPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/clusters/updates
/// query parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerUpdatesQuery{
    #[garde(range(min=1))]
	#[serde(rename = "fromBlock")]
	pub from_block: f64,
	
}


















pub enum ClusterV4ControllerUpdatesResponse {
	Status200,Status500,
}

impl IntoResponse for ClusterV4ControllerUpdatesResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ClusterV4ControllerUpdatesResponse> for Response<Body> {
    fn from(response: ClusterV4ControllerUpdatesResponse) -> Response<Body> {
			match response {
				
                
				ClusterV4ControllerUpdatesResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ClusterV4ControllerUpdatesResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/clusters/{id}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerByIdPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "id")]
	pub id: ClusterV4ControllerByIdIdPathVariant,
	
}




/// GET /api/v4/{network}/clusters/{id}
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct ClusterV4ControllerByIdQuery{
    #[garde(dive)]
	#[serde(rename = "operatorDetails")]
	pub operator_details: Option<super::model::ClusterV4ControllerByIdOperatorDetailsQueryVariant>,
	
}


















pub enum ClusterV4ControllerByIdResponse {
	Status200,Status500,
}

impl IntoResponse for ClusterV4ControllerByIdResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ClusterV4ControllerByIdResponse> for Response<Body> {
    fn from(response: ClusterV4ControllerByIdResponse) -> Response<Body> {
			match response {
				
                
				ClusterV4ControllerByIdResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ClusterV4ControllerByIdResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/clusters/owner/{owner}/operators/{operators}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerByOwnerAndOperatorsPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "owner")]
	pub owner: ClusterV4ControllerByOwnerAndOperatorsOwnerPathVariant,
	#[garde(dive)]
	#[serde(rename = "operators")]
	pub operators: ClusterV4ControllerByOwnerAndOperatorsOperatorsPathVariant,
	
}




/// GET /api/v4/{network}/clusters/owner/{owner}/operators/{operators}
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct ClusterV4ControllerByOwnerAndOperatorsQuery{
    #[garde(dive)]
	#[serde(rename = "operatorDetails")]
	pub operator_details: Option<super::model::ClusterV4ControllerByOwnerAndOperatorsOperatorDetailsQueryVariant>,
	
}


















pub enum ClusterV4ControllerByOwnerAndOperatorsResponse {
	Status200,Status500,
}

impl IntoResponse for ClusterV4ControllerByOwnerAndOperatorsResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ClusterV4ControllerByOwnerAndOperatorsResponse> for Response<Body> {
    fn from(response: ClusterV4ControllerByOwnerAndOperatorsResponse) -> Response<Body> {
			match response {
				
                
				ClusterV4ControllerByOwnerAndOperatorsResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ClusterV4ControllerByOwnerAndOperatorsResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/clusters/owner/{owner}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerByOwnerPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "owner")]
	pub owner: ClusterV4ControllerByOwnerOwnerPathVariant,
	
}




/// GET /api/v4/{network}/clusters/owner/{owner}
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct ClusterV4ControllerByOwnerQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=100)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
    #[garde(skip)]
	#[serde(rename = "ordering")]
	pub ordering: Option<String>,
	
    #[garde(dive)]
	#[serde(rename = "operatorDetails")]
	pub operator_details: Option<super::model::ClusterV4ControllerByOwnerOperatorDetailsQueryVariant>,
	
}


















pub enum ClusterV4ControllerByOwnerResponse {
	Status200,Status500,
}

impl IntoResponse for ClusterV4ControllerByOwnerResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ClusterV4ControllerByOwnerResponse> for Response<Body> {
    fn from(response: ClusterV4ControllerByOwnerResponse) -> Response<Body> {
			match response {
				
                
				ClusterV4ControllerByOwnerResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ClusterV4ControllerByOwnerResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/clusters/hash/{clusterHash}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ClusterV4ControllerValidatorsPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "clusterHash")]
	pub cluster_hash: ClusterV4ControllerValidatorsClusterHashPathVariant,
	
}




/// GET /api/v4/{network}/clusters/hash/{clusterHash}
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct ClusterV4ControllerValidatorsQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=100)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
}


















pub enum ClusterV4ControllerValidatorsResponse {
	Status200,Status500,
}

impl IntoResponse for ClusterV4ControllerValidatorsResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ClusterV4ControllerValidatorsResponse> for Response<Body> {
    fn from(response: ClusterV4ControllerValidatorsResponse) -> Response<Body> {
			match response {
				
                
				ClusterV4ControllerValidatorsResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ClusterV4ControllerValidatorsResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}
















/// GET /api/v4/{network}/duties/{validator}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct DutiesV4ControllerDutiesPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "validator")]
	pub validator: DutiesV4ControllerDutiesValidatorPathVariant,
	
}




/// GET /api/v4/{network}/duties/{validator}
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct DutiesV4ControllerDutiesQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=100)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
}






























pub enum DutiesV4ControllerDutiesResponse {
	Status200,Status400,Status404,Status500,
}

impl IntoResponse for DutiesV4ControllerDutiesResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<DutiesV4ControllerDutiesResponse> for Response<Body> {
    fn from(response: DutiesV4ControllerDutiesResponse) -> Response<Body> {
			match response {
				
                
				DutiesV4ControllerDutiesResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				DutiesV4ControllerDutiesResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				DutiesV4ControllerDutiesResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				DutiesV4ControllerDutiesResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}




























/// GET /api/v4/{network}/events/{txHash}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ApiV4EventsGetPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(skip)]
	#[serde(rename = "txHash")]
	pub tx_hash: String,
	
}

















pub enum ApiV4EventsGetResponse {
	Status200,Status500,
}

impl IntoResponse for ApiV4EventsGetResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ApiV4EventsGetResponse> for Response<Body> {
    fn from(response: ApiV4EventsGetResponse) -> Response<Body> {
			match response {
				
                
				ApiV4EventsGetResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ApiV4EventsGetResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}
















/// GET /api/v4/{network}/faucet
/// path parameters
#[derive(Validate, Deserialize)]
pub struct FaucetControllerGetTransactionsPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}

















pub enum FaucetControllerGetTransactionsResponse {
	Status200,Status500,
}

impl IntoResponse for FaucetControllerGetTransactionsResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<FaucetControllerGetTransactionsResponse> for Response<Body> {
    fn from(response: FaucetControllerGetTransactionsResponse) -> Response<Body> {
			match response {
				
                
				FaucetControllerGetTransactionsResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				FaucetControllerGetTransactionsResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// POST /api/v4/{network}/faucet
/// path parameters
#[derive(Validate, Deserialize)]
pub struct FaucetControllerSetTransactionPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}





























pub enum FaucetControllerSetTransactionResponse {
	Status200,Status400,Status406,Status500,
}

impl IntoResponse for FaucetControllerSetTransactionResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<FaucetControllerSetTransactionResponse> for Response<Body> {
    fn from(response: FaucetControllerSetTransactionResponse) -> Response<Body> {
			match response {
				
                
				FaucetControllerSetTransactionResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				FaucetControllerSetTransactionResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				FaucetControllerSetTransactionResponse::Status406 => StatusCode::from_u16(406).unwrap().into_response(),
				
                
				FaucetControllerSetTransactionResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// GET /api/v4/{network}/faucet/config
/// path parameters
#[derive(Validate, Deserialize)]
pub struct FaucetControllerGetFaucetConfigPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}

















pub enum FaucetControllerGetFaucetConfigResponse {
	Status200,Status500,
}

impl IntoResponse for FaucetControllerGetFaucetConfigResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<FaucetControllerGetFaucetConfigResponse> for Response<Body> {
    fn from(response: FaucetControllerGetFaucetConfigResponse) -> Response<Body> {
			match response {
				
                
				FaucetControllerGetFaucetConfigResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				FaucetControllerGetFaucetConfigResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}
















/// GET /api/finance/currency/convert/{symbol}/{quote}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct FinanceControllerCurrencyConvertPath{#[garde(dive)]
	#[serde(rename = "symbol")]
	pub symbol: FinanceControllerCurrencyConvertSymbolPathVariant,
	#[garde(dive)]
	#[serde(rename = "quote")]
	pub quote: FinanceControllerCurrencyConvertQuotePathVariant,
	
}





























pub enum FinanceControllerCurrencyConvertResponse {
	Status200(serde_json::Value),Status404(serde_json::Value),Status429(serde_json::Value),Status500,
}

impl IntoResponse for FinanceControllerCurrencyConvertResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<FinanceControllerCurrencyConvertResponse> for Response<Body> {
    fn from(response: FinanceControllerCurrencyConvertResponse) -> Response<Body> {
			match response {
				
                
				FinanceControllerCurrencyConvertResponse::Status200(model) =>
                    
                    
(StatusCode::from_u16(200).unwrap(), Json(model)).into_response()

                    ,
				
                
				FinanceControllerCurrencyConvertResponse::Status404(model) =>
                    
                    
(StatusCode::from_u16(404).unwrap(), Json(model)).into_response()

                    ,
				
                
				FinanceControllerCurrencyConvertResponse::Status429(model) =>
                    
                    
(StatusCode::from_u16(429).unwrap(), Json(model)).into_response()

                    ,
				
                
				FinanceControllerCurrencyConvertResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}




























/// GET /api/v4/{network}/operators/graph
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerGraphPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/operators/graph
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct OperatorsV4ControllerGraphQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=10000)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
    #[garde(skip)]
	#[serde(rename = "randomize")]
	pub randomize: Option<super::model::OperatorsV4ControllerGraphRandomizeQueryVariant>,
	
}
























pub enum OperatorsV4ControllerGraphResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for OperatorsV4ControllerGraphResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerGraphResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerGraphResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerGraphResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerGraphResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				OperatorsV4ControllerGraphResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}






















/// GET /api/v4/{network}/health
/// path parameters
#[derive(Validate, Deserialize)]
pub struct HealthV4ControllerHealthPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}

















pub enum HealthV4ControllerHealthResponse {
	Status200,Status500,
}

impl IntoResponse for HealthV4ControllerHealthResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<HealthV4ControllerHealthResponse> for Response<Body> {
    fn from(response: HealthV4ControllerHealthResponse) -> Response<Body> {
			match response {
				
                
				HealthV4ControllerHealthResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				HealthV4ControllerHealthResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}
















/// GET /api/v4/{network}/incentivization/merkle-tree
/// path parameters
#[derive(Validate, Deserialize)]
pub struct IncentivizationV4ControllerMigrationOperatorsDistributionPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}











pub enum IncentivizationV4ControllerMigrationOperatorsDistributionResponse {
	Status200,
}

impl IntoResponse for IncentivizationV4ControllerMigrationOperatorsDistributionResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<IncentivizationV4ControllerMigrationOperatorsDistributionResponse> for Response<Body> {
    fn from(response: IncentivizationV4ControllerMigrationOperatorsDistributionResponse) -> Response<Body> {
			match response {
				
                
				IncentivizationV4ControllerMigrationOperatorsDistributionResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
			}
    }
}










/// GET /api/v4/{network}/operators/graph
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerGraphPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/operators/graph
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct OperatorsV4ControllerGraphQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=10000)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
    #[garde(skip)]
	#[serde(rename = "randomize")]
	pub randomize: Option<super::model::OperatorsV4ControllerGraphRandomizeQueryVariant>,
	
}
























pub enum OperatorsV4ControllerGraphResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for OperatorsV4ControllerGraphResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerGraphResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerGraphResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerGraphResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerGraphResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				OperatorsV4ControllerGraphResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/operators/owned_by/{ownerAddress}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerOwnedByPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "ownerAddress")]
	pub owner_address: OperatorsV4ControllerOwnedByOwnerAddressPathVariant,
	
}




/// GET /api/v4/{network}/operators/owned_by/{ownerAddress}
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct OperatorsV4ControllerOwnedByQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=100)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
    #[garde(skip)]
	#[serde(rename = "ordering")]
	pub ordering: Option<String>,
	
}


















pub enum OperatorsV4ControllerOwnedByResponse {
	Status200,Status500,
}

impl IntoResponse for OperatorsV4ControllerOwnedByResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerOwnedByResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerOwnedByResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerOwnedByResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerOwnedByResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/operators/incentivized/{operator}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerIncentivizedPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "operator")]
	pub operator: OperatorsV4ControllerIncentivizedOperatorPathVariant,
	
}




/// GET /api/v4/{network}/operators/incentivized/{operator}
/// query parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerIncentivizedQuery{
    #[garde(range(min=1))]
	#[serde(rename = "epochFrom")]
	pub epoch_from: f64,
	
    #[garde(range(min=1))]
	#[serde(rename = "epochsPerRound")]
	pub epochs_per_round: f64,
	
    #[garde(inner(range(min=1,max=5)))]
	#[serde(rename = "rounds")]
	pub rounds: Option<f64>,
	
}






























pub enum OperatorsV4ControllerIncentivizedResponse {
	Status200,Status400,Status404,Status500,
}

impl IntoResponse for OperatorsV4ControllerIncentivizedResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerIncentivizedResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerIncentivizedResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerIncentivizedResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerIncentivizedResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				OperatorsV4ControllerIncentivizedResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				OperatorsV4ControllerIncentivizedResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// GET /api/v4/{network}/operators/{operator}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerGetOperatorPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "operator")]
	pub operator: OperatorsV4ControllerGetOperatorOperatorPathVariant,
	
}





























pub enum OperatorsV4ControllerGetOperatorResponse {
	Status200,Status400,Status404,Status500,
}

impl IntoResponse for OperatorsV4ControllerGetOperatorResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerGetOperatorResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerGetOperatorResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerGetOperatorResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetOperatorResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetOperatorResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetOperatorResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// POST /api/v4/{network}/operators/dkg_health_check
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerGetDkgHealthCheckPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}























pub enum OperatorsV4ControllerGetDkgHealthCheckResponse {
	Status200,Status201,Status500,
}

impl IntoResponse for OperatorsV4ControllerGetDkgHealthCheckResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerGetDkgHealthCheckResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerGetDkgHealthCheckResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerGetDkgHealthCheckResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetDkgHealthCheckResponse::Status201 => StatusCode::from_u16(201).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetDkgHealthCheckResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/operators/public_key/{public_key}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerGetByPublicKeyPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "public_key")]
	pub public_key: OperatorsV4ControllerGetByPublicKeyPublicKeyPathVariant,
	
}





























pub enum OperatorsV4ControllerGetByPublicKeyResponse {
	Status200,Status400,Status404,Status500,
}

impl IntoResponse for OperatorsV4ControllerGetByPublicKeyResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerGetByPublicKeyResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerGetByPublicKeyResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerGetByPublicKeyResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetByPublicKeyResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetByPublicKeyResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetByPublicKeyResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// GET /api/v4/{network}/operators
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerOperatorsPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/operators
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct OperatorsV4ControllerOperatorsQuery{
    #[garde(skip)]
	#[serde(rename = "type")]
	pub type_: Option<String>,
	
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=5000)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
    #[garde(skip)]
	#[serde(rename = "ordering")]
	pub ordering: Option<String>,
	
    #[garde(skip)]
	#[serde(rename = "search")]
	pub search: Option<String>,
	
    #[garde(skip)]
	#[serde(rename = "has_dkg_address")]
	pub has_dkg_address: Option<bool>,
	
}
























pub enum OperatorsV4ControllerOperatorsResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for OperatorsV4ControllerOperatorsResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerOperatorsResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerOperatorsResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerOperatorsResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerOperatorsResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				OperatorsV4ControllerOperatorsResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// POST /api/v4/{network}/operators
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerGetByIdsPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}

















pub enum OperatorsV4ControllerGetByIdsResponse {
	Status200,Status500,
}

impl IntoResponse for OperatorsV4ControllerGetByIdsResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerGetByIdsResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerGetByIdsResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerGetByIdsResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerGetByIdsResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// PUT /api/v4/{network}/operators/{operator}/metadata
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerUpdateMetadataPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(skip)]
	#[serde(rename = "operator")]
	pub operator: String,
	
}





























pub enum OperatorsV4ControllerUpdateMetadataResponse {
	Status200,Status401,Status404,Status500,
}

impl IntoResponse for OperatorsV4ControllerUpdateMetadataResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerUpdateMetadataResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerUpdateMetadataResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerUpdateMetadataResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerUpdateMetadataResponse::Status401 => StatusCode::from_u16(401).unwrap().into_response(),
				
                
				OperatorsV4ControllerUpdateMetadataResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				OperatorsV4ControllerUpdateMetadataResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// GET /api/v4/{network}/operators/nodes/{layer}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerNodesPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "layer")]
	pub layer: OperatorsV4ControllerNodesLayerPathVariant,
	
}























pub enum OperatorsV4ControllerNodesResponse {
	Status200,Status400,Status500,
}

impl IntoResponse for OperatorsV4ControllerNodesResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerNodesResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerNodesResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerNodesResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerNodesResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				OperatorsV4ControllerNodesResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/operators/locations
/// path parameters
#[derive(Validate, Deserialize)]
pub struct OperatorsV4ControllerLocationsPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}

















pub enum OperatorsV4ControllerLocationsResponse {
	Status200,Status500,
}

impl IntoResponse for OperatorsV4ControllerLocationsResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<OperatorsV4ControllerLocationsResponse> for Response<Body> {
    fn from(response: OperatorsV4ControllerLocationsResponse) -> Response<Body> {
			match response {
				
                
				OperatorsV4ControllerLocationsResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				OperatorsV4ControllerLocationsResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}
















/// GET /api/v4/{network}/search
/// path parameters
#[derive(Validate, Deserialize)]
pub struct SearchV4ControllerSearchPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/search
/// query parameters
#[derive(Validate, Deserialize)]
pub struct SearchV4ControllerSearchQuery{
    #[garde(skip)]
	#[serde(rename = "search")]
	pub search: String,
	
    #[garde(skip)]
	#[serde(rename = "searchFor")]
	pub search_for: Option<super::model::SearchV4ControllerSearchSearchForQueryVariant>,
	
    #[garde(inner(range(min=0)))]
	#[serde(rename = "operatorsLimit")]
	pub operators_limit: Option<f64>,
	
    #[garde(inner(range(min=0)))]
	#[serde(rename = "validatorsLimit")]
	pub validators_limit: Option<f64>,
	
}
























pub enum SearchV4ControllerSearchResponse {
	Status200,Status400,Status500,
}

impl IntoResponse for SearchV4ControllerSearchResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<SearchV4ControllerSearchResponse> for Response<Body> {
    fn from(response: SearchV4ControllerSearchResponse) -> Response<Body> {
			match response {
				
                
				SearchV4ControllerSearchResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				SearchV4ControllerSearchResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				SearchV4ControllerSearchResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}






















/// GET /api/v4/{network}/validators/countActiveValidators
/// path parameters
#[derive(Validate, Deserialize)]
pub struct CountActiveValidatorsInNetworkPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}

















pub enum CountActiveValidatorsInNetworkResponse {
	Status200,Status500,
}

impl IntoResponse for CountActiveValidatorsInNetworkResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<CountActiveValidatorsInNetworkResponse> for Response<Body> {
    fn from(response: CountActiveValidatorsInNetworkResponse) -> Response<Body> {
			match response {
				
                
				CountActiveValidatorsInNetworkResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				CountActiveValidatorsInNetworkResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/validators/owned_by/{ownerAddress}/cost
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ValidatorsV4ControllerCostPath{#[garde(dive)]
	#[serde(rename = "ownerAddress")]
	pub owner_address: ValidatorsV4ControllerCostOwnerAddressPathVariant,
	#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}























pub enum ValidatorsV4ControllerCostResponse {
	Status200,Status400,Status500,
}

impl IntoResponse for ValidatorsV4ControllerCostResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ValidatorsV4ControllerCostResponse> for Response<Body> {
    fn from(response: ValidatorsV4ControllerCostResponse) -> Response<Body> {
			match response {
				
                
				ValidatorsV4ControllerCostResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ValidatorsV4ControllerCostResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				ValidatorsV4ControllerCostResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/validators/in_operator/{operator}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ValidatorsV4ControllerInOperatorPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "operator")]
	pub operator: ValidatorsV4ControllerInOperatorOperatorPathVariant,
	
}




/// GET /api/v4/{network}/validators/in_operator/{operator}
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct ValidatorsV4ControllerInOperatorQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=100)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
}
























pub enum ValidatorsV4ControllerInOperatorResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for ValidatorsV4ControllerInOperatorResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ValidatorsV4ControllerInOperatorResponse> for Response<Body> {
    fn from(response: ValidatorsV4ControllerInOperatorResponse) -> Response<Body> {
			match response {
				
                
				ValidatorsV4ControllerInOperatorResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ValidatorsV4ControllerInOperatorResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				ValidatorsV4ControllerInOperatorResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/validators/incentivized/{validator}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ValidatorsV4ControllerIncentivizedPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "validator")]
	pub validator: ValidatorsV4ControllerIncentivizedValidatorPathVariant,
	
}




/// GET /api/v4/{network}/validators/incentivized/{validator}
/// query parameters
#[derive(Validate, Deserialize)]
pub struct ValidatorsV4ControllerIncentivizedQuery{
    #[garde(range(min=1))]
	#[serde(rename = "epochFrom")]
	pub epoch_from: f64,
	
    #[garde(range(min=1))]
	#[serde(rename = "epochsPerRound")]
	pub epochs_per_round: f64,
	
    #[garde(inner(range(min=1,max=5)))]
	#[serde(rename = "rounds")]
	pub rounds: Option<f64>,
	
}






























pub enum ValidatorsV4ControllerIncentivizedResponse {
	Status200,Status400,Status404,Status500,
}

impl IntoResponse for ValidatorsV4ControllerIncentivizedResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ValidatorsV4ControllerIncentivizedResponse> for Response<Body> {
    fn from(response: ValidatorsV4ControllerIncentivizedResponse) -> Response<Body> {
			match response {
				
                
				ValidatorsV4ControllerIncentivizedResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ValidatorsV4ControllerIncentivizedResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				ValidatorsV4ControllerIncentivizedResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				ValidatorsV4ControllerIncentivizedResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// GET /api/v4/{network}/validators/{validator}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ValidatorsV4ControllerValidatorPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "validator")]
	pub validator: ValidatorsV4ControllerValidatorValidatorPathVariant,
	
}





























pub enum ValidatorsV4ControllerValidatorResponse {
	Status200,Status400,Status404,Status500,
}

impl IntoResponse for ValidatorsV4ControllerValidatorResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ValidatorsV4ControllerValidatorResponse> for Response<Body> {
    fn from(response: ValidatorsV4ControllerValidatorResponse) -> Response<Body> {
			match response {
				
                
				ValidatorsV4ControllerValidatorResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ValidatorsV4ControllerValidatorResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				ValidatorsV4ControllerValidatorResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				ValidatorsV4ControllerValidatorResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// GET /api/v4/{network}/validators/isRegisteredValidator/{validator}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ValidatorsV4ControllerIsRegisteredValidatorPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "validator")]
	pub validator: ValidatorsV4ControllerIsRegisteredValidatorValidatorPathVariant,
	
}





























pub enum ValidatorsV4ControllerIsRegisteredValidatorResponse {
	Status200,Status400,Status404,Status500,
}

impl IntoResponse for ValidatorsV4ControllerIsRegisteredValidatorResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ValidatorsV4ControllerIsRegisteredValidatorResponse> for Response<Body> {
    fn from(response: ValidatorsV4ControllerIsRegisteredValidatorResponse) -> Response<Body> {
			match response {
				
                
				ValidatorsV4ControllerIsRegisteredValidatorResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ValidatorsV4ControllerIsRegisteredValidatorResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				ValidatorsV4ControllerIsRegisteredValidatorResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				ValidatorsV4ControllerIsRegisteredValidatorResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// POST /api/v4/{network}/validators/registeredByPublicKeys
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ApiV4ValidatorRegisteredByPublicKeyCreatePath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}























pub enum ApiV4ValidatorRegisteredByPublicKeyCreateResponse {
	Status200,Status400,Status500,
}

impl IntoResponse for ApiV4ValidatorRegisteredByPublicKeyCreateResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ApiV4ValidatorRegisteredByPublicKeyCreateResponse> for Response<Body> {
    fn from(response: ApiV4ValidatorRegisteredByPublicKeyCreateResponse) -> Response<Body> {
			match response {
				
                
				ApiV4ValidatorRegisteredByPublicKeyCreateResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ApiV4ValidatorRegisteredByPublicKeyCreateResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				ApiV4ValidatorRegisteredByPublicKeyCreateResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/validators
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ValidatorsV4ControllerValidatorsPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/validators
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct ValidatorsV4ControllerValidatorsQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "lastId")]
	pub last_id: Option<f64>,
	
    #[garde(skip)]
	#[serde(rename = "pageDirection")]
	pub page_direction: Option<String>,
	
    #[garde(inner(range(min=1,max=1000)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
    #[garde(skip)]
	#[serde(rename = "ordering")]
	pub ordering: Option<String>,
	
    #[garde(skip)]
	#[serde(rename = "ownerAddress")]
	pub owner_address: Option<String>,
	
    #[garde(skip)]
	#[serde(rename = "search")]
	pub search: Option<String>,
	
}
























pub enum ValidatorsV4ControllerValidatorsResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for ValidatorsV4ControllerValidatorsResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ValidatorsV4ControllerValidatorsResponse> for Response<Body> {
    fn from(response: ValidatorsV4ControllerValidatorsResponse) -> Response<Body> {
			match response {
				
                
				ValidatorsV4ControllerValidatorsResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ValidatorsV4ControllerValidatorsResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				ValidatorsV4ControllerValidatorsResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/validators/duty_counts/{from_epoch}/{to_epoch}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ValidatorsV4ControllerDutyCountsPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "from_epoch")]
	pub from_epoch: ValidatorsV4ControllerDutyCountsFromEpochPathVariant,
	#[garde(dive)]
	#[serde(rename = "to_epoch")]
	pub to_epoch: ValidatorsV4ControllerDutyCountsToEpochPathVariant,
	
}























pub enum ValidatorsV4ControllerDutyCountsResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for ValidatorsV4ControllerDutyCountsResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ValidatorsV4ControllerDutyCountsResponse> for Response<Body> {
    fn from(response: ValidatorsV4ControllerDutyCountsResponse) -> Response<Body> {
			match response {
				
                
				ValidatorsV4ControllerDutyCountsResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ValidatorsV4ControllerDutyCountsResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				ValidatorsV4ControllerDutyCountsResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/validators/validatorsByClusterHash/{clusterHash}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ApiV4ValidatorsValidatorsByClusterHashGetPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(skip)]
	#[serde(rename = "clusterHash")]
	pub cluster_hash: String,
	
}























pub enum ApiV4ValidatorsValidatorsByClusterHashGetResponse {
	Status200,Status400,Status500,
}

impl IntoResponse for ApiV4ValidatorsValidatorsByClusterHashGetResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ApiV4ValidatorsValidatorsByClusterHashGetResponse> for Response<Body> {
    fn from(response: ApiV4ValidatorsValidatorsByClusterHashGetResponse) -> Response<Body> {
			match response {
				
                
				ApiV4ValidatorsValidatorsByClusterHashGetResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ApiV4ValidatorsValidatorsByClusterHashGetResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				ApiV4ValidatorsValidatorsByClusterHashGetResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// POST /api/v4/{network}/validators/validatorsWithdrawCredentials
/// path parameters
#[derive(Validate, Deserialize)]
pub struct ApiV4ValidatorValidatorsWithdrawCredentialCreatePath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}























pub enum ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse {
	Status200,Status400,Status500,
}

impl IntoResponse for ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse> for Response<Body> {
    fn from(response: ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse) -> Response<Body> {
			match response {
				
                
				ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}






















/// GET /api/v4/{network}/accounts
/// path parameters
#[derive(Validate, Deserialize)]
pub struct AccountV4ControllerListPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/accounts
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct AccountV4ControllerListQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=100)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
}
























pub enum AccountV4ControllerListResponse {
	Status200,Status404,Status500,
}

impl IntoResponse for AccountV4ControllerListResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<AccountV4ControllerListResponse> for Response<Body> {
    fn from(response: AccountV4ControllerListResponse) -> Response<Body> {
			match response {
				
                
				AccountV4ControllerListResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				AccountV4ControllerListResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				AccountV4ControllerListResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}





















/// GET /api/v4/{network}/accounts/{ownerAddress}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct AccountV4ControllerByIdPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "ownerAddress")]
	pub owner_address: AccountV4ControllerByIdOwnerAddressPathVariant,
	
}

















pub enum AccountV4ControllerByIdResponse {
	Status200,Status500,
}

impl IntoResponse for AccountV4ControllerByIdResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<AccountV4ControllerByIdResponse> for Response<Body> {
    fn from(response: AccountV4ControllerByIdResponse) -> Response<Body> {
			match response {
				
                
				AccountV4ControllerByIdResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				AccountV4ControllerByIdResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/duties/{validator}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct DutiesV4ControllerDutiesPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	#[garde(dive)]
	#[serde(rename = "validator")]
	pub validator: DutiesV4ControllerDutiesValidatorPathVariant,
	
}




/// GET /api/v4/{network}/duties/{validator}
/// query parameters
#[derive(Validate, Deserialize,Default)]
pub struct DutiesV4ControllerDutiesQuery{
    #[garde(inner(range(min=1)))]
	#[serde(rename = "page")]
	pub page: Option<f64>,
	
    #[garde(inner(range(min=1,max=100)))]
	#[serde(rename = "perPage")]
	pub per_page: Option<f64>,
	
}






























pub enum DutiesV4ControllerDutiesResponse {
	Status200,Status400,Status404,Status500,
}

impl IntoResponse for DutiesV4ControllerDutiesResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<DutiesV4ControllerDutiesResponse> for Response<Body> {
    fn from(response: DutiesV4ControllerDutiesResponse) -> Response<Body> {
			match response {
				
                
				DutiesV4ControllerDutiesResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				DutiesV4ControllerDutiesResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				DutiesV4ControllerDutiesResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
				
                
				DutiesV4ControllerDutiesResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}



























/// GET /api/v4/{network}/health
/// path parameters
#[derive(Validate, Deserialize)]
pub struct HealthV4ControllerHealthPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}

















pub enum HealthV4ControllerHealthResponse {
	Status200,Status500,
}

impl IntoResponse for HealthV4ControllerHealthResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<HealthV4ControllerHealthResponse> for Response<Body> {
    fn from(response: HealthV4ControllerHealthResponse) -> Response<Body> {
			match response {
				
                
				HealthV4ControllerHealthResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				HealthV4ControllerHealthResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}















/// GET /api/v4/{network}/incentivization/merkle-tree
/// path parameters
#[derive(Validate, Deserialize)]
pub struct IncentivizationV4ControllerMigrationOperatorsDistributionPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}











pub enum IncentivizationV4ControllerMigrationOperatorsDistributionResponse {
	Status200,
}

impl IntoResponse for IncentivizationV4ControllerMigrationOperatorsDistributionResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<IncentivizationV4ControllerMigrationOperatorsDistributionResponse> for Response<Body> {
    fn from(response: IncentivizationV4ControllerMigrationOperatorsDistributionResponse) -> Response<Body> {
			match response {
				
                
				IncentivizationV4ControllerMigrationOperatorsDistributionResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
			}
    }
}









/// GET /api/v4/{network}/search
/// path parameters
#[derive(Validate, Deserialize)]
pub struct SearchV4ControllerSearchPath{#[garde(skip)]
	#[serde(rename = "network")]
	pub network: String,
	
}




/// GET /api/v4/{network}/search
/// query parameters
#[derive(Validate, Deserialize)]
pub struct SearchV4ControllerSearchQuery{
    #[garde(skip)]
	#[serde(rename = "search")]
	pub search: String,
	
    #[garde(skip)]
	#[serde(rename = "searchFor")]
	pub search_for: Option<super::model::SearchV4ControllerSearchSearchForQueryVariant>,
	
    #[garde(inner(range(min=0)))]
	#[serde(rename = "operatorsLimit")]
	pub operators_limit: Option<f64>,
	
    #[garde(inner(range(min=0)))]
	#[serde(rename = "validatorsLimit")]
	pub validators_limit: Option<f64>,
	
}
























pub enum SearchV4ControllerSearchResponse {
	Status200,Status400,Status500,
}

impl IntoResponse for SearchV4ControllerSearchResponse {
    fn into_response(self) -> axum::response::Response {
        self.into()
    }
}

impl From<SearchV4ControllerSearchResponse> for Response<Body> {
    fn from(response: SearchV4ControllerSearchResponse) -> Response<Body> {
			match response {
				
                
				SearchV4ControllerSearchResponse::Status200 => StatusCode::from_u16(200).unwrap().into_response(),
				
                
				SearchV4ControllerSearchResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
				
                
				SearchV4ControllerSearchResponse::Status500 => StatusCode::from_u16(500).unwrap().into_response(),
				
			}
    }
}


























/// all convertable Result are acceptable
macro_rules! impl_from_result {
    ($t:ident) => {
        impl<O, E> From<Result<O, E>> for $t
        where
            O: Into<$t>,
            E: Into<$t>,
        {
            fn from(res: Result<O, E>) -> Self {
                match res {
                    Ok(r) => r.into(),
                    Err(e) => e.into(),
                }
            }
        }
    };
}


impl_from_result!(AccountV4ControllerListResponse);
impl_from_result!(AccountV4ControllerByIdResponse);
impl_from_result!(ApiV4AccountsCountsGetResponse);
impl_from_result!(ClusterV4ControllerCountResponse);
impl_from_result!(ClusterV4ControllerListResponse);
impl_from_result!(ClusterV4ControllerUpdatesResponse);
impl_from_result!(ClusterV4ControllerByIdResponse);
impl_from_result!(ClusterV4ControllerByOwnerAndOperatorsResponse);
impl_from_result!(ClusterV4ControllerByOwnerResponse);
impl_from_result!(ClusterV4ControllerValidatorsResponse);
impl_from_result!(DutiesV4ControllerDutiesResponse);
impl_from_result!(ApiV4EventsGetResponse);
impl_from_result!(FaucetControllerGetTransactionsResponse);
impl_from_result!(FaucetControllerSetTransactionResponse);
impl_from_result!(FaucetControllerGetFaucetConfigResponse);
impl_from_result!(FinanceControllerCurrencyConvertResponse);
impl_from_result!(OperatorsV4ControllerGraphResponse);
impl_from_result!(HealthV4ControllerHealthResponse);
impl_from_result!(IncentivizationV4ControllerMigrationOperatorsDistributionResponse);
impl_from_result!(OperatorsV4ControllerGraphResponse);
impl_from_result!(OperatorsV4ControllerOwnedByResponse);
impl_from_result!(OperatorsV4ControllerIncentivizedResponse);
impl_from_result!(OperatorsV4ControllerGetOperatorResponse);
impl_from_result!(OperatorsV4ControllerGetDkgHealthCheckResponse);
impl_from_result!(OperatorsV4ControllerGetByPublicKeyResponse);
impl_from_result!(OperatorsV4ControllerOperatorsResponse);
impl_from_result!(OperatorsV4ControllerGetByIdsResponse);
impl_from_result!(OperatorsV4ControllerUpdateMetadataResponse);
impl_from_result!(OperatorsV4ControllerNodesResponse);
impl_from_result!(OperatorsV4ControllerLocationsResponse);
impl_from_result!(SearchV4ControllerSearchResponse);
impl_from_result!(CountActiveValidatorsInNetworkResponse);
impl_from_result!(ValidatorsV4ControllerCostResponse);
impl_from_result!(ValidatorsV4ControllerInOperatorResponse);
impl_from_result!(ValidatorsV4ControllerIncentivizedResponse);
impl_from_result!(ValidatorsV4ControllerValidatorResponse);
impl_from_result!(ValidatorsV4ControllerIsRegisteredValidatorResponse);
impl_from_result!(ApiV4ValidatorRegisteredByPublicKeyCreateResponse);
impl_from_result!(ValidatorsV4ControllerValidatorsResponse);
impl_from_result!(ValidatorsV4ControllerDutyCountsResponse);
impl_from_result!(ApiV4ValidatorsValidatorsByClusterHashGetResponse);
impl_from_result!(ApiV4ValidatorValidatorsWithdrawCredentialCreateResponse);
impl_from_result!(AccountV4ControllerListResponse);
impl_from_result!(AccountV4ControllerByIdResponse);
impl_from_result!(DutiesV4ControllerDutiesResponse);
impl_from_result!(HealthV4ControllerHealthResponse);
impl_from_result!(IncentivizationV4ControllerMigrationOperatorsDistributionResponse);
impl_from_result!(SearchV4ControllerSearchResponse);

/// custom deserializer for query parameters /users?id=3,4,5
/// style = form, explode = false according to https://swagger.io/docs/specification/serialization/
pub fn deserialized_id_list<'de, D, I>(deserializer: D) -> std::result::Result<Vec<I>, D::Error>
where
    D: de::Deserializer<'de>,
    I: de::DeserializeOwned,
{
    struct StringVecVisitor<I>(std::marker::PhantomData<I>);

    impl<'de, I> de::Visitor<'de> for StringVecVisitor<I>
    where I: de::DeserializeOwned {
        type Value = Vec<I>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a string containing a list")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut ids = Vec::new();
            for id in v.split(',') {
                let id = I::deserialize(id.into_deserializer())?;
                ids.push(id);
            }
            Ok(ids)
        }
    }

    deserializer.deserialize_any(StringVecVisitor(std::marker::PhantomData::<I>))
}

pub fn deserialized_opt_id_list<'de, D, I>(deserializer: D) -> std::result::Result<Option<Vec<I>>, D::Error>
where
    D: de::Deserializer<'de>,
    I: de::DeserializeOwned,
{
    struct StringVecVisitor<I>(std::marker::PhantomData<I>);

    impl<'de, I> de::Visitor<'de> for StringVecVisitor<I>
    where I: de::DeserializeOwned {
        type Value = Option<Vec<I>>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a string containing a list")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut ids = Vec::new();
            for id in v.split(',') {
                let id = I::deserialize(id.into_deserializer())?;
                ids.push(id);
            }
            Ok(Some(ids))
        }

        fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(StringVecVisitor(std::marker::PhantomData::<I>))
}
