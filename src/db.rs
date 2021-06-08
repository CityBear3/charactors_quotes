use super::api_error::ApiError;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use lazy_static::lazy_static;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("Please set db url");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create connection pool")
    };
}

pub fn connection() -> Result<DBConnection, ApiError> {
    POOL.get()
        .map_err(|err| ApiError::new(500, format!("Failed to get connection: {}", err)))
}
