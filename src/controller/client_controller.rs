use actix_web::{
    delete, get, patch, post,
    web::{Json, Path},
    HttpResponse, Responder,
};

use crate::{
    common::{config::Patch, errors::Result},
    pojo::{
        form::client::{ClientSettingCreateForm, ClientSettingUpdateForm},
        po::client::ClientSetting,
    },
    repository::client_repository,
    service::client_service::{self},
};

#[utoipa::path(
  operation_id = "创建 client",
  request_body = ClientSettingCreateForm,
  responses(
      (status = 200, description = "OK"),
  )
)]
#[post("/clients")]
pub async fn create_client(
    actix_web::web::Json(form): actix_web::web::Json<ClientSettingCreateForm>,
) -> Result<impl Responder> {
    let setting: ClientSetting = form.into();
    client_repository::insert_or_update_client(&setting.client).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  operation_id = "查询 client 列表",
  responses(
      (status = 200, description = "OK"),
  )
)]
#[get("/clients")]
pub async fn list_clients(client_id: actix_web::web::Path<String>) -> Result<impl Responder> {
    client_service::get_client_setting(&client_id.into_inner())
        .await
        .map(Json)
}

#[utoipa::path(
  operation_id = "根据 client_id 查询 client 详细信息",
  params(
      ("client_id" = String, Path,),
  ),
  responses(
      (status = 200, description = "OK"),
  )
)]
#[get("/clients/{client_id}")]
pub async fn client_details(client_id: actix_web::web::Path<String>) -> Result<impl Responder> {
    client_service::get_client_setting(&client_id.into_inner())
        .await
        .map(Json)
}

#[utoipa::path(
  operation_id = "修改 client 配置",
  request_body = ClientSettingUpdateForm,
  responses(
      (status = 200, description = "OK"),
  )
)]
#[patch("/clients")]
pub async fn set_client_config(
    actix_web::web::Json(form): actix_web::web::Json<ClientSettingUpdateForm>,
) -> Result<impl Responder> {
    let mut origin = client_service::get_client_setting(&form.client_id).await?;
    form.merge(&mut origin);
    client_repository::insert_or_update_client_setting(&origin).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  operation_id = "删除 client",
  params(
    ("client_id" = String, Path,),
  ),
  responses(
      (status = 200, description = "OK"),
  )
)]
#[delete("/clients/{client_id}")]
pub async fn remove_client(client_id: Path<String>) -> Result<impl Responder> {
    client_service::get_client(&client_id).await?;
    client_service::remove_client(&client_id).await?;
    Ok(HttpResponse::Ok().finish())
}
