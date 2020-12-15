use crate::game::model::GAME_NAME_REGEX;
use crate::session::{AdminUser, UserWithoutBan};
use crate::user::model::Role;
use crate::*;
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use rand::seq::SliceRandom;
use serde::Deserialize;
use sqlx::{PgPool, query};

#[derive(Deserialize)]
pub struct NewGameData {
    name: String,
    users_id: Vec<uuid::Uuid>,
    moves_needed: u8,
}

async fn process(mut data: NewGameData, pool: &PgPool, check_ban: bool) -> HttpResponse {
    {
        data.users_id.sort();
        let orig_len = data.users_id.len();
        data.users_id.dedup();
        if orig_len != data.users_id.len() {
            return resp_400_BadReq!("Your id list contains duplicates");
        }
    }

    if check_ban {
        let futs = data
            .users_id
            .iter()
            .map(|user_id| {
                async move {
                    (query!("select exists(select * from roles_to_users where user_id = $1 and role_id = $2) as banned",
                        *user_id,
                        Role::Banned as i32
                        )
                        .fetch_one(pool).await,
                    *user_id)
                }
            })
            .collect::<Vec<_>>();
        let res = futures::future::join_all(futs).await;
        for res in res {
            match res {
                (Ok(row), user_id) if row.banned == Some(true) => {
                    return resp_400_BadReq!(format!("User with id '{}' is banned", user_id))
                }
                (Ok(_), _) => {}
                (Err(_), _) => return resp_500_IntSerErr!(),
            }
        }
    }

    let mut rng = rand::thread_rng();
    let last_played = data.users_id.choose(&mut rng).unwrap();

    match query!(
        "call new_game_request($1, $2, $3, $4)",
        data.name,
        *last_played,
        data.users_id.as_slice(),
        data.moves_needed as i16
    )
    .execute(pool)
    .await
    {
        Ok(_) => resp_200_Ok!("Ok"),
        Err(sqlx::Error::Database(_)) => resp_400_BadReq!("Some user or users doesn't exists"),
        Err(_) => resp_500_IntSerErr!(),
    }
}

pub async fn new(
    user: UserWithoutBan,
    pool: Data<PgPool>,
    mut data: Json<NewGameData>,
) -> impl Responder {
    match data.moves_needed {
        3..=8 => {
            if GAME_NAME_REGEX.is_match(data.name.as_ref()).unwrap() {
                if data.users_id.contains(&user.id) {
                    return resp_400_BadReq!("Your id is automatically included");
                }
                match data.users_id.len() {
                    1..=4 => {
                        data.users_id.push(user.id);
                        process(data.into_inner(), pool.get_ref(), true).await
                    }
                    x if x > 4 => resp_400_BadReq!("Too much users"),
                    x if x < 1 => resp_400_BadReq!("You can't play only with yourself"),
                    _ => unreachable!("Impossible number..."),
                }
            } else {
                resp_400_BadReq!("Invalid game name")
            }
        },
        x if x > 8 => resp_400_BadReq!("Moves needed is too large."),
        x if x < 3 => resp_400_BadReq!("Moves needed is too small."),
        _ => unreachable!("Impossible number..."),
    }
}

pub async fn admin_new(
    _: AdminUser,
    pool: Data<PgPool>,
    data: Json<NewGameData>,
) -> impl Responder {
    match data.users_id.len() {
        2..=5 => process(data.into_inner(), pool.get_ref(), false).await,
        x if x < 2 => resp_400_BadReq!("At least 2 users are needed"),
        x if x > 5 => resp_400_BadReq!("Make more symbols and I will raise the limit"),
        _ => unreachable!("Impossible number..."),
    }
}
