use super::hash_utils::verify_password;
use crate::session::NotLoggedInUser;
use crate::*;
use actix_session::Session;
use actix_web::{
    web::{Data, Json},
    Responder,
};
use serde::Deserialize;
use sqlx::{query_as, query, PgPool};
use std::convert::TryInto;

#[derive(Deserialize)]
pub struct LoginData {
    email: String,
    password: String,
}

pub async fn login(
    _: NotLoggedInUser,
    session: Session,
    pool: Data<PgPool>,
    data: Json<LoginData>,
) -> impl Responder {
    struct Row {
        id: uuid::Uuid,
        hash: Vec<u8>,
        salt: String,
        nick: String,
    }

    let row = query_as!(
        Row,
        "select id, nick, hash, salt from users where email = $1",
        data.email
    )
    .fetch_one(pool.get_ref())
    .await;

    match row {
        Ok(row) => {
            let hash = row.hash.as_slice().try_into();
            if hash.is_err() {
                log::error!("Hash (Vec<u8>) has wrong length ({})", row.hash.len());
                return resp_500_IntSerErr!();
            }
            if verify_password(&hash.unwrap(), &row.salt, &data.password) {
                match session.set("id", row.id) {
                    Err(_) => {
                        log::error!(
                            "Redis error ({}) - couldn't set id to user session",
                            file!()
                        );
                        resp_500_IntSerErr!()
                    }
                    Ok(_) => {
                        let roles_q = query!(
                            "select role_id from roles_to_users where user_id = $1",
                            row.id
                        )
                        .fetch_all(pool.get_ref())
                        .await;

                        match roles_q {
                            Ok(roles) => {
                                let roles = roles
                                    .into_iter()
                                    .map(|role| role.role_id as i16)
                                    .collect::<Vec<i16>>();
                                #[derive(serde::Serialize)]
                                struct Response {
                                    nick: String,
                                    roles: Vec<i16>,
                                    uuid: uuid::Uuid,
                                }
                                resp_200_Ok_json!(&Response {
                                    nick: row.nick,
                                    roles: roles,
                                    uuid: row.id
                                })
                            }
                            Err(_) => {
                                log::error!(
                                    "Database error ({}) - couldn't get user from database",
                                    file!()
                                );
                                resp_500_IntSerErr!()
                            }
                        }
                    }
                }
            } else {
                resp_400_BadReq!("Invalid password")
            }
        }
        Err(sqlx::Error::RowNotFound) => resp_400_BadReq!("Email doesn't exists"),
        Err(_) => {
            log::error!(
                "Database error ({}) - couldn't get user from database",
                file!()
            );
            resp_500_IntSerErr!()
        }
    }
}
