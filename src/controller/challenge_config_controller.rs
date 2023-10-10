use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse, Responder,
};

use crate::{
    common::errors::Result, pojo::po::challenge::ChallengeCofig,
    repository::challenge_config_repository,
};

#[utoipa::path(
    operation_id = "根据 client_id 查询 challenge 配置",
    params(
        ("client_id" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = ChallengeCofig),
    )
)]
#[get("/config/challenge/{client_id}")]
pub async fn list_challenge_config(client_id: Path<String>) -> Result<impl Responder> {
    challenge_config_repository::select_challenge_configs(&client_id.into_inner())
        .await
        .map(Json)
}

#[utoipa::path(
    operation_id = "为指定 client 设置 challenge 配置",
    request_body = ChallengeCofig,
    responses(
        (status = 200, description = "OK"),
    )
)]
#[post("/config/challenge")]
pub async fn set_challenge_config(Json(config): Json<ChallengeCofig>) -> Result<impl Responder> {
    challenge_config_repository::save_challenge_config(&config).await?;
    Ok(HttpResponse::Ok().finish())
}
