use super::errors::ConfigError;
use crate::common::config::env_var;
use redis::{aio::Connection, Client, RedisError, RedisResult};

lazy_static::lazy_static! {
  pub static ref CACHE: Client = {
    Client::open(format!("redis://{}:{}", env_var::<String>("REDIS_HOST"), env_var::<u16>("REDIS_PORT")))
        .map_err(|e| ConfigError::Redis(e))
        .unwrap()
  };
}

pub async fn borrow() -> Connection {
    CACHE
        .get_async_connection()
        .await
        .map_err(|e: redis::RedisError| ConfigError::Redis(e))
        .unwrap()
}

pub async fn set<T>(key: &str, value: T) -> RedisResult<()>
where
    T: serde::Serialize,
{
    let mut conn = borrow().await;

    let vv = serde_json::to_string(&value).map_err(|e| RedisError::from(e))?;
    redis::cmd("set")
        .arg(key)
        .arg(vv)
        .query_async(&mut conn)
        .await
}

pub async fn get<T>(key: &str) -> Result<Option<T>, RedisError>
where
    T: serde::de::DeserializeOwned,
{
    let mut conn = borrow().await;
    let value: Option<String> = redis::cmd("get").arg(key).query_async(&mut conn).await?;
    match value {
        Some(v) => {
            let deserialize = serde_json::from_str::<T>(&v).map_err(|e| RedisError::from(e))?;
            Ok(Some(deserialize))
        }
        None => Ok(None),
    }
}
