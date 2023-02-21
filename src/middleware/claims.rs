use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};

use crate::auth::Claims;

pub struct ExtractClaims(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractClaims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get(AUTHORIZATION);

        if let Some(header_value) = auth_header {
            let token = header_value.to_str().unwrap().split(" ").nth(1).unwrap();

            if let Some(token_data) = Claims::from_token(token).ok() {
                return Ok(ExtractClaims(token_data.claims));
            }
        }

        Err((StatusCode::UNAUTHORIZED, "Invalid access token"))
    }
}
