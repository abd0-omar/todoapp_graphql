use std::sync::Arc;

use todoapp_graphql_config::Config;
use todoapp_graphql_db::{connect_pool, DbPool};
use todoapp_graphql_jwt_jsonwebtoken::Hs256JwtService;
use todoapp_graphql_jwt_port::JwtAuthPort;
use todoapp_graphql_redis::{connection_manager, RedisRefreshTokenStore};
use todoapp_graphql_refresh_token_port::RefreshTokenStore;

use crate::graphql::{build_schema, AppSchema, AuthTtls};

/// The application's state that is available in [`crate::controllers`] and [`crate::middlewares`].
pub struct AppState {
    /// The database pool that's used to get a connection to the application's database (see [`todoapp_graphql_db::DbPool`]).
    pub db_pool: DbPool,
    /// Redis-backed refresh token store.
    pub refresh_tokens: Arc<dyn RefreshTokenStore>,
    /// JWT access-token issue and verification.
    pub jwt: Arc<dyn JwtAuthPort>,
    /// TTL (seconds) for newly issued access tokens (e.g. login/signup).
    pub jwt_access_token_ttl_secs: u64,
    /// GraphQL schema (shared across requests).
    pub graphql_schema: AppSchema,
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

    let jwt: Arc<dyn JwtAuthPort> = Arc::new(Hs256JwtService::new(
        config.jwt.secret.as_bytes(),
        config.jwt.issuer.clone(),
        config.jwt.audience.clone(),
    ));

    let jwt_access_token_ttl_secs = config.jwt.access_token_ttl_secs;
    let auth_ttls = AuthTtls {
        access_token_secs: jwt_access_token_ttl_secs,
        refresh_token_secs: config.jwt.refresh_token_ttl_secs,
    };
    let graphql_schema = build_schema(
        db_pool.clone(),
        jwt.clone(),
        auth_ttls,
        refresh_tokens.clone(),
    );

    AppState {
        db_pool,
        refresh_tokens,
        jwt,
        jwt_access_token_ttl_secs,
        graphql_schema,
    }
}
