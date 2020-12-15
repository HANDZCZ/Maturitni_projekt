use crate::user::model::Role;
use actix_session::Session;
use actix_web::{dev::Payload, error, web::Data, Error, FromRequest, HttpRequest};
use futures::Future;
use futures::TryStreamExt;
use std::pin::Pin;

#[derive(Debug)]
pub enum UserStatus {
    LoggedIn(uuid::Uuid),
    NotLoggedIn,
}

impl FromRequest for UserStatus {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let session = Session::from_request(req, payload);
        Box::pin(async move {
            match session.await {
                Ok(session) => match session.get::<uuid::Uuid>("id") {
                    Ok(Some(id)) => {
                        session.renew();
                        Ok(UserStatus::LoggedIn(id))
                    }
                    Ok(None) => Ok(UserStatus::NotLoggedIn),
                    Err(_) => {
                        log::error!(
                            "Redis error ({}:{}) - couldn't get id from user session",
                            file!(),
                            line!()
                        );
                        Err(error::ErrorInternalServerError(""))
                    }
                },
                Err(_) => {
                    log::error!(
                        "Redis error ({}:{}) - couldn't get user session",
                        file!(),
                        line!()
                    );
                    Err(error::ErrorInternalServerError(""))
                }
            }
        })
    }
}

#[derive(Debug)]
pub struct LoggedInUser(pub uuid::Uuid);

impl FromRequest for LoggedInUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let status = UserStatus::from_request(req, payload);
        Box::pin(async move {
            match status.await? {
                UserStatus::LoggedIn(id) => Ok(LoggedInUser(id)),
                UserStatus::NotLoggedIn => Err(error::ErrorUnauthorized("User not logged in")),
            }
        })
    }
}

#[derive(Debug)]
pub struct NotLoggedInUser;

impl FromRequest for NotLoggedInUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let status = UserStatus::from_request(req, payload);
        Box::pin(async move {
            match status.await? {
                UserStatus::NotLoggedIn => Ok(NotLoggedInUser),
                UserStatus::LoggedIn(_) => Err(error::ErrorBadRequest("User already logged in")),
            }
        })
    }
}

#[derive(Debug)]
pub struct UserWithRoles {
    pub id: uuid::Uuid,
    pub roles: Vec<Role>,
}

use crate::*;

impl FromRequest for UserWithRoles {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let user = LoggedInUser::from_request(req, payload);
        match req.app_data::<Data<sqlx::PgPool>>() {
            Some(pool) => {
                let pool = pool.get_ref().clone();
                Box::pin(async move {
                    struct Row {
                        role_id: Role,
                    }

                    let user_id = user.await?.0;
                    let roles: Result<Vec<Role>, Self::Error> = sqlx::query_as_unchecked!(
                        Row,
                        "select role_id from roles_to_users where user_id = $1",
                        user_id
                    )
                    .fetch(&pool)
                    .map_ok(|row| row.role_id)
                    .map_err(|err| match err {
                        sqlx::Error::Decode(err) => {
                            log::error!("Roles error ({}:{}) - {}", file!(), line!(), err);
                            error::ErrorInternalServerError("")
                        }
                        _ => actix_err_500_IntSerErr!(),
                    })
                    .try_collect()
                    .await;

                    Ok(UserWithRoles {
                        id: user_id,
                        roles: roles?,
                    })
                })
            }
            None => Box::pin(async {
                log::error!(
                    "Actix appdata error ({}:{}) - couldn't get pool",
                    file!(),
                    line!()
                );
                Err(error::ErrorInternalServerError(""))
            }),
        }
    }
}

impl UserWithRoles {
    #[inline]
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Role::Admin)
    }

    #[inline]
    pub fn is_banned(&self) -> bool {
        self.roles.contains(&Role::Banned)
    }
}

#[derive(Debug)]
pub struct AdminUser {
    pub id: uuid::Uuid,
    pub roles: Vec<Role>,
}

impl FromRequest for AdminUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let user_fut = UserWithRoles::from_request(req, payload);
        Box::pin(async move {
            let user = user_fut.await?;
            if user.is_admin() {
                Ok(AdminUser {
                    id: user.id,
                    roles: user.roles,
                })
            } else {
                Err(error::ErrorUnauthorized("Admin role needed"))
            }
        })
    }
}

#[derive(Debug)]
pub struct UserWithoutBan {
    pub id: uuid::Uuid,
    pub roles: Vec<Role>,
}

impl FromRequest for UserWithoutBan {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let user_fut = UserWithRoles::from_request(req, payload);
        Box::pin(async move {
            let user = user_fut.await?;
            if !user.is_banned() {
                Ok(UserWithoutBan {
                    id: user.id,
                    roles: user.roles,
                })
            } else {
                Err(error::ErrorUnauthorized(
                    "You can't perform this action while you are banned",
                ))
            }
        })
    }
}
