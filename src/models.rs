
use serde::{Deserialize, Serialize};
use diesel::Queryable;
use super::schema::users;

#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
