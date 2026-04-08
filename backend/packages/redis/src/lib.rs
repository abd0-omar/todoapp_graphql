mod store;

pub use redis::aio::ConnectionManager;
pub use redis::RedisError;
pub use store::RedisRefreshTokenStore;

pub async fn connection_manager(redis_url: &str) -> Result<ConnectionManager, RedisError> {
    let client = redis::Client::open(redis_url)?;
    ConnectionManager::new(client).await
}
