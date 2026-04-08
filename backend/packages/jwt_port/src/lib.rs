use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedToken {
    pub sub: String,
    pub exp: Option<i64>,
}

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("invalid JWT: {0}")]
    InvalidToken(String),
    #[error("JWT has expired")]
    Expired,
    #[error("JWT signature verification failed")]
    InvalidSignature,
    #[error("failed to encode JWT: {0}")]
    EncodeFailed(String),
}

pub trait JwtAuthPort: Send + Sync {
    fn verify_access_token(&self, token: &str) -> Result<VerifiedToken, JwtError>;

    fn sign_access_token(&self, subject: &str, ttl_secs: u64) -> Result<String, JwtError>;
}
