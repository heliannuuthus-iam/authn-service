use actix_web::{
    delete, get, patch, post,
    web::{Json, Path},
    HttpResponse, Responder,
};

use crate::{
    common::{config::Patch, errors::Result},
    pojo::form::server::{ServerSettingCreateForm, ServerSettingUpdateForm},
    service::server_service,
};

#[utoipa::path(
  operation_id = "根据 server_id 查询 server 详细信息",
  params(
      ("server_id" = String, Path,),
  ),
  responses(
      (status = 200, description = "OK"),
  )
)]
#[get("/servers/{server_id}")]
pub async fn server_detail(server_id: Path<String>) -> Result<impl Responder> {
    server_service::get_server_setting(&server_id)
        .await
        .map(Json)
}

#[utoipa::path(
  operation_id = "创建 server",
  request_body = ServerSettingCreateForm,
  responses(
      (status = 200, description = "OK"),
  )
)]
#[post("/servers")]
pub async fn create_server(Json(server): Json<ServerSettingCreateForm>) -> Result<impl Responder> {
    server_service::set_server(&server.into()).await.map(Json)
}

#[utoipa::path(
  operation_id = "修改 server 配置",
  request_body = ServerSettingUpdateForm,
  responses(
      (status = 200, description = "OK"),
  )
)]
#[patch("/servers")]
pub async fn patch_server(Json(form): Json<ServerSettingUpdateForm>) -> Result<impl Responder> {
    let mut setting = server_service::get_server_setting(&form.server_id).await?;
    form.merge(&mut setting);
    server_service::set_server(&setting).await.map(Json)
}

#[utoipa::path(
  operation_id = "删除 server",
  params(
    ("server_id" = String, Path,),
  ),
  responses(
      (status = 200, description = "OK"),
  )
)]
#[delete("/servers/{server_id}")]
pub async fn remove_server(server_id: Path<String>) -> Result<impl Responder> {
    server_service::get_server(&server_id).await?;
    server_service::delete_server(&server_id).await?;
    Ok(HttpResponse::Ok())
}
