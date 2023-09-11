use actix_web::error::ErrorNotFound;
use anyhow::Context;
use serde_json::json;
use sqlx::QueryBuilder;

use crate::{
    common::{datasource::CONN, enums::IdpType, errors::Result},
    pojo::{dto::user::UserAssociationDTO, po::user::UserAssociation},
};

pub async fn select_user_association(
    openid: &str,
    idp: IdpType,
    idp_openid: Option<String>,
) -> Result<Option<UserAssociationDTO>> {
    let mut builder = QueryBuilder::new(
        "SELECT idp_openid, idp_type, idp_extra FROM t_user_association WHERE openid = ? AND \
         idp_type = ? ",
    );
    builder.push_bind(openid).push_bind(idp);
    if let Some(idp_oid) = idp_openid {
        builder.push("AND idp_openid = ? ");
        builder.push_bind(idp_oid);
    };
    Ok(builder
        .build_query_as::<UserAssociationDTO>()
        .fetch_optional(&*CONN)
        .await
        .context(format!("select user{openid} associations failed"))?)
}

pub async fn select_user_associations(openid: &str) -> Result<Vec<UserAssociationDTO>> {
    Ok(sqlx::query_as!(
        UserAssociationDTO,
        "SELECT idp_openid, idp_type, idp_extra FROM t_user_association WHERE openid = ?",
        openid
    )
    .fetch_all(&*CONN)
    .await
    .context(format!("select user{openid} associations failed"))?)
}

pub async fn select_user_associations_by_idp_openid(
    idp_openid: &str,
) -> Result<(String, Vec<UserAssociationDTO>)> {
    let idp_association = &(sqlx::query_as!(
        UserAssociation,
        "SELECT openid, idp_openid, idp_type, idp_extra FROM t_user_association WHERE idp_openid \
         = ?",
        idp_openid
    )
    .fetch_optional(&*CONN)
    .await
    .context("select user{idp_openid} associations failed")?
    .ok_or(ErrorNotFound(format!(
        "Not fount user{idp_openid} associations"
    )))?);

    Ok((
        idp_association.openid.clone(),
        select_user_associations(&idp_association.openid).await?,
    ))
}

pub async fn create_associations(
    tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
    openid: &str,
    associations: &Vec<UserAssociationDTO>,
) -> Result<()> {
    let mut builder: QueryBuilder<'_, sqlx::MySql> = QueryBuilder::new(
        "INSERT INTO t_user_association(openid, idp_type, idp_openid, idp_extra) ",
    );
    builder
        .push_values(associations, |mut v, assocition| {
            v.push_bind(openid)
                .push_bind(assocition.idp_type)
                .push_bind(assocition.idp_openid.to_string())
                .push_bind(json!(assocition.idp_extra.clone()));
        })
        .build()
        .execute(&mut **tx)
        .await
        .with_context(|| {
            let msg = format!("ceate user({openid}) association failed");
            tracing::error!(msg);
            msg
        })?;

    Ok(())
}
