use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use rocket::fairing::AdHoc;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> AdHoc {
    AdHoc::on_ignite("Database Pool", |rocket| async {
        let manager = ConnectionManager::<SqliteConnection>::new("sqlite.db");
        let pool: ::r2d2::Pool<ConnectionManager<SqliteConnection>> = r2d2::Pool::builder()
            .build(manager)
            .expect("db pool failure");
        rocket.manage(pool)
    })
}
