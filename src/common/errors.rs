use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use redis::RedisError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Redis error {0}")]
    Redis(#[from] RedisError),
    #[error("datasource error {0}")]
    DataSource(#[from] sqlx::Error),
    #[error("reqwest error {0}")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("{0}")]
    ReponseError(#[from] actix_web::Error),
    #[error("an unspecified internal error occurred {0}")]
    InternalError(#[from] anyhow::Error),
    #[error("internal config error {0}")]
    InternalConfigError(#[from] ConfigError),
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            ServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::InternalConfigError(e) => match e {
                ConfigError::DataSource(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ConfigError::Reqwest(e) => e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                ConfigError::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            ServiceError::ReponseError(e) => e.as_response_error().status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(format!(
                r#"{{
                "code": {},
                "msg": "{}"
            }}"#,
                self.status_code().as_str(),
                self
            ))
    }
}
