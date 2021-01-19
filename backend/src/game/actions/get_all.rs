use crate::*;
use actix_web::{
    web::Data,
    Responder,
};
use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(Serialize)]
struct Game {
    name: String,
    id: uuid::Uuid,
    players: Option<serde_json::value::Value>,
    ended: bool,
    winner: Option<uuid::Uuid>,
}

pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(
        Game,
        "select id, name, ended, winner, (select jsonb_agg(jsonb_build_array(nick, id)) as players from users where players_id ? id::text group by true) from (select id, name, ended, winner, a.users as players_id from (select game_id, jsonb_agg(user_id) as users from games_to_users group by game_id) a join games on games.id = a.game_id) b",
        )
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(games) => resp_200_Ok_json!(&games),
        Err(_) => resp_500_IntSerErr!()
    }
}
