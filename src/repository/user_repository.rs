use anyhow::Context;
use sqlx::{mysql::MySql, QueryBuilder};

use crate::{
    common::{datasource::CONN, errors::Result},
    pojo::po::user::User,
};

pub async fn save_user(user: &User) -> Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO t_user(avatar, username, gander, email, email_verified) VALUES(?, ?, ?, ?, ?)
        "#,
        user.avatar,
        user.username,
        user.gander,
        user.email,
        user.email_verified
    )
    .execute(&*CONN)
    .await.context("create user failed")?;
    Ok(())
}

pub async fn select_profile(openid: Option<String>, email: Option<String>) -> Result<Option<User>> {
    let mut query: QueryBuilder<'_, MySql> = QueryBuilder::new(
        "SELECT openid, avatar, username, gander, email, email_verified, registried_at FROM \
         t_user WHERE 1=1 ",
    );
    if let Some(e) = email {
        query.push(" AND email = ");
        query.push_bind(e);
    };
    if let Some(o) = openid {
        query.push(" AND openid = ");
        query.push_bind(o);
    };
    Ok(query
        .build_query_as::<User>()
        .fetch_optional(&*CONN)
        .await
        .context("select user profile failed")?)
}
