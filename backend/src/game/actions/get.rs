use crate::*;
use actix_web::{
    web::{Data, Path},
    Responder,
};
use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(Serialize)]
struct Game {
    players: Option<serde_json::value::Value>,
    data: Vec<u8>,
    ended: bool,
    winner: Option<uuid::Uuid>,
    last_played: uuid::Uuid,
    moves_needed: i16,
}

pub async fn get(pool: Data<PgPool>, Path(id): Path<uuid::Uuid>) -> impl Responder {
    match query_as!(
        Game,
        r#"select ended,
       data,
       last_played,
       winner,
       moves_needed,
       (
           select jsonb_agg(json_build_array(u.nick, user_id))
           from games_to_users
                    join users u on games_to_users.user_id = u.id
           where game_id = $1
       ) players
from games
where id = $1"#,
        id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(game) => resp_200_Ok_json!(&game),
        Err(sqlx::Error::RowNotFound) => resp_400_BadReq!("Game doesn't exists"),
        Err(_) => resp_500_IntSerErr!(),
    }
}
