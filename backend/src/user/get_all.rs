use crate::*;
use actix_web::{
    web::Data,
    Responder,
};
use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(Serialize)]
struct User {
    id: uuid::Uuid,
    nick: String,
    created_at: time::PrimitiveDateTime,
    gender: String,
    ties: Option<i64>,
    victories: Option<i64>,
    losses: Option<i64>,
}

pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(
        User,
r#"select id,
       nick,
       created_at,
       gender,
       (select count(*) "ties"
        from games
                 join games_to_users on games.id = games_to_users.game_id
        where ended
          and user_id = users.id
          and winner is null),
       (select count(*) victories
        from games
                 join games_to_users on games.id = games_to_users.game_id
        where ended
          and user_id = users.id
          and winner = users.id),
       (select count(*) losses
        from games
                 join games_to_users on games.id = games_to_users.game_id
        where ended
          and user_id = users.id
          and winner != users.id)
from users"#,
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(users) => resp_200_Ok_json!(&users),
        Err(_) => resp_500_IntSerErr!(),
    }
}
