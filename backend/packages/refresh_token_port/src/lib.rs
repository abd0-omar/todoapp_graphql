use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("refresh token store operation failed: {0}")]
    Storage(String),
}

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
