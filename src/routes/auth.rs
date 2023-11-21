use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use diesel::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::db::Pool;
use crate::schema::users::dsl::*;
use crate::models::{NewUser, User, LoginRequest};


#[post("/register", data = "<new_user>")]
pub async fn register(new_user: Json<NewUser>, db_pool: &State<Pool>) -> Status {
    let mut conn = db_pool.get().expect("database connection failure");

    let existing_user = users
        .filter(username.eq(&new_user.username))
        .first::<User>(&mut conn)
        .optional()
        .expect("Error loading user");

    if existing_user.is_some() {
        return Status::Conflict;
    }

    let hashed_password  = hash(&new_user.password_hash, DEFAULT_COST).expect("Error hashing password");

    let new_user = NewUser {
        username: new_user.username.clone(),
        password_hash: hashed_password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error saving new user");

    Status::Created
}

#[post("/login", data = "<login_request>")]
pub async fn login(login_request: Json<LoginRequest>, db_pool: &State<Pool>) -> Status {
    let mut conn = db_pool.get().expect("database connection failure");

    let db_user = users
        .filter(username.eq(&login_request.username))
        .first::<User>(&mut conn)
        .optional()
        .expect("Error loading user");

    match db_user {
        Some(user) => {
            if verify(&login_request.password, &user.password_hash).expect("Error verifying password") {
                Status::Ok
            } else {
                Status::Unauthorized
            }
        }
        None => Status::NotFound,
    }
}
