use actix_web::{get, patch, post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    common::{config::Patch, datasource::from_vec, errors::Result, utils::gen_id},
    pojo::po::client::ClientConfig,
    repository::client_repository,
    service::client_service,
};

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ClientConfigCreateForm {
    pub name: String,
    pub logo: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ClientConfigUpdateForm {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(deserialize_with = "from_vec")]
    pub redirect_url: Option<String>,
}

impl Patch for ClientConfigUpdateForm {
    type Into = ClientConfig;

    fn merge(&self, into: &mut Self::Into) {
        if self.name.is_some() {
            into.name = self.name.clone().unwrap();
        }
        if self.logo.is_some() {
            into.logo = self.logo.clone().unwrap();
        }
        if self.desc.is_some() {
            into.description = self.desc.clone().unwrap();
        }
        if self.redirect_url.is_some() {
            into.redirect_url = self.redirect_url.clone();
        }
    }
}

impl Into<ClientConfig> for ClientConfigCreateForm {
    fn into(self) -> ClientConfig {
        ClientConfig {
            client_id: gen_id(32),
            name: self.name,
            logo: self.logo,
            description: self.desc.unwrap_or_default(),
            ..Default::default()
        }
    }
}

#[utoipa::path(
    params(
        ClientConfigCreateForm
    ),
    responses(
        (status = 200, description = "OK"),
    )
)]
#[post("/client")]
pub async fn create_client(
    actix_web::web::Json(form): actix_web::web::Json<ClientConfigCreateForm>,
) -> Result<impl Responder> {
    client_repository::create_client(&form.into()).await?;
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
#[get("/client/{client_id}")]
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
#[patch("/client")]
pub async fn set_client_config(
    actix_web::web::Json(form): actix_web::web::Json<ClientConfigUpdateForm>,
) -> Result<impl Responder> {
    let mut origin = client_service::get_client_config(&form.client_id).await?;
    form.merge(&mut origin);
    client_repository::set_client_config(&origin).await?;
    Ok(HttpResponse::Ok().finish())
}
