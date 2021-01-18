use crate::session::LoggedInUser;
use crate::*;
use actix_web::{
    web::Data, Responder,
};
use serde::{Deserialize,Serialize};
use sqlx::{PgPool, query_as};

#[derive(Deserialize, Serialize)]
struct Invite {
    id: uuid::Uuid,
    name: String,
    moves_needed: i16,
}

pub async fn get(
    user: LoggedInUser,
    pool: Data<PgPool>,
) -> impl Responder {
    match query_as!(
        Invite,
        "select gr.name, gr.id, gr.moves_needed from users_to_game_requests join game_requests gr on users_to_game_requests.game_request_id = gr.id and accepted = false where user_id = $1",
        user.0
        )
        .fetch_all(pool.get_ref())
        .await 
        {
            Ok(invites) => resp_200_Ok_json!(&invites),
            Err(_) => resp_500_IntSerErr!()
        }
}
