use super::hash_utils::{make_hash, make_salt};
use super::model::{valid_user_fields, Role};
use crate::session::{AdminUser, LoggedInUser};
use crate::*;
use actix_session::Session;
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;
use sqlx::{query_unchecked, PgPool};
use thiserror::Error;

#[derive(Deserialize)]
pub struct UpdateData {
    nick: Option<String>,
    gender: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[derive(Error, Debug)]
enum Error {
    #[error("User doesn't exists")]
    UserNotFound,
}

async fn process(
    pool: &PgPool,
    id: uuid::Uuid,
    nick: Option<&String>,
    gender: Option<&String>,
    email: Option<&String>,
    password: Option<&String>,
    roles: Option<&Vec<i16>>,
) -> Result<HttpResponse, Error> {
    let (hash, salt) = if let Some(password) = password {
        let salt = make_salt();
        (Some(make_hash(password, &salt).to_vec()), Some(salt))
    } else {
        (None, None)
    };

    match query_unchecked!(
        "call update_user($1,$2,$3,$4,$5,$6,$7)",
        id,
        nick,
        gender,
        email,
        hash,
        salt,
        roles
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(resp_200_Ok!("Ok")),
        Err(sqlx::Error::Database(_)) => Err(Error::UserNotFound),
        Err(_) => Ok(resp_500_IntSerErr!()),
    }
}

pub async fn update_self(
    user: LoggedInUser,
    session: Session,
    pool: Data<PgPool>,
    data: Json<UpdateData>,
) -> impl Responder {
    if valid_user_fields(
        data.nick.as_ref(),
        data.gender.as_ref(),
        data.email.as_ref(),
        data.password.as_ref(),
    ) {
        match process(
            pool.get_ref(),
            user.0,
            data.nick.as_ref(),
            data.gender.as_ref(),
            data.email.as_ref(),
            data.password.as_ref(),
            None,
        )
        .await
        {
            Ok(res) => res,
            Err(error) => match error {
                Error::UserNotFound => {
                    session.purge();
                    HttpResponse::Gone().body("User doesn't exists")
                }
            },
        }
    } else {
        resp_400_BadReq!("Invalid data")
    }
}

#[derive(Deserialize)]
pub struct AdminUpdateData {
    id: uuid::Uuid,
    nick: Option<String>,
    gender: Option<String>,
    email: Option<String>,
    password: Option<String>,
    roles: Option<Vec<Role>>,
}

pub async fn update_user(
    _: AdminUser,
    pool: Data<PgPool>,
    data: Json<AdminUpdateData>,
) -> impl Responder {
    let roles = if let Some(roles) = data.roles.as_ref() {
        Some(
            roles
                .iter()
                .map(|role| *role as i16)
                .collect::<Vec<i16>>(),
        )
    } else {
        None
    };

    match process(
        pool.get_ref(),
        data.id,
        data.nick.as_ref(),
        data.gender.as_ref(),
        data.email.as_ref(),
        data.password.as_ref(),
        roles.as_ref(),
    )
    .await
    {
        Ok(res) => res,
        Err(error) => match error {
            Error::UserNotFound => HttpResponse::Gone().body("User doesn't exists"),
        },
    }
}
