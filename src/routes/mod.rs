mod auth;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![
        auth::register,
        auth::login,
    ]
}
