use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Ssv {
    openapi: String,

    paths: Paths,

    info: Info,

    tags: Vec<Option<serde_json::Value>>,

    servers: Vec<Option<serde_json::Value>>,

    components: SsvComponents,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SsvComponents {
    schemas: PurpleSchemas,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleSchemas {
    operator_metadata_dto: OperatorMetadataDto,

    dkg_health_check_dto: DkgHealthCheckDto,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DkgHealthCheckDto {
    #[serde(rename = "type")]
    dkg_health_check_dto_type: String,

    properties: DkgHealthCheckDtoProperties,

    required: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DkgHealthCheckDtoProperties {
    dkg_address: DkgAddress,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DkgAddress {
    #[serde(rename = "type")]
    dkg_address_type: Type,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Boolean,

    Number,

    String,

    #[serde(rename = "string[]")]
    TypeString,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OperatorMetadataDto {
    #[serde(rename = "type")]
    operator_metadata_dto_type: String,

    properties: OperatorMetadataDtoProperties,

    required: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorMetadataDtoProperties {
    operator_name: DkgAddress,

    description: Description,

    location: Description,

    setup_provider: Description,

    eth1_node_client: Description,

    eth2_node_client: Description,

    mev_relays: Description,

    website_url: Description,

    twitter_url: Description,

    linkedin_url: Description,

    dkg_address: Description,

    logo: Logo,

    signature: Logo,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    #[serde(rename = "type")]
    description_type: Type,

    #[serde(rename = "default")]
    description_default: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Logo {
    #[serde(rename = "type")]
    logo_type: Type,

    description: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    title: String,

    description: String,

    version: String,

    contact: Contact,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Paths {
    #[serde(rename = "/api/v4/{network}/accounts")]
    api_v4_network_accounts: ApiV4NetworkAccountsClass,

    #[serde(rename = "/api/v4/{network}/accounts/{ownerAddress}")]
    api_v4_network_accounts_owner_address: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/accounts/counts/{ownerAddress}")]
    api_v4_network_accounts_counts_owner_address: ApiV4NetworkAccountsCountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/clusters/count")]
    api_v4_network_clusters_count: ApiV4NetworkClustersCountClass,

    #[serde(rename = "/api/v4/{network}/clusters")]
    api_v4_network_clusters: ApiV4NetworkAccountsClass,

    #[serde(rename = "/api/v4/{network}/clusters/updates")]
    api_v4_network_clusters_updates: ApiV4NetworkClustersUpdates,

    #[serde(rename = "/api/v4/{network}/clusters/{id}")]
    api_v4_network_clusters_id: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/clusters/owner/{owner}/operators/{operators}")]
    api_v4_network_clusters_owner_owner_operators_operators: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/clusters/owner/{owner}")]
    api_v4_network_clusters_owner_owner: ApiV4NetworkClustersOwnerOwnerClass,

    #[serde(rename = "/api/v4/{network}/clusters/hash/{clusterHash}")]
    api_v4_network_clusters_hash_cluster_hash: ApiV4NetworkAccountsClass,

    #[serde(rename = "/api/v4/{network}/duties/{validator}")]
    api_v4_network_duties_validator: ApiV4NetworkAccountsClass,

    #[serde(rename = "/api/v4/{network}/events/{txHash}")]
    api_v4_network_events_tx_hash: ApiV4NetworkEventsTxHashClass,

    #[serde(rename = "/api/v4/{network}/faucet")]
    api_v4_network_faucet: ApiV4NetworkFaucet,

    #[serde(rename = "/api/v4/{network}/faucet/config")]
    api_v4_network_faucet_config: ApiV4NetworkClustersCountClass,

    #[serde(rename = "/api/finance/currency/convert/{symbol}/{quote}")]
    api_finance_currency_convert_symbol_quote: ApiFinanceCurrencyConvertSymbolQuote,

    #[serde(rename = "/api/v4/{network}/health")]
    api_v4_network_health: ApiV4NetworkClustersCountClass,

    #[serde(rename = "/api/v4/{network}/incentivization/merkle-tree")]
    api_v4_network_incentivization_merkle_tree: ApiV4NetworkIncentivizationMerkleTree,

    #[serde(rename = "/api/v4/{network}/operators/graph")]
    api_v4_network_operators_graph: ApiV4NetworkClustersOwnerOwnerClass,

    #[serde(rename = "/api/v4/{network}/operators/owned_by/{ownerAddress}")]
    api_v4_network_operators_owned_by_owner_address: ApiV4NetworkClustersOwnerOwnerClass,

    #[serde(rename = "/api/v4/{network}/operators/incentivized/{operator}")]
    api_v4_network_operators_incentivized_operator: ApiV4NetworkAccountsClass,

    #[serde(rename = "/api/v4/{network}/operators/{operator}")]
    api_v4_network_operators_operator: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/operators/dkg_health_check")]
    api_v4_network_operators_dkg_health_check: ApiV4NetworkOperatorsDkgHealthCheck,

    #[serde(rename = "/api/v4/{network}/operators/public_key/{public_key}")]
    api_v4_network_operators_public_key_public_key: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/operators")]
    api_v4_network_operators: ApiV4NetworkOperators,

    #[serde(rename = "/api/v4/{network}/operators/{operator}/metadata")]
    api_v4_network_operators_operator_metadata: ApiV4NetworkOperatorsOperatorMetadata,

    #[serde(rename = "/api/v4/{network}/operators/nodes/{layer}")]
    api_v4_network_operators_nodes_layer: ApiV4NetworkOperatorsNodesLayer,

    #[serde(rename = "/api/v4/{network}/operators/locations")]
    api_v4_network_operators_locations: ApiV4NetworkClustersCountClass,

    #[serde(rename = "/api/v4/{network}/search")]
    api_v4_network_search: ApiV4NetworkSearch,

    #[serde(rename = "/api/v4/{network}/validators/countActiveValidators")]
    api_v4_network_validators_count_active_validators: ApiV4NetworkEventsTxHashClass,

    #[serde(rename = "/api/v4/{network}/validators/owned_by/{ownerAddress}/cost")]
    api_v4_network_validators_owned_by_owner_address_cost: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/validators/in_operator/{operator}")]
    api_v4_network_validators_in_operator_operator: ApiV4NetworkAccountsClass,

    #[serde(rename = "/api/v4/{network}/validators/incentivized/{validator}")]
    api_v4_network_validators_incentivized_validator: ApiV4NetworkAccountsClass,

    #[serde(rename = "/api/v4/{network}/validators/{validator}")]
    api_v4_network_validators_validator: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/validators/isRegisteredValidator/{validator}")]
    api_v4_network_validators_is_registered_validator_validator: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/validators/registeredByPublicKeys")]
    api_v4_network_validators_registered_by_public_keys: ApiV4NetworkValidators,

    #[serde(rename = "/api/v4/{network}/validators")]
    api_v4_network_validators: ApiV4NetworkClustersOwnerOwnerClass,

    #[serde(rename = "/api/v4/{network}/validators/duty_counts/{from_epoch}/{to_epoch}")]
    api_v4_network_validators_duty_counts_from_epoch_to_epoch: ApiV4NetworkAccountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/validators/validatorsByClusterHash/{clusterHash}")]
    api_v4_network_validators_validators_by_cluster_hash_cluster_hash: ApiV4NetworkAccountsCountsOwnerAddressClass,

    #[serde(rename = "/api/v4/{network}/validators/validatorsWithdrawCredentials")]
    api_v4_network_validators_validators_withdraw_credentials: ApiV4NetworkValidators,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiFinanceCurrencyConvertSymbolQuote {
    get: ApiFinanceCurrencyConvertSymbolQuoteGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiFinanceCurrencyConvertSymbolQuoteGet {
    operation_id: String,

    summary: String,

    parameters: Vec<PurpleParameter>,

    responses: HashMap<String, PurpleResponse>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleParameter {
    name: String,

    required: bool,

    #[serde(rename = "in")]
    parameter_in: In,

    description: String,

    schema: PurpleSchema,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum In {
    Path,

    Query,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleSchema {
    one_of: Vec<DkgAddress>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleResponse {
    description: String,

    content: Option<ResponseContent>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseContent {
    #[serde(rename = "application/json")]
    application_json: PurpleApplicationJson,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleApplicationJson {
    schema: FluffySchema,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffySchema {
    properties: PurpleProperties,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleProperties {
    symbol: Option<DkgAddress>,

    quote: Option<DkgAddress>,

    price: Option<DkgAddress>,

    error: Option<DkgAddress>,

    message: Option<DkgAddress>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkAccountsClass {
    get: ApiV4NetworkAccountsGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkAccountsGet {
    operation_id: String,

    summary: String,

    parameters: Vec<FluffyParameter>,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyParameter {
    name: String,

    required: bool,

    #[serde(rename = "in")]
    parameter_in: In,

    schema: TentacledSchema,

    description: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledSchema {
    #[serde(rename = "type")]
    schema_type: Option<Type>,

    minimum: Option<i64>,

    #[serde(rename = "default")]
    schema_default: Option<i64>,

    maximum: Option<i64>,

    one_of: Option<Vec<DkgAddress>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct The200_Value {
    description: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkAccountsCountsOwnerAddressClass {
    parameters: Vec<ApiV4NetworkAccountsCountsOwnerAddressParameter>,

    get: ApiV4NetworkAccountsCountsOwnerAddressGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkAccountsCountsOwnerAddressGet {
    summary: String,

    tags: Vec<String>,

    responses: HashMap<String, The200_Value>,

    request_body: Option<PurpleRequestBody>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleRequestBody {
    description: String,

    required: bool,

    content: PurpleContent,

    components: RequestBodyComponents,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestBodyComponents {
    schemas: FluffySchemas,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffySchemas {
    registered_by_public_keys_dto: Option<PublicKeysDto>,

    validator_public_keys_dto: Option<PublicKeysDto>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicKeysDto {
    #[serde(rename = "type")]
    public_keys_dto_type: String,

    properties: RegisteredByPublicKeysDtoProperties,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredByPublicKeysDtoProperties {
    public_keys: DkgAddress,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleContent {
    #[serde(rename = "application/json")]
    application_json: FluffyApplicationJson,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyApplicationJson {
    schema: StickySchema,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct StickySchema {
    #[serde(rename = "type")]
    schema_type: String,

    properties: FluffyProperties,

    required: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyProperties {
    public_keys: PublicKeys,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicKeys {
    #[serde(rename = "type")]
    public_keys_type: String,

    items: DkgAddress,

    description: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkAccountsCountsOwnerAddressParameter {
    name: String,

    #[serde(rename = "in")]
    parameter_in: In,

    required: bool,

    schema: DkgAddress,

    description: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkAccountsOwnerAddressClass {
    get: ApiV4NetworkAccountsOwnerAddressGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkAccountsOwnerAddressGet {
    operation_id: String,

    summary: String,

    parameters: Vec<TentacledParameter>,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledParameter {
    name: String,

    required: bool,

    #[serde(rename = "in")]
    parameter_in: In,

    schema: IndigoSchema,

    description: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndigoSchema {
    #[serde(rename = "type")]
    schema_type: Option<Type>,

    one_of: Option<Vec<DkgAddress>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkClustersCountClass {
    get: ApiV4NetworkClustersCountGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkClustersCountGet {
    operation_id: String,

    summary: String,

    parameters: Vec<ApiV4NetworkAccountsCountsOwnerAddressParameter>,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,

    request_body: Option<FluffyRequestBody>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyRequestBody {
    required: bool,

    content: FluffyContent,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyContent {
    #[serde(rename = "application/json")]
    application_json: TentacledApplicationJson,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledApplicationJson {
    schema: IndecentSchema,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentSchema {
    #[serde(rename = "type")]
    schema_type: String,

    required: Vec<String>,

    properties: TentacledProperties,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledProperties {
    ids: Ids,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Ids {
    #[serde(rename = "type")]
    ids_type: String,

    items: DkgAddress,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkClustersOwnerOwnerClass {
    get: ApiV4NetworkClustersOwnerOwnerGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkClustersOwnerOwnerGet {
    operation_id: String,

    summary: String,

    parameters: Vec<StickyParameter>,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyParameter {
    name: String,

    required: bool,

    #[serde(rename = "in")]
    parameter_in: In,

    schema: HilariousSchema,

    description: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HilariousSchema {
    #[serde(rename = "type")]
    schema_type: Option<Type>,

    minimum: Option<i64>,

    #[serde(rename = "default")]
    schema_default: Option<DefaultUnion>,

    maximum: Option<i64>,

    one_of: Option<Vec<DkgAddress>>,

    #[serde(rename = "enum")]
    schema_enum: Option<Vec<String>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefaultUnion {
    Enum(DefaultEnum),

    Integer(i64),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultEnum {
    Both,

    #[serde(rename = "")]
    Empty,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkClustersUpdates {
    get: ApiV4NetworkClustersUpdatesGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkClustersUpdatesGet {
    operation_id: String,

    summary: String,

    parameters: Vec<IndigoParameter>,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoParameter {
    name: String,

    required: bool,

    #[serde(rename = "in")]
    parameter_in: In,

    schema: TentacledSchema,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkEventsTxHashClass {
    parameters: Vec<ApiV4NetworkAccountsCountsOwnerAddressParameter>,

    get: ApiV4NetworkEventsTxHashGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkEventsTxHashGet {
    summary: String,

    tags: Vec<String>,

    responses: HashMap<String, The200_Value>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkFaucet {
    get: ApiV4NetworkClustersCountGet,

    post: ApiV4NetworkFaucetPost,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkFaucetPost {
    operation_id: String,

    summary: String,

    parameters: Vec<ApiV4NetworkAccountsCountsOwnerAddressParameter>,

    request_body: TentacledRequestBody,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledRequestBody {
    required: bool,

    content: TentacledContent,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledContent {
    #[serde(rename = "application/json")]
    application_json: StickyApplicationJson,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyApplicationJson {
    schema: AmbitiousSchema,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbitiousSchema {
    #[serde(rename = "type")]
    schema_type: String,

    required: Vec<String>,

    properties: StickyProperties,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyProperties {
    owner_address: DkgAddress,

    #[serde(rename = "networkId")]
    network_id: DkgAddress,

    version: DkgAddress,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkIncentivizationMerkleTree {
    get: ApiV4NetworkIncentivizationMerkleTreeGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkIncentivizationMerkleTreeGet {
    operation_id: String,

    summary: String,

    parameters: Vec<ApiV4NetworkAccountsCountsOwnerAddressParameter>,

    responses: Responses,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Responses {
    #[serde(rename = "200")]
    the_200: The200_Value,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkOperators {
    get: ApiV4NetworkClustersOwnerOwnerGet,

    post: ApiV4NetworkClustersCountGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkOperatorsDkgHealthCheck {
    post: PutClass,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutClass {
    operation_id: String,

    summary: String,

    parameters: Vec<ApiV4NetworkAccountsCountsOwnerAddressParameter>,

    request_body: PutRequestBody,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PutRequestBody {
    required: bool,

    content: StickyContent,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyContent {
    #[serde(rename = "application/json")]
    application_json: IndigoApplicationJson,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct IndigoApplicationJson {
    schema: CunningSchema,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CunningSchema {
    #[serde(rename = "$ref")]
    schema_ref: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkOperatorsNodesLayer {
    get: ApiV4NetworkOperatorsNodesLayerGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkOperatorsNodesLayerGet {
    operation_id: String,

    summary: String,

    parameters: Vec<IndecentParameter>,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct IndecentParameter {
    name: String,

    required: bool,

    #[serde(rename = "in")]
    parameter_in: In,

    schema: MagentaSchema,

    description: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagentaSchema {
    #[serde(rename = "type")]
    schema_type: Option<Type>,

    one_of: Option<Vec<DkgAddress>>,

    #[serde(rename = "enum")]
    schema_enum: Option<Vec<String>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkOperatorsOperatorMetadata {
    put: PutClass,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkSearch {
    get: ApiV4NetworkSearchGet,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiV4NetworkSearchGet {
    operation_id: String,

    summary: String,

    parameters: Vec<HilariousParameter>,

    responses: HashMap<String, The200_Value>,

    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct HilariousParameter {
    name: String,

    required: bool,

    #[serde(rename = "in")]
    parameter_in: In,

    schema: HilariousSchema,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiV4NetworkValidators {
    parameters: Vec<ApiV4NetworkAccountsCountsOwnerAddressParameter>,

    post: ApiV4NetworkAccountsCountsOwnerAddressGet,
}
