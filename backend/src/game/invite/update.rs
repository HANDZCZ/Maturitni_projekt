use crate::game::model::GameData;
use crate::session::LoggedInUser;
use crate::*;
use actix_web::{
    web::{Data, Json},
    Responder,
};
use serde::Deserialize;
use sqlx::{query, PgPool};

#[derive(Deserialize)]
pub struct InviteData {
    id: uuid::Uuid,
    accepted: bool,
}

pub async fn update(
    user: LoggedInUser,
    pool: Data<PgPool>,
    data: Json<InviteData>,
) -> impl Responder {
    match query!(
        "call update_invite($1, $2, $3, $4)",
        user.0,
        data.id,
        data.accepted,
        bincode::serialize(&GameData::default()).unwrap()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => resp_200_Ok!("Ok"),
        Err(sqlx::Error::Database(_)) => {
            resp_400_BadReq!("You are not in this game invite or the game invite doesn't exists.")
        }
        Err(_) => resp_500_IntSerErr!(),
    }
}
