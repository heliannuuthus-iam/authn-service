use anyhow::Context;

use crate::{
    common::{
        datasource::CONN,
        errors::{Result, ServiceError::NotFount},
    },
    pojo::{dto::user::UserAssociationDTO, po::user::UserAssociation},
};

pub async fn select_user_associations(openid: &str) -> Result<Vec<UserAssociationDTO>> {
    Ok(sqlx::query_as!(
        UserAssociation,
        "SELECT openid, idp_openid, idp_type FROM t_user_association WHERE openid = ?",
        openid
    )
    .fetch_all(&*CONN)
    .await
    .context(format!("select user{openid} associations failed"))?
    .iter()
    .map(|s| UserAssociationDTO::from(s.clone()))
    .collect())
}

pub async fn select_user_associations_by_idp_openid(
    idp_openid: &str,
) -> Result<(String, Vec<UserAssociationDTO>)> {
    let idp_association = &(sqlx::query_as!(
        UserAssociation,
        "SELECT openid, idp_openid, idp_type FROM t_user_association WHERE idp_openid = ?",
        idp_openid
    )
    .fetch_optional(&*CONN)
    .await
    .context("select user{idp_openid} associations failed")?
    .ok_or(NotFount(format!("Not fount user{idp_openid} associations")))?);

    Ok((
        idp_association.openid.clone(),
        select_user_associations(&idp_association.openid).await?,
    ))
}
