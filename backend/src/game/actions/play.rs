#![allow(clippy::blocks_in_if_conditions)]
use crate::game::model::{Croft, GameData};
use crate::session::LoggedInUser;
use crate::*;
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;
use sqlx::{query, PgPool};

#[derive(Deserialize)]
pub struct PlayGameData {
    game_id: uuid::Uuid,
    #[serde(flatten)]
    croft: Croft,
    won: Option<WonGameData>,
    tie: Option<bool>,
}

#[derive(Deserialize)]
enum WonDirection {
    Horizontal,
    Vertical,
    DiagonalLR,
    DiagonalRL,
}

#[derive(Deserialize)]
pub struct WonGameData {
    moves: Vec<Croft>,
    direction: WonDirection,
}

async fn process(data: PlayGameData, pool: &PgPool, user_id: uuid::Uuid) -> HttpResponse {
    if data.croft.x > 30 || data.croft.y > 30 {
        return resp_400_BadReq!("Playing feld is limited to 30x30");
    }

    match query!(
        "select user_id from games_to_users where game_id = $1 order by user_id",
        data.game_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(users) => {
            let users = users
                .into_iter()
                .map(|user_struct| user_struct.user_id)
                .collect::<Vec<uuid::Uuid>>();
            if !users.contains(&user_id) {
                return resp_401_Unauth!("You aren't in this game.");
            }

            match query!(
                "select last_played, data, ended, moves_needed from games where id = $1",
                data.game_id
            )
            .fetch_one(pool)
            .await
            {
                Ok(game) => {
                    if game.ended {
                        return resp_400_BadReq!("Game already ended.");
                    }

                    if users[{
                        let index = users
                            .iter()
                            .position(|&id| id == game.last_played)
                            .expect("Impossible")
                            + 1;
                        if index == users.len() {
                            0
                        } else {
                            index
                        }
                    }] == user_id
                    {
                        let mut game_data: GameData = bincode::deserialize(game.data.as_slice())
                            .expect("Should never happen.");
                        if !game_data.field.contains(&data.croft) {
                            game_data.field.push(data.croft);

                            if let Some(true) = data.tie {
                                if game_data.field.len() == 30 * 30 {
                                    match query!(
                                        "update games set data = $1, last_played = $2, ended = true where id = $3",
                                        bincode::serialize(&game_data).expect("Should never happen."),
                                        user_id,
                                        data.game_id
                                    )
                                    .execute(pool)
                                    .await
                                    {
                                        Ok(_) => resp_200_Ok!("Ok"),
                                        Err(_) => resp_500_IntSerErr!(),
                                    }
                                } else {
                                    return resp_400_BadReq!("Move is invalid - false tie.");
                                }
                            } else if let Some(WonGameData {
                                mut moves,
                                direction,
                            }) = data.won
                            {
                                moves.push(data.croft);
                                if moves.len() != game.moves_needed as usize {
                                    return resp_400_BadReq!(
                                        "Invalid move - false win, wrong amount crofts for win."
                                    );
                                } else if !match direction {
                                    WonDirection::Horizontal => {
                                        moves.iter().skip(1).all(|m| moves[0].y == m.y) && {
                                            moves.sort_by_cached_key(|m| m.x);
                                            moves
                                                .iter()
                                                .enumerate()
                                                .skip(1)
                                                .all(|(offset, m)| moves[0].x == m.x - offset as u8)
                                        }
                                    }
                                    WonDirection::Vertical => {
                                        moves.iter().skip(1).all(|m| moves[0].x == m.x) && {
                                            moves.sort_by_cached_key(|m| m.y);
                                            moves
                                                .iter()
                                                .enumerate()
                                                .skip(1)
                                                .all(|(offset, m)| moves[0].y == m.y - offset as u8)
                                        }
                                    }
                                    WonDirection::DiagonalLR => {
                                        moves.sort_by_cached_key(|m| m.x);
                                        moves.iter().enumerate().skip(1).all(|(offset, m)| {
                                            moves[0].x == m.x - offset as u8
                                                && moves[0].y == m.y - offset as u8
                                        })
                                    }
                                    WonDirection::DiagonalRL => {
                                        moves.sort_by_cached_key(|m| m.x);
                                        moves.iter().enumerate().skip(1).all(|(offset, m)| {
                                            moves[0].x == m.x - offset as u8
                                                && moves[0].y == m.y + offset as u8
                                        })
                                    }
                                } {
                                    return resp_400_BadReq!(
                                        "Invalid move - false win, moves don't add up to win."
                                    );
                                }

                                let c = game_data
                                    .field
                                    .iter()
                                    .rev()
                                    .step_by(users.len())
                                    .filter(|m| moves.contains(m))
                                    .count();
                                if moves.len() == c {
                                    match query!(
                                        "update games set data = $1, last_played = $2, ended = true, winner = $2 where id = $3",
                                        bincode::serialize(&game_data).expect("Should never happen."),
                                        user_id,
                                        data.game_id
                                    )
                                    .execute(pool)
                                    .await
                                    {
                                        Ok(_) => resp_200_Ok!("Ok"),
                                        Err(_) => resp_500_IntSerErr!(),
                                    }
                                } else {
                                    resp_400_BadReq!(
                                        "Move is invalid - false win, some moves aren't yours or some moves doesn't exists."
                                    )
                                }
                            } else {
                                match query!(
                                    "update games set data = $1, last_played = $2 where id = $3",
                                    bincode::serialize(&game_data).expect("Should never happen."),
                                    user_id,
                                    data.game_id
                                )
                                .execute(pool)
                                .await
                                {
                                    Ok(_) => resp_200_Ok!("Ok"),
                                    Err(_) => resp_500_IntSerErr!(),
                                }
                            }
                        } else {
                            resp_400_BadReq!("This croft was already used.")
                        }
                    } else {
                        resp_400_BadReq!("It's not your turn to play.")
                    }
                }
                Err(_) => return resp_500_IntSerErr!(),
            }
        }
        Err(sqlx::Error::Database(_)) => {
            return resp_400_BadReq!("Game with that id doesn't exists.")
        }
        Err(_) => return resp_500_IntSerErr!(),
    }
}

pub async fn play(
    user: LoggedInUser,
    pool: Data<PgPool>,
    data: Json<PlayGameData>,
) -> impl Responder {
    process(data.into_inner(), pool.get_ref(), user.0).await
}
