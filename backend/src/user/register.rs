use super::hash_utils::{make_hash, make_salt};
use super::model::valid_user_fields;
use crate::session::NotLoggedInUser;
use crate::*;
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;
use sqlx::{query, PgPool};

#[derive(Deserialize)]
pub struct RegisterData {
    nick: String,
    gender: String,
    email: String,
    password: String,
}

pub async fn register(
    _: NotLoggedInUser,
    pool: Data<PgPool>,
    data: Json<RegisterData>,
) -> impl Responder {
    if valid_user_fields(
        Some(&data.nick),
        Some(&data.gender),
        Some(&data.email),
        Some(&data.password),
    ) {
        let salt = make_salt();
        let hash = make_hash(&data.password, &salt);

        match query!(
            "insert into users (nick, gender, email, hash, salt) values ($1,$2,$3,$4,$5)",
            data.nick,
            data.gender,
            data.email,
            hash.to_vec(),
            salt
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => resp_200_Ok!("Ok"),
            Err(sqlx::Error::Database(_)) => HttpResponse::Conflict().body("Email already in use."),
            Err(_) => {
                log::error!(
                    "Database error ({}:{}) - couldn't insert new user",
                    file!(),
                    line!()
                );
                resp_500_IntSerErr!()
            }
        }
    } else {
        resp_400_BadReq!("Invalid Data")
    }
}
