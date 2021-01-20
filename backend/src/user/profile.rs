use crate::*;
use actix_web::{
    web::{Data, Path},
    Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Deserialize, Serialize)]
struct User {
    nick: String,
    created_at: time::PrimitiveDateTime,
    gender: String,
    description: String,
}

#[derive(Serialize)]
struct Game {
    name: String,
    id: uuid::Uuid,
    players: Option<serde_json::value::Value>,
    ended: bool,
    winner: Option<uuid::Uuid>,
    moves_needed: i16,
}

#[derive(Serialize)]
struct ResponseData {
    user: User,
    games: Vec<Game>,
}

pub async fn profile(pool: Data<PgPool>, Path(id): Path<uuid::Uuid>) -> impl Responder {
    match query_as!(
        User,
        "select nick, created_at, gender, description from users where id = $1",
        id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(user) => {
            match query_as!(
                Game,
                "select id, name, ended, winner, moves_needed, (select jsonb_agg(jsonb_build_array(nick, id)) as players from users where players_id ? id::text group by true) from (select id, name, ended, winner, moves_needed, a.users as players_id from (select game_id, jsonb_agg(user_id) as users from games_to_users group by game_id) a join games on games.id = a.game_id where a.users ? $1) b",
                id.to_string(),
                )
                .fetch_all(pool.get_ref())
                .await
            {
                Ok(games) => resp_200_Ok_json!(&ResponseData { 
                    user,
                    games
                }),
                Err(_) => resp_500_IntSerErr!()
            }
        }
        Err(sqlx::Error::RowNotFound) => resp_400_BadReq!("User with that id doesn't exists"),
        Err(_) => resp_500_IntSerErr!(),
    }
}
