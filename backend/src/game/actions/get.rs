use crate::session::LoggedInUser;
use crate::*;
use actix_web::{
    web::Data, Responder,
};
use serde::Serialize;
use sqlx::{query_as, PgPool};

#[derive(Serialize)]
pub struct GameInvite {
    name: String,
    id: uuid::Uuid,
    moves_needed: i16,
}

pub async fn get(user: LoggedInUser, pool: Data<PgPool>) -> impl Responder {
    match query_as!(
        GameInvite,
        "select gr.name, gr.id, gr.moves_needed from users_to_game_requests join game_requests gr on users_to_game_requests.game_request_id = gr.id and accepted = false where user_id = $1",
        user.0
        )
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(invites) => resp_200_Ok_json!(&invites),
        Err(_) => resp_500_IntSerErr!(),
    }
}
