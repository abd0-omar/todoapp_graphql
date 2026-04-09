use async_trait::async_trait;
use std::fmt;

#[derive(Debug)]
pub enum StoreError {
    Storage(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::Storage(s) => write!(f, "refresh token store operation failed: {s}"),
        }
    }
}

impl std::error::Error for StoreError {}

#[async_trait]
pub trait RefreshTokenStore: Send + Sync {
    async fn store(
        &self,
        token_id: &str,
        subject_user_id: &str,
        ttl_secs: u64,
    ) -> Result<(), StoreError>;

    async fn validate(&self, token_id: &str) -> Result<Option<String>, StoreError>;

    async fn revoke(&self, token_id: &str) -> Result<(), StoreError>;
}
