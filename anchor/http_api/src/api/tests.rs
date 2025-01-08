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
    async fn test_handle_get_api_v4_network_accounts() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/accounts")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_accounts_ownerAddress() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/accounts/{ownerAddress}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_accounts_counts_ownerAddress() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/accounts/counts/{ownerAddress}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_clusters_count() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/clusters/count")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_clusters() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/clusters")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_clusters_updates() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/clusters/updates")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_clusters_id() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/clusters/{id}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_clusters_owner_owner_operators_operators() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/clusters/owner/{owner}/operators/{operators}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_clusters_owner_owner() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/clusters/owner/{owner}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_clusters_hash_clusterHash() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/clusters/hash/{clusterHash}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_duties_validator() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/duties/{validator}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_events_txHash() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/events/{txHash}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_faucet() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/faucet")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_post_api_v4_network_faucet() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/faucet")
                    .method("POST")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_faucet_config() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/faucet/config")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_finance_currency_convert_symbol_quote() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/finance/currency/convert/{symbol}/{quote}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_health() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/health")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_incentivization_merkle_tree() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/incentivization/merkle-tree")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_operators_graph() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/graph")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_operators_owned_by_ownerAddress() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/owned_by/{ownerAddress}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_operators_incentivized_operator() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/incentivized/{operator}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_operators_operator() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/{operator}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_post_api_v4_network_operators_dkg_health_check() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/dkg_health_check")
                    .method("POST")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_operators_public_key_public_key() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/public_key/{public_key}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_operators() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_post_api_v4_network_operators() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators")
                    .method("POST")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_put_api_v4_network_operators_operator_metadata() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/{operator}/metadata")
                    .method("PUT")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_operators_nodes_layer() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/nodes/{layer}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_operators_locations() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/operators/locations")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_search() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/search")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators_countActiveValidators() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/countActiveValidators")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators_owned_by_ownerAddress_cost() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/owned_by/{ownerAddress}/cost")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators_in_operator_operator() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/in_operator/{operator}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators_incentivized_validator() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/incentivized/{validator}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators_validator() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/{validator}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators_isRegisteredValidator_validator() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/isRegisteredValidator/{validator}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_post_api_v4_network_validators_registeredByPublicKeys() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/registeredByPublicKeys")
                    .method("POST")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators_duty_counts_from_epoch_to_epoch() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/duty_counts/{from_epoch}/{to_epoch}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_get_api_v4_network_validators_validatorsByClusterHash_clusterHash() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/validatorsByClusterHash/{clusterHash}")
                    .method("GET")
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
    
    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_post_api_v4_network_validators_validatorsWithdrawCredentials() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v4/{network}/validators/validatorsWithdrawCredentials")
                    .method("POST")
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