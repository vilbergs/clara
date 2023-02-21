use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use orion::pwhash;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::auth::Claims;
use crate::entities::{user, user::Entity as User};
use crate::middleware::ExtractClaims;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

pub async fn index(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let users: Vec<user::Model> = match User::find().all(&db).await {
        Ok(users) => users,
        Err(_) => Vec::new(),
    };

    Json(users)
}

pub async fn show(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>,
    ExtractClaims(claims): ExtractClaims,
) -> impl IntoResponse {
    if user_id != claims.sub {
        return Err((
            StatusCode::UNAUTHORIZED,
            format!("{} doesnt match {}", user_id, claims.sub),
        ));
    }

    let user: user::Model = User::find_by_id(claims.sub)
        .one(&db)
        .await
        .expect("Could not fetch user")
        .unwrap();

    return Ok(Json(user));
}

pub async fn store(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<user::Model>,
) -> impl IntoResponse {
    let password = pwhash::Password::from_slice(payload.password.as_bytes())
        .expect("Could not create password");
    let pw_hash = pwhash::hash_password(&password, 3, 1 << 16).expect("Could not hash password");

    let new_user = user::ActiveModel {
        name: Set(payload.name.to_owned()),
        email: Set(payload.email.to_owned()),
        password: Set(pw_hash.unprotected_as_encoded().into()),
        ..Default::default()
    };

    let user = new_user.insert(&db).await.unwrap();

    Json(user)
}

pub async fn authorize(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginCredentials>,
) -> impl IntoResponse {
    todo!("Implement authorize endpoint")
}

pub async fn token(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginCredentials>,
) -> impl IntoResponse {
    let maybe_user = User::find()
        .filter(user::Column::Email.eq(payload.email))
        .one(&db)
        .await
        .expect("Could not fetch user");

    let password = pwhash::Password::from_slice(payload.password.as_bytes()).unwrap();

    if let Some(user) = maybe_user {
        let user_pw_hash = pwhash::PasswordHash::from_encoded(&user.password).unwrap();
        let verified = pwhash::hash_password_verify(&user_pw_hash, &password).is_ok();

        if verified {
            let claims = Claims::new(user.id);

            let token = claims.to_token().ok();

            return match token {
                Some(token) => Ok(Json(json!({ "access_token": token, "type": "Bearer" }))),
                None => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".into(),
                )),
            };
        }

        return Err((StatusCode::UNAUTHORIZED, ""));
    }

    Err((StatusCode::UNAUTHORIZED, ""))
}
