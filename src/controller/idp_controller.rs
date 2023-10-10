use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse, Responder,
};

use crate::{
    common::{enums::IdpType, errors::Result},
    pojo::form::client::ClientIdpConfigSaveOrUpdateForm,
    repository::idp_config_repository,
    service::client_service,
};

#[utoipa::path(
    operation_id = "查询指定 client 指定 idp 配置",
    params(
        ("client_id" = String, Path, ),
        ("idp_type" = IdpType, Path, )
    ),
    responses(
        (status = 200, description = "OK", body = ClientIdpConfig),
    )
)]
#[get("/clients/{client_id}/idps/{idp_type}")]
pub async fn client_specify_idp_config(
    path_params: Path<(String, IdpType)>,
) -> Result<impl Responder> {
    let (client_id, idp_type) = path_params.into_inner();
    idp_config_repository::select_specify_idp_config(&client_id, idp_type)
        .await
        .map(Json)
}

#[utoipa::path(
    operation_id = "查询指定 client 的 idp 配置",
    params(
        ("client_id" = String, Path, )
    ),
    responses(
    (status = 200, description = "OK", body = ClientIdpConfig),
    )
)]
#[get("/clients/{client_id}/idps")]
pub async fn client_all_idp_config(client_id: Path<String>) -> Result<impl Responder> {
    Ok(idp_config_repository::select_client_idp_config(&client_id)
        .await
        .map(Json))
}

#[utoipa::path(
    operation_id = "设置指定 client idp 配置",
    params(
        ("client_id" = IdpType, Path, )
    ),
    request_body = ClientIdpConfigSaveOrUpdateForm,
    responses(
        (status = 200, description = "OK", ),
    )
  )]
#[post("/clients/{client_id}/idps")]
pub async fn set_client_idp_config(
    client_id: Path<String>,
    Json(form): Json<ClientIdpConfigSaveOrUpdateForm>,
) -> Result<impl Responder> {
    let client = client_service::get_client(&client_id.into_inner()).await?;
    idp_config_repository::save_or_update_client_idp_config(&client.client_id, &form.into())
        .await?;
    Ok(HttpResponse::Ok().finish())
}
