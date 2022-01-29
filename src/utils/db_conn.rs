use diesel::r2d2::{self, ConnectionManager, PooledConnection};

#[derive(Clone)]
pub struct Pool {
    pub pool: r2d2::Pool<ConnectionManager<diesel::PgConnection>>,
}

impl Pool {
    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
        self.pool.clone().get().unwrap()
    }
}
