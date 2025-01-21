

//
// This file is generated from openapi specification. Please do not modify it.
// It should be .gitignored
//
#![allow(warnings)]
#![allow(clippy::all)]

use serde::{Serialize, Deserialize, Deserializer};
use std::collections::BTreeMap;
use garde::Validate;
use regex::Regex;
use once_cell::sync::Lazy;






pub static UUID: Lazy<Regex> = Lazy::new(|| { Regex::new(r"^\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b$").unwrap() });
pub static FLOAT: Lazy<Regex> = Lazy::new(|| { Regex::new(r"^[0-9\.]+$").unwrap() });




#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct OperatorMetadataDto {#[garde(skip)]
    #[serde(rename = "operatorName", skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<String>,#[garde(skip)]
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,#[garde(skip)]
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,#[garde(skip)]
    #[serde(rename = "setupProvider", skip_serializing_if = "Option::is_none")]
    pub setup_provider: Option<String>,#[garde(skip)]
    #[serde(rename = "eth1NodeClient", skip_serializing_if = "Option::is_none")]
    pub eth_1_node_client: Option<String>,#[garde(skip)]
    #[serde(rename = "eth2NodeClient", skip_serializing_if = "Option::is_none")]
    pub eth_2_node_client: Option<String>,#[garde(skip)]
    #[serde(rename = "mevRelays", skip_serializing_if = "Option::is_none")]
    pub mev_relays: Option<String>,#[garde(skip)]
    #[serde(rename = "websiteUrl", skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,#[garde(skip)]
    #[serde(rename = "twitterUrl", skip_serializing_if = "Option::is_none")]
    pub twitter_url: Option<String>,#[garde(skip)]
    #[serde(rename = "linkedinUrl", skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,#[garde(skip)]
    #[serde(rename = "dkgAddress", skip_serializing_if = "Option::is_none")]
    pub dkg_address: Option<String>,#[garde(skip)]
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,#[garde(skip)]
    #[serde(rename = "signature")]
    pub signature: String,
}

impl OperatorMetadataDto { 
    pub fn new(operator_name: Option<String>, description: Option<String>, location: Option<String>, setup_provider: Option<String>, eth_1_node_client: Option<String>, eth_2_node_client: Option<String>, mev_relays: Option<String>, website_url: Option<String>, twitter_url: Option<String>, linkedin_url: Option<String>, dkg_address: Option<String>, logo: Option<String>, signature: String, ) -> Self {
        Self {
            operator_name,
            description,
            location,
            setup_provider,
            eth_1_node_client,
            eth_2_node_client,
            mev_relays,
            website_url,
            twitter_url,
            linkedin_url,
            dkg_address,
            logo,
            signature,
            
        }
    }

    
}



#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct DkgHealthCheckDto {#[garde(skip)]
    #[serde(rename = "dkgAddress")]
    pub dkg_address: String,
}

impl DkgHealthCheckDto { 
    pub fn new(dkg_address: String, ) -> Self {
        Self {
            dkg_address,
            
        }
    }

    
}



#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct ValidatorPublicKeysDtoPublicKeys {
}

