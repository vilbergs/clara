use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: i32) -> Self {
        let exp = Utc::now() + Duration::minutes(15);

        Self {
            sub,
            exp: exp.timestamp() as usize,
        }
    }

    pub fn to_token(&self) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret("secret".as_ref()),
        )
    }

    pub fn from_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(Algorithm::HS256),
        )
    }
}
