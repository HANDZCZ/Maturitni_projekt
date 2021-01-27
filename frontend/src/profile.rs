pub struct Profile {
    props: Props,
    games: Vec<Game>,
    user: User,
    link: ComponentLink<Self>,
    ft: Option<FetchTask>,
}
use crate::base::{ActiveNav, Base};
use crate::games::Game;
use crate::notifications::*;
use crate::{AppRoute, UserInfo};
use serde::Deserialize;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchService, FetchTask, Request, Response};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct User {
    pub nick: String,
    pub created_at: time::PrimitiveDateTime,
    pub gender: String,
    pub description: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            description: String::new(),
            created_at: time::PrimitiveDateTime::parse("2021-01-01 00:00:00", "%F %T").unwrap(),
            gender: String::new(),
            nick: String::new(),
        }
    }
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub user_id: String,
    pub model_callback: Callback<crate::Msg>,
}

pub enum Msg {
    Get,
    GetFailed,
    Got(String),
}

impl Component for Profile {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Get);
        Self {
            props,
            link,
            games: Vec::new(),
            user: User::default(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Got(data) => {
                self.ft = None;
                #[derive(Deserialize)]
                struct Data {
                    user: User,
                    games: Vec<Game>,
                }
                let Data { user, games } = serde_json::from_str(&data).unwrap();
                self.user = user;
                self.games = games;
                self.games.sort_by_key(|game| game.ended);
                notification(
                    "Načítání profilu úspěšné".to_owned(),
                    Position::BottomLeft,
                    Status::Success,
                    None,
                );
                true
            }
            Msg::GetFailed => {
                self.ft = None;
                notification(
                    "Načítání profilu selhalo".to_owned(),
                    Position::BottomLeft,
                    Status::Danger,
                    None,
                );
                false
            }
            Msg::Get => {
                notification(
                    "Načítání profilu".to_owned(),
                    Position::BottomLeft,
                    Status::Primary,
                    None,
                );
                let req = Request::get(format!(
                    "{}/user/profile/{}",
                    crate::DOMAIN,
                    self.props.user_id
                ))
                .body(Nothing)
                .unwrap();
                let options = FetchOptions {
                    credentials: Some(yew::web_sys::RequestCredentials::Include),
                    ..FetchOptions::default()
                };
                self.ft = Some(
                    FetchService::fetch_with_options(
                        req,
                        options,
                        self.link.callback(|response: Response<Result<String, _>>| {
                            let (meta, body) = response.into_parts();
                            match meta.status.as_u16() {
                                200 => {
                                    if let Ok(body) = body {
                                        Msg::Got(body)
                                    } else {
                                        notification(
                                            "Server neposlal žádnou odpověď".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                        Msg::GetFailed
                                    }
                                }
                                400 => {
                                    if let Ok(body) = body {
                                        notification(
                                            body,
                                            Position::BottomLeft,
                                            Status::Danger,
                                            None,
                                        );
                                    } else {
                                        notification(
                                            "Server neposlal žádnou chybovou hlášku.".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                    }
                                    Msg::GetFailed
                                }
                                500 => {
                                    notification(
                                        "Nastala chyba serveru".to_owned(),
                                        Position::BottomLeft,
                                        Status::Warning,
                                        None,
                                    );
                                    Msg::GetFailed
                                }
                                _ => {
                                    notification(
                                        "Nastala neimplementovaná chyba".to_owned(),
                                        Position::BottomLeft,
                                        Status::Warning,
                                        None,
                                    );
                                    Msg::GetFailed
                                }
                            }
                        }),
                    )
                    .unwrap(),
                );
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props.user_info != props.user_info;
        let ch2 = self.props.user_id != props.user_id;
        if changed || ch2 {
            if ch2 {
                self.user = User::default();
                self.games = Vec::new();
                self.link.send_message(Msg::Get);
            }
            self.props = props;
        }
        changed || ch2
    }

    fn view(&self) -> Html {
        let (victories, losses, ties): (u32, u32, u32) =
            self.games
                .iter()
                .fold((0, 0, 0), |(victories, losses, ties), game| {
                    if !game.ended {
                        (victories, losses, ties)
                    } else if let Some(winner) = &game.winner {
                        if *winner == self.props.user_id {
                            (victories + 1, losses, ties)
                        } else {
                            (victories, losses + 1, ties)
                        }
                    } else {
                        (victories, losses, ties + 1)
                    }
                });
        html! {
            <Base user_info=&self.props.user_info active_nav={
                if let Some(user_info) = &self.props.user_info {
                    if user_info.uuid == self.props.user_id {
                        Some(ActiveNav::Profile)
                    } else { None }
                } else { None }
            } background_image="jonas-elia-x6HHgq2zDvI-unsplash.jpg" model_callback=self.props.model_callback.clone()>
        <div class="uk-container uk-margin-medium-top">
            <div class="uk-card uk-card-secondary">
                <div class="uk-card-header">
                    <div class="uk-grid-small uk-flex-middle" uk-grid="">
                        <div class="uk-width-auto">
                            <span uk-icon="icon: user; ratio: 5"></span>
                        </div>
                        <div class="uk-width-expand">
                            <h3 class="uk-card-title uk-margin-remove-bottom">{ &self.user.nick }</h3>
                            <p class="uk-text-meta uk-margin-remove-top">
                                { "Registrace: "}{ &self.user.created_at.format("%F %T") }<br/>
                                { "Pohlaví: " }{ &self.user.gender }<br/>
                                { "UUID: " }{ &self.props.user_id }
                            </p>
                        </div>
                    </div>
                </div>
                <div class="uk-card-body">
                    { &self.user.description }
                </div>
                <div class="uk-card-footer">
                    <p>{ "Winrate: " }{ format!("{:.2}", victories as f32 / losses as f32) }</p>
                    <div class="uk-flex uk-flex-wrap">
                        <a class="uk-width-1-1@s uk-width-1-6@m uk-button uk-button-default">{ "Výhry: " }{ victories }</a>
                        <a class="uk-width-1-1@s uk-width-1-6@m uk-button uk-button-default">{ "Remízy: " }{ ties }</a>
                        <a class="uk-width-1-1@s uk-width-1-6@m uk-button uk-button-default">{ "Prohry: " }{ losses }</a>
                        {
                            if let Some(user_info) = &self.props.user_info {
                                if user_info.is_admin() || self.props.user_id == user_info.uuid {
                                    html! {
                                        <a class="uk-margin-auto-left uk-width-1-1@s uk-width-1-6@m uk-button uk-button-danger">
                                            <RouterAnchor<AppRoute> route=AppRoute::Edit(self.props.user_id.clone())>
                                                { "Upravit účet" }
                                            </RouterAnchor<AppRoute>>
                                        </a>
                                    }
                                } else { html! {} }
                            } else { html! {} }
                        }
                    </div>
                </div>
            </div>
        </div>

        <div class="uk-container uk-padding-large">
            <div class="uk-child-width-1-3@l uk-child-width-1-2@m uk-child-width-1-1@s uk-text-center uk-flex-center"
                uk-grid="masonry: false">
                { for self.games.iter().map(Game::view) }
            </div>
        </div>
            </Base>
        }
    }
}