impl ValidatorPublicKeysDtoPublicKeys { 
    pub fn new() -> Self {
        Self {
            
        }
    }

    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountV4ControllerByIdOwnerAddressPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerListOperatorDetailsQueryVariant {
    Bool(#[garde(skip)]
        bool
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerByIdIdPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerByIdOperatorDetailsQueryVariant {
    Bool(#[garde(skip)]
        bool
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerByOwnerAndOperatorsOwnerPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerByOwnerAndOperatorsOperatorsPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerByOwnerAndOperatorsOperatorDetailsQueryVariant {
    Bool(#[garde(skip)]
        bool
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerByOwnerOperatorDetailsQueryVariant {
    Bool(#[garde(skip)]
        bool
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerByOwnerOwnerPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClusterV4ControllerValidatorsClusterHashPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DutiesV4ControllerDutiesValidatorPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct FaucetControllerSetTransactionRequestBody {#[garde(skip)]
    #[serde(rename = "owner_address", skip_serializing_if = "Option::is_none")]
    pub owner_address: Option<String>,#[garde(skip)]
    #[serde(rename = "networkId", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<f64>,#[garde(skip)]
    #[serde(rename = "version")]
    pub version: String,
}

impl FaucetControllerSetTransactionRequestBody { 
    pub fn new(owner_address: Option<String>, network_id: Option<f64>, version: String, ) -> Self {
        Self {
            owner_address,
            network_id,
            version,
            
        }
    }

    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FinanceControllerCurrencyConvertSymbolPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FinanceControllerCurrencyConvertQuotePathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub enum OperatorsV4ControllerGraphRandomizeQueryVariant {
    
    #[serde(rename = "true")]
    True,
    
    #[serde(rename = "false")]
    False,
    ,
}

impl std::string::ToString for OperatorsV4ControllerGraphRandomizeQueryVariant {
    fn to_string(&self) -> String {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self:: => "",
        }.to_string()
    }
}

impl std::str::FromStr for OperatorsV4ControllerGraphRandomizeQueryVariant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "" => Ok(Self::),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperatorsV4ControllerOwnedByOwnerAddressPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperatorsV4ControllerIncentivizedOperatorPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperatorsV4ControllerGetOperatorOperatorPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct OperatorsV4ControllerGetDkgHealthCheckRequestBody {#[garde(skip)]
    #[serde(rename = "dkgAddress")]
    pub dkg_address: String,
}

impl OperatorsV4ControllerGetDkgHealthCheckRequestBody { 
    pub fn new(dkg_address: String, ) -> Self {
        Self {
            dkg_address,
            
        }
    }

    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperatorsV4ControllerGetByPublicKeyPublicKeyPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct OperatorsV4ControllerGetByIdsRequestBody {#[garde(skip)]
    #[serde(rename = "ids")]
    pub ids: Vec<f64>,
}

impl OperatorsV4ControllerGetByIdsRequestBody { 
    pub fn new(ids: Vec<f64>, ) -> Self {
        Self {
            ids,
            
        }
    }

    
}



#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct OperatorsV4ControllerUpdateMetadataRequestBody {#[garde(skip)]
    #[serde(rename = "operatorName", skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<String>,#[garde(skip)]
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,#[garde(skip)]
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,#[garde(skip)]
    #[serde(rename = "setupProvider", skip_serializing_if = "Option::is_none")]
    pub setup_provider: Option<String>,#[garde(skip)]
    #[serde(rename = "eth1NodeClient", skip_serializing_if = "Option::is_none")]
    pub eth_1_node_client: Option<String>,#[garde(skip)]
    #[serde(rename = "eth2NodeClient", skip_serializing_if = "Option::is_none")]
    pub eth_2_node_client: Option<String>,#[garde(skip)]
    #[serde(rename = "mevRelays", skip_serializing_if = "Option::is_none")]
    pub mev_relays: Option<String>,#[garde(skip)]
    #[serde(rename = "websiteUrl", skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,#[garde(skip)]
    #[serde(rename = "twitterUrl", skip_serializing_if = "Option::is_none")]
    pub twitter_url: Option<String>,#[garde(skip)]
    #[serde(rename = "linkedinUrl", skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,#[garde(skip)]
    #[serde(rename = "dkgAddress", skip_serializing_if = "Option::is_none")]
    pub dkg_address: Option<String>,#[garde(skip)]
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,#[garde(skip)]
    #[serde(rename = "signature")]
    pub signature: String,
}

impl OperatorsV4ControllerUpdateMetadataRequestBody { 
    pub fn new(operator_name: Option<String>, description: Option<String>, location: Option<String>, setup_provider: Option<String>, eth_1_node_client: Option<String>, eth_2_node_client: Option<String>, mev_relays: Option<String>, website_url: Option<String>, twitter_url: Option<String>, linkedin_url: Option<String>, dkg_address: Option<String>, logo: Option<String>, signature: String, ) -> Self {
        Self {
            operator_name,
            description,
            location,
            setup_provider,
            eth_1_node_client,
            eth_2_node_client,
            mev_relays,
            website_url,
            twitter_url,
            linkedin_url,
            dkg_address,
            logo,
            signature,
            
        }
    }

    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperatorsV4ControllerNodesLayerPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub enum SearchV4ControllerSearchSearchForQueryVariant {
    
    #[serde(rename = "operators")]
    Operators,
    
    #[serde(rename = "validators")]
    Validators,
    
    #[serde(rename = "both")]
    Both,
    
}

impl std::string::ToString for SearchV4ControllerSearchSearchForQueryVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Operators => "operators",
            Self::Validators => "validators",
            Self::Both => "both",
        }.to_string()
    }
}

impl std::str::FromStr for SearchV4ControllerSearchSearchForQueryVariant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "operators" => Ok(Self::Operators),
            "validators" => Ok(Self::Validators),
            "both" => Ok(Self::Both),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorsV4ControllerCostOwnerAddressPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorsV4ControllerInOperatorOperatorPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorsV4ControllerIncentivizedValidatorPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorsV4ControllerValidatorValidatorPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorsV4ControllerIsRegisteredValidatorValidatorPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorsV4ControllerDutyCountsFromEpochPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorsV4ControllerDutyCountsToEpochPathVariant {
    String(#[garde(skip)]
        String
    ),
    
}



#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct ApiV4ValidatorValidatorsWithdrawCredentialCreateRequestBody {#[garde(skip)]
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>,
}

impl ApiV4ValidatorValidatorsWithdrawCredentialCreateRequestBody { 
    pub fn new(public_keys: Vec<String>, ) -> Self {
        Self {
            public_keys,
            
        }
    }

    
}




pub fn optional_nullable<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer).map(Some)
}


