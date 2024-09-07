use deadpool_diesel::postgres::{Manager, Pool};
use std::env;

pub fn establish_connection_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = Manager::new(&database_url, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager).build().unwrap()
}
