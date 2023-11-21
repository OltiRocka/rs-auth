#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod db;
mod schema;
mod routes;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::init_pool())
        .mount("/", routes::get_routes())
}
