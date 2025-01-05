#[cfg(test)]
use super::*;

// Server starts successfully when enabled with valid config
#[tokio::test]
async fn server_starts_with_valid_config() {
    let config = Config {
        allow_origin: Some("*".to_string()),
        ..Default::default()
    };

    let result = run(config).await;

    assert!(result.is_ok());
}

// // Server fails to bind to already in-use port
#[tokio::test]
// #[ignore]
async fn server_fails_on_port_conflict() {
    let port = 8080;
    let addr = "127.0.0.1".parse().unwrap();

    // Create a listener to occupy the port
    let _existing = TcpListener::bind((addr, port))
        .await
        .expect("Failed to bind first listener");

    let config = Config {
        allow_origin: Some("*".to_string()),
        enabled: true,
        listen_addr: addr,
        listen_port: port,
    };

    let result = run(config).await;

    let err_string = result.unwrap_err().to_string();

    assert!(
        err_string.contains("in use") || err_string.contains("EADDRINUSE"),
        "Expected error about port conflict, got: {}",
        err_string
    );
}
