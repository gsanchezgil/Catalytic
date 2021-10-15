use std::env;

/// Mandatory: the keyspace to use when validating queries and mapping table names to Rust structures.
/// Note: this crate can add rows! Make sure this is a test keyspace or development environment
pub const TEST_DB_KEYSPACE_KEY: &str = "TEST_DB_KEYSPACE_KEY";
/// Defaults to 127.0.0.1:9042
pub const SCYLLA_URI: &str = "SCYLLA_URI";

pub fn keyspace() -> String {
    env::var(TEST_DB_KEYSPACE_KEY).unwrap_or_else(|_| {
        panic!(
            "Add env property {} which determines the schema to use",
            TEST_DB_KEYSPACE_KEY
        )
    })
}

pub fn database_url() -> String {
    env::var(SCYLLA_URI).unwrap_or_else(|_| "127.0.0.1:9042".to_string())
}
