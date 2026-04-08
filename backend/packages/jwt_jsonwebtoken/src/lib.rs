use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use todoapp_graphql_jwt_port::{JwtAuthPort, JwtError, VerifiedToken};

const JWT_ALG: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::HS256;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

/// HMAC-SHA256 JWT issue/verify using a shared secret.
pub struct Hs256JwtService {
    decode_key: DecodingKey,
    encode_key: EncodingKey,
    validation: Validation,
}

impl Hs256JwtService {
    /// Build service from raw secret bytes and optional issuer/audience validation.
    pub fn new(secret: impl AsRef<[u8]>, issuer: Option<String>, audience: Option<String>) -> Self {
        let secret = secret.as_ref();
        let mut validation = Validation::new(JWT_ALG);
        if let Some(ref iss) = issuer {
            validation.set_issuer(&[iss.as_str()]);
        }
        if let Some(ref aud) = audience {
            validation.set_audience(&[aud.as_str()]);
        }
        Self {
            decode_key: DecodingKey::from_secret(secret),
            encode_key: EncodingKey::from_secret(secret),
            validation,
        }
    }
}

fn map_err(e: jsonwebtoken::errors::Error) -> JwtError {
    use jsonwebtoken::errors::ErrorKind;
    match e.kind() {
        ErrorKind::ExpiredSignature => JwtError::Expired,
        ErrorKind::InvalidSignature => JwtError::InvalidSignature,
        _ => JwtError::InvalidToken(e.to_string()),
    }
}

impl JwtAuthPort for Hs256JwtService {
    fn verify_access_token(&self, token: &str) -> Result<VerifiedToken, JwtError> {
        let data = decode::<Claims>(token, &self.decode_key, &self.validation).map_err(map_err)?;
        Ok(VerifiedToken {
            sub: data.claims.sub,
            exp: Some(data.claims.exp),
        })
    }

    fn sign_access_token(&self, subject: &str, ttl_secs: u64) -> Result<String, JwtError> {
        let exp = jsonwebtoken::get_current_timestamp() as i64 + ttl_secs as i64;
        let claims = Claims {
            sub: subject.to_string(),
            exp,
        };
        let header = Header::new(JWT_ALG);
        encode(&header, &claims, &self.encode_key)
            .map_err(|e| JwtError::EncodeFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_roundtrip() {
        let svc = Hs256JwtService::new(b"unit-test-secret-32-bytes-min!!", None, None);
        let token = svc.sign_access_token("user-uuid", 3600).unwrap();
        let v = svc.verify_access_token(&token).unwrap();
        assert_eq!(v.sub, "user-uuid");
        assert!(v.exp.is_some());
    }
}
