use std::string::String;

use chrono::Duration;
use lazy_static::lazy_static;
use serde::{ser::SerializeSeq, Deserialize};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

use crate::common::config::env_var;

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

pub fn to_vec<S>(v: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let str_split: Vec<String> = v.split(',').map(|s| s.to_string()).collect();
    let mut seq_serializer: <S as serde::Serializer>::SerializeSeq =
        serializer.serialize_seq(Some(str_split.len()))?;
    for strs in str_split {
        seq_serializer.serialize_element(&strs)?;
    }
    seq_serializer.end()
}

pub fn from_vec<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let str_sequence = Vec::<String>::deserialize(deserializer)?;
    if str_sequence.len() == 0 {
        Ok(None)
    } else {
        Ok(Some(str_sequence.join(",")))
    }
}
