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
    players: serde_json::value::Value,
    ended: bool,
    winner: Option<uuid::Uuid>,
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
            struct RowData {
                name: String,
                id: uuid::Uuid,
                players: Option<serde_json::value::Value>,
                ended: bool,
                winner: Option<uuid::Uuid>,
            }
            match query_as!(
                RowData,
                "select id, name, ended, winner, (select jsonb_agg(jsonb_build_array(nick, id)) as players from users where players_id ? id::text group by true) from (select id, name, ended, winner, a.users as players_id from (select game_id, jsonb_agg(user_id) as users from games_to_users group by game_id) a join games on games.id = a.game_id where a.users ? $1) b",
                id.to_string(),
                )
                .fetch_all(pool.get_ref())
                .await
            {
                Ok(games) => resp_200_Ok_json!(&ResponseData { 
                    user,
                    games: games
                        .into_iter()
                        .map(|RowData { name, id, players, ended, winner }: RowData|
                            Game { name, id, ended, winner, players: players.unwrap() }
                            )
                        .collect()
                }),
                Err(_) => resp_500_IntSerErr!()
            }
        }
        Err(sqlx::Error::RowNotFound) => resp_400_BadReq!("User with that id doesn't exists"),
        Err(_) => resp_500_IntSerErr!(),
    }
}
