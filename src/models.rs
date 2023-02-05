// use crate::schema::users;
// use chrono::NaiveDateTime;
// use diesel::prelude::*;

// #[derive(Insertable)]
// #[diesel(table_name = users)]
// pub struct NewUser<'a> {
//     pub email: &'a str,
//     pub password: &'a str,
//     pub firstname: &'a str,
//     pub lastname: &'a str,
// }

// #[derive(Queryable)]
// pub struct User {
//     pub id: i32,
//     pub firstname: String,
//     pub lastname: String,
//     pub email: String,
//     pub password: String,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
// }
