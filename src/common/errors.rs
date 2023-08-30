use actix_web::{
    body,
    http::{header::ContentType, StatusCode},
    HttpResponse, Responder, ResponseError,
};
use redis::RedisError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServiceError>;

pub struct Resp<T> {
    pub body: T,
    pub code: StatusCode,
}

impl<T> Resp<T> {
    fn new(body: T, code: StatusCode) -> Self {
        Self { body, code }
    }
    pub fn success(body: T) -> Self {
        Self::new(body, StatusCode::OK)
    }
}

impl<T> Responder for Resp<T>
where
    T: serde::ser::Serialize,
{
    type Body = body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(self.code)
            .insert_header(ContentType::json())
            .body(body::BoxBody::new(
                serde_json::to_string(&self.body).unwrap(),
            ))
    }
}

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
    #[error("BadRequest {0}")]
    BadRequestError(String),
    #[error("Unfulfilled identify {0}")]
    Unauthenticated(String),
    #[error("Unallowable scope {0}")]
    Forbidden(String),
    #[error("Unknown information {0}")]
    NotFount(String),
    #[error("Not access {0}")]
    NotAccessible(String),
    #[error("Unprocessable content {0}")]
    UnprocessableEntity(String),
    #[error("an unspecified internal error occurred {0}")]
    InternalError(#[from] anyhow::Error),
    #[error("internal config error {0}")]
    InternalConfigError(#[from] ConfigError),
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            ServiceError::BadRequestError(_) => StatusCode::BAD_REQUEST,
            ServiceError::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
            ServiceError::Forbidden(_) => StatusCode::FORBIDDEN,
            ServiceError::NotFount(_) => StatusCode::NOT_FOUND,
            ServiceError::NotAccessible(_) => StatusCode::NOT_ACCEPTABLE,
            ServiceError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::InternalConfigError(e) => match e {
                ConfigError::DataSource(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ConfigError::Reqwest(e) => e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                ConfigError::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
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
