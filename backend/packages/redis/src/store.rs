use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use todoapp_graphql_refresh_token_port::{RefreshTokenStore, StoreError};

pub struct RedisRefreshTokenStore {
    conn: ConnectionManager,
    key_prefix: &'static str,
}

impl RedisRefreshTokenStore {
    // rt stands for refresh token
    pub const DEFAULT_KEY_PREFIX: &'static str = "rt:";

    pub fn new(conn: ConnectionManager) -> Self {
        Self {
            conn,
            key_prefix: Self::DEFAULT_KEY_PREFIX,
        }
    }

    fn key(&self, token_id: &str) -> String {
        format!("{}{}", self.key_prefix, token_id)
    }
}

#[async_trait]
impl RefreshTokenStore for RedisRefreshTokenStore {
    async fn store(
        &self,
        token_id: &str,
        subject_user_id: &str,
        ttl_secs: u64,
    ) -> Result<(), StoreError> {
        let key = self.key(token_id);
        let mut conn = self.conn.clone();
        conn.set_ex::<_, _, ()>(key, subject_user_id, ttl_secs)
            .await
            .map_err(|e| StoreError::Storage(e.to_string()))
    }

    async fn validate(&self, token_id: &str) -> Result<Option<String>, StoreError> {
        let key = self.key(token_id);
        let mut conn = self.conn.clone();
        let v: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| StoreError::Storage(e.to_string()))?;
        Ok(v)
    }

    async fn revoke(&self, token_id: &str) -> Result<(), StoreError> {
        let key = self.key(token_id);
        let mut conn = self.conn.clone();
        conn.del::<_, ()>(key)
            .await
            .map_err(|e| StoreError::Storage(e.to_string()))
    }
}
