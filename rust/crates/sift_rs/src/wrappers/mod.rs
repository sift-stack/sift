/// Offers a wrapper over Sift's assets API.
pub mod assets;

/// Offers a wrapper over Sift's ingestion configs API.
pub mod ingestion_configs;

/// Offers a wrapper over Sift's metadata API.
pub mod metadata;

/// Offers a wrapper over Sift's runs API.
pub mod runs;

/// Used to identify resources being queried
enum ResourceIdentifier {
    Id(String),
    ClientKey(String),
}
