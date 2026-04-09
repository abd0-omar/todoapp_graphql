use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedToken {
    pub sub: String,
    pub exp: Option<i64>,
}

#[derive(Debug)]
pub enum JwtError {
    InvalidToken(String),
    Expired,
    InvalidSignature,
    EncodeFailed(String),
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::InvalidToken(s) => write!(f, "invalid JWT: {s}"),
            JwtError::Expired => write!(f, "JWT has expired"),
            JwtError::InvalidSignature => write!(f, "JWT signature verification failed"),
            JwtError::EncodeFailed(s) => write!(f, "failed to encode JWT: {s}"),
        }
    }
}

impl std::error::Error for JwtError {}

pub trait JwtAuthPort: Send + Sync {
    fn verify_access_token(&self, token: &str) -> Result<VerifiedToken, JwtError>;

    fn sign_access_token(&self, subject: &str, ttl_secs: u64) -> Result<String, JwtError>;
}
