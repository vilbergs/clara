use chrono::{DateTime, NaiveDateTime, Utc};
use dotenvy::dotenv;
use sea_orm::*;
use std::env;

mod entities;

use entities::{prelude::*, *};

async fn run() -> Result<(), DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_URL must be set");
    let db = Database::connect(format!("{}/{}", db_url, "postgres")).await?;

    let db = &match db.get_database_backend() {
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

    let new_user = user::ActiveModel {
        name: ActiveValue::Set("Vilberg".to_owned()),
        email: ActiveValue::Set("v@example.com".to_owned()),
        password: ActiveValue::Set("Vilberg".to_owned()),
        salt: ActiveValue::Set("Vilberg".to_owned()),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let res = User::insert(new_user).exec(db).await?;

    println!("{:?}", res);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();

    run().await?;

    Ok(())
}
