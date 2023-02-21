use axum::{routing::get, routing::post, Router};
use dotenvy::dotenv;
use sea_orm::*;
use std::env;

mod auth;
mod entities;
mod handlers;
mod middleware;

use handlers::user as UserHandler;

async fn run() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_URL must be set");
    let db = Database::connect(format!("{}/{}", db_url, "postgres")).await?;

    let db = match db.get_database_backend() {
        DbBackend::Postgres => {
            // Check if DB exists before creating, running this in one query does not work with sea_orm
            let existing_db = db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower('{}')", db_name),
            ))
            .await?;

            if existing_db.rows_affected() == 0 {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE {}", db_name),
                ))
                .await?;
            }

            let url = format!("{}/{}", db_url, db_name);
            Database::connect(&url).await?
        }
        DbBackend::MySql => todo!("Implement connection for MySQLs"),
        DbBackend::Sqlite => todo!("Implement connection for Sqlite"),
    };

    Ok(db)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = run().await.expect("DB Connection failed");

    let app = Router::new()
        .route("/users", get(UserHandler::index).post(UserHandler::store))
        .route("/users/:id", get(UserHandler::show))
        .route("/oauth/token", post(UserHandler::token))
        .with_state(db);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Server Crashed")
}
