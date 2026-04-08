use std::sync::Arc;
use todoapp_graphql_config::Config;
use todoapp_graphql_db::{connect_pool, DbPool};
use todoapp_graphql_redis::{connection_manager, RedisRefreshTokenStore};
use todoapp_graphql_refresh_token_port::RefreshTokenStore;

/// The application's state that is available in [`crate::controllers`] and [`crate::middlewares`].
pub struct AppState {
    /// The database pool that's used to get a connection to the application's database (see [`todoapp_graphql_db::DbPool`]).
    pub db_pool: DbPool,
    /// When [`todoapp_graphql_config::Config::redis`] is set, a Redis-backed refresh token store; otherwise `None`.
    pub refresh_tokens: Arc<dyn RefreshTokenStore>,
}

/// The application's state as it is shared across the application, e.g. in controllers and middlewares.
///
/// This is the [`AppState`] struct wrappend in an [`std::sync::Arc`].
pub type SharedAppState = Arc<AppState>;

/// Initializes the application state.
///
/// This function creates an [`AppState`] based on the current [`todoapp_graphql_config::Config`].
pub async fn init_app_state(config: Config) -> AppState {
    let db_pool = connect_pool(config.database)
        .await
        .expect("Could not connect to database!");

    let refresh_tokens = {
        let mgr = connection_manager(&config.redis.url)
            .await
            .expect("Could not connect to Redis!");
        Arc::new(RedisRefreshTokenStore::new(mgr)) as Arc<dyn RefreshTokenStore>
    };

    AppState {
        db_pool,
        refresh_tokens,
    }
}
