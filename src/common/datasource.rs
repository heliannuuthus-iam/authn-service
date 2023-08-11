use crate::common::config::env_var;
use chrono::Duration;
use lazy_static::lazy_static;
use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};

lazy_static! {
    pub static ref CONN: Pool<MySql> = {
        MySqlPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::seconds(2).to_std().unwrap())
            .idle_timeout(Duration::seconds(60).to_std().unwrap())
            .connect_lazy(env_var::<String>("DATABASE_URL").as_str())
            .unwrap()
    };
}
