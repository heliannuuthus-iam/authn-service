use actix_web::{body, HttpResponse, Responder, ResponseError};
use actix_web::{
    http::{
        header::{self, ContentType, HeaderValue},
        ConnectionType, StatusCode,
    },
    web,
};
use redis::RedisError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServiceError>;

pub struct Resp<T> {
    pub body: T,
    pub status: StatusCode,
}

impl<T> Resp<T> {
    fn new(body: T, status: StatusCode) -> Self {
        Self { body, status }
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

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::build(self.status)
            .insert_header(ContentType::json())
            .body(body::BoxBody::new(
                serde_json::to_string(&self.body).unwrap(),
            ))
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Redis error {0}")]
    Redis(RedisError),
    #[error("datasource error {0}")]
    DataSource(sqlx::Error),
    #[error("reqwest error {0}")]
    Reqwest(reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("BadRequest {0}")]
    BadRequestError(String),
    #[error("Unfulfilled identify {0}")]
    Unauthenticated(String),
    #[error("Unallowable scope {0}")]
    Forbidden(String),
    #[error("Unknown infomation {0}")]
    NotFount(String),
    #[error("Not access {0}")]
    NotAccessable(String),
    #[error("Unprocessable content {0}")]
    UnprocessableEntity(String),
    #[error("an unspecified internal error occurred {0}")]
    InternalError(#[from] anyhow::Error),
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            ServiceError::BadRequestError(_) => StatusCode::BAD_REQUEST,
            ServiceError::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
            ServiceError::Forbidden(_) => StatusCode::FORBIDDEN,
            ServiceError::NotFount(_) => StatusCode::NOT_FOUND,
            ServiceError::NotAccessable(_) => StatusCode::NOT_ACCEPTABLE,
            ServiceError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(format!(
                r#"{{
                "code": {},
                "msg": "{}"
            }}"#,
                self.status_code().as_str(),
                self.to_string()
            ))
    }
}
