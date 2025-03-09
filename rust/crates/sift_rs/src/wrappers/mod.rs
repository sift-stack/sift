pub mod ingestion_configs;
pub mod runs;

/// Used to identify resources being queried
enum ResourceIdentifier {
    Id(String),
    ClientKey(String),
}
