use std::env;

use diesel::{
    r2d2::{self, ConnectionManager, PooledConnection},
    PgConnection,
};


#[derive(Clone)]
pub struct Pool {
    pub pool: r2d2::Pool<ConnectionManager<diesel::PgConnection>>,
}

impl Pool {
    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
        self.pool.clone().get().unwrap()
    }

    pub async fn test_pool() -> r2d2::Pool<ConnectionManager<diesel::PgConnection>> {
        use diesel::r2d2::Pool;
        use dotenv::dotenv;
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create pool.");

        pool
    }
}

