use sift_rs::{
    Credentials, RetryConfig, RetryExt, SiftChannelBuilder,
    wrappers::{
        assets::{AssetServiceWrapper, new_asset_service},
        ingestion_configs::{IngestionConfigServiceWrapper, new_ingestion_config_service},
    },
};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::Config {
        apikey: env::var("SIFT_API_KEY").unwrap(),
        uri: env::var("SIFT_URI").unwrap(),
    };

    let conn = SiftChannelBuilder::new(credentials).build()?;

    // Example 1: Using default retry configuration
    println!("Example 1: Using default retry configuration");
    example_default_retry(&conn).await?;

    // Example 2: Using custom retry configuration
    println!("\nExample 2: Using custom retry configuration");
    example_custom_retry(&conn).await?;

    // Example 3: Using retry with ingestion config service
    println!("\nExample 3: Using retry with ingestion config service");
    example_ingestion_config_retry(&conn).await?;

    Ok(())
}

/// Example demonstrating retry with default configuration.
///
/// The default configuration provides:
/// - 3 total attempts
/// - 100ms base delay
/// - 5s maximum delay
/// - 2.0 backoff multiplier (exponential)
async fn example_default_retry(
    conn: &sift_connect::SiftChannel,
) -> Result<(), Box<dyn std::error::Error>> {
    let asset_service = new_asset_service(conn.clone());

    // Wrap the service with default retry configuration
    let retrying_service = asset_service.retrying(RetryConfig::default());

    // Use the retrying service with the closure pattern
    // The closure receives a cloned wrapper, allowing &mut self methods to work
    let result = retrying_service
        .call(|mut wrapper| async move {
            // This will automatically retry on transient gRPC errors like:
            // - Unavailable
            // - ResourceExhausted
            // - DeadlineExceeded
            wrapper.try_get_asset_by_id("example-asset-id").await
        })
        .await;

    match result {
        Ok(asset) => {
            println!("Successfully retrieved asset: {}", asset.name);
        }
        Err(e) => {
            println!("Failed to retrieve asset after retries: {}", e);
        }
    }

    Ok(())
}

/// Example demonstrating retry with custom configuration.
///
/// Custom configuration allows you to tune retry behavior for your specific use case:
/// - More aggressive retries (more attempts, shorter delays)
/// - More conservative retries (fewer attempts, longer delays)
/// - Different backoff strategies
async fn example_custom_retry(
    conn: &sift_connect::SiftChannel,
) -> Result<(), Box<dyn std::error::Error>> {
    let asset_service = new_asset_service(conn.clone());

    // Create a custom retry configuration
    let custom_config = RetryConfig {
        max_attempts: 5,                        // Try up to 5 times
        base_delay: Duration::from_millis(200), // Start with 200ms delay
        max_delay: Duration::from_secs(10),     // Cap at 10 seconds
        backoff_multiplier: 1.5,                // Linear-ish backoff (1.5x)
    };

    let retrying_service = asset_service.retrying(custom_config);

    // Example: Update an asset with retry logic
    // Note: Only use retries for idempotent operations!
    let result = retrying_service
        .call(|mut wrapper| async move {
            // First, get an asset to update
            let asset = wrapper.try_get_asset_by_id("example-asset-id").await?;

            // Then update it (this is idempotent if the update is the same)
            wrapper
                .try_update_asset(asset, vec!["display_name".to_string()])
                .await
        })
        .await;

    match result {
        Ok(asset) => {
            println!("Successfully updated asset: {}", asset.name);
        }
        Err(e) => {
            println!("Failed to update asset after retries: {}", e);
        }
    }

    Ok(())
}

/// Example demonstrating retry with ingestion config service.
///
/// Shows that the retry mechanism works with any wrapper service that implements Clone.
async fn example_ingestion_config_retry(
    conn: &sift_connect::SiftChannel,
) -> Result<(), Box<dyn std::error::Error>> {
    let ingestion_config_service = new_ingestion_config_service(conn.clone());

    // Use a moderate retry configuration for ingestion config operations
    let retry_config = RetryConfig {
        max_attempts: 4,
        base_delay: Duration::from_millis(150),
        max_delay: Duration::from_secs(3),
        backoff_multiplier: 2.0,
    };

    let retrying_service = ingestion_config_service.retrying(retry_config);

    // Retrieve an ingestion config by ID with automatic retry
    let result = retrying_service
        .call(|mut wrapper| async move {
            wrapper
                .try_get_ingestion_config_by_id("example-config-id")
                .await
        })
        .await;

    match result {
        Ok(config) => {
            println!(
                "Successfully retrieved ingestion config: {}",
                config.client_key
            );
        }
        Err(e) => {
            println!("Failed to retrieve ingestion config after retries: {}", e);
        }
    }

    // Example: Retrieve by client key with retry
    let result = retrying_service
        .call(|mut wrapper| async move {
            wrapper
                .try_get_ingestion_config_by_client_key("example-client-key")
                .await
        })
        .await;

    match result {
        Ok(config) => {
            println!(
                "Successfully retrieved ingestion config by client key: {}",
                config.client_key
            );
        }
        Err(e) => {
            println!(
                "Failed to retrieve ingestion config by client key after retries: {}",
                e
            );
        }
    }

    Ok(())
}
