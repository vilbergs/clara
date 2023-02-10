use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use sea_orm::DatabaseConnection;
use sea_orm::*;

use crate::entities::{prelude::*, user::Model, *};

pub struct UserHandler {}

impl UserHandler {
    pub async fn index(State(db): State<DatabaseConnection>) -> Json<Vec<Model>> {
        let users: Vec<user::Model> = match User::find().all(&db).await {
            Ok(users) => users,
            Err(_) => Vec::new(),
        };

        Json(users)
    }
}
