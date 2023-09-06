use actix_web::{
    get, patch, post,
    web::{Json, Path},
    HttpResponse, Responder,
};

use crate::{
    common::{config::Patch, enums::IdpType, errors::Result},
    pojo::{
        form::client::{
            ClientConfigCreateForm, ClientConfigUpdateForm, ClientIdpConfigSaveOrUpdateForm,
        },
        po::client::ClientIdpConfig,
    },
    repository::{client_config_repository, client_idp_config_repository},
    service::client_service,
};

#[utoipa::path(
    params(
        ClientConfigCreateForm
    ),
    responses(
        (status = 200, description = "OK"),
    )
)]
#[post("/clients")]
pub async fn create_client(
    actix_web::web::Json(form): actix_web::web::Json<ClientConfigCreateForm>,
) -> Result<impl Responder> {
    client_config_repository::create_client(&form.into()).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    params(
        ("client_id" = String, Path,),
    ),
    responses(
        (status = 200, description = "OK"),
    )
)]
#[get("/clients/{client_id}")]
pub async fn client_config(client_id: actix_web::web::Path<String>) -> Result<impl Responder> {
    client_service::get_client_config(&client_id.into_inner())
        .await
        .map(Json)
}

#[utoipa::path(
    params(
        ClientConfigUpdateForm
    ),
    responses(
        (status = 200, description = "OK"),
    )
)]
#[patch("/clients")]
pub async fn set_client_config(
    actix_web::web::Json(form): actix_web::web::Json<ClientConfigUpdateForm>,
) -> Result<impl Responder> {
    let mut origin = client_service::get_client_config(&form.client_id).await?;
    form.merge(&mut origin);
    client_config_repository::set_client_config(&origin).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
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
    client_idp_config_repository::select_spcify_idp_config(&client_id, idp_type)
        .await
        .map(Json)
}

#[utoipa::path(
    params(
        ("client_id" = String, Path, )
    ),
    responses(
    (status = 200, description = "OK", body = ClientIdpConfig),
    )
)]
#[get("/clients/{client_id}/idps")]
pub async fn client_all_idp_config(client_id: Path<String>) -> Result<impl Responder> {
    Ok(
        client_idp_config_repository::select_client_idp_config(&client_id)
            .await
            .map(Json),
    )
}

#[utoipa::path(
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
    let client = client_service::get_client_config(&client_id.into_inner()).await?;
    let mut idp_config: ClientIdpConfig = form.into();
    idp_config.client_id = client.client_id;
    client_idp_config_repository::save_or_update_client_idp_config(&idp_config).await?;
    Ok(HttpResponse::Ok().finish())
}
