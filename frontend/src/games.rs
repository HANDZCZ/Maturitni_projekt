pub struct Games {
    props: Props,
    link: ComponentLink<Self>,
    games: Vec<Game>,

    ft: Option<FetchTask>,
}

use crate::base::{ActiveNav, Base};
use crate::notifications::*;
use crate::{AppRoute, UserInfo};
use serde::Deserialize;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchService, FetchTask, Request, Response};
use yew_router::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>,
}

pub enum Msg {
    Get,
    Got(String),
    Failed,
}

impl Component for Games {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Get);
        Self {
            props,
            link,
            games: Vec::new(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Get => {
                if let None = self.ft {
                    notification(
                        "Načítám hry".to_owned(),
                        Position::BottomLeft,
                        Status::Primary,
                        None,
                    );
                    let req = Request::get(format!("{}/game/get_all", crate::DOMAIN))
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
                            self.link
                                .callback(move |response: Response<Result<String, _>>| {
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
                                                Msg::Failed
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
                                                    "Server neposlal žádnou chybovou hlášku."
                                                        .to_owned(),
                                                    Position::BottomLeft,
                                                    Status::Warning,
                                                    None,
                                                );
                                            }
                                            Msg::Failed
                                        }
                                        500 => {
                                            notification(
                                                "Nastala chyba serveru".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            Msg::Failed
                                        }
                                        _ => {
                                            notification(
                                                "Nastala neimplementovaná chyba".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            Msg::Failed
                                        }
                                    }
                                }),
                        )
                        .unwrap(),
                    );
                } else {
                    notification(
                        "Načítání her stále probíhá".to_owned(),
                        Position::BottomLeft,
                        Status::Warning,
                        None,
                    );
                }
                false
            }
            Msg::Got(data) => {
                self.ft = None;
                self.games = serde_json::from_str(&data).unwrap();
                notification(
                    "Hry načteny".to_owned(),
                    Position::BottomLeft,
                    Status::Success,
                    None,
                );
                true
            }
            Msg::Failed => {
                self.ft = None;
                notification(
                    "Načítání her selhalo".to_owned(),
                    Position::BottomLeft,
                    Status::Danger,
                    None,
                );
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props.user_info != props.user_info;
        if changed {
            self.props = props;
        }
        changed
    }

    fn view(&self) -> Html {
        html! {
            <Base user_info=&self.props.user_info active_nav=ActiveNav::Games background_image="toyamakanna-H_D-Kscai28-unsplash.jpg" model_callback=self.props.model_callback.clone()>
            <div class="uk-container uk-padding-large">
            <div class="uk-child-width-1-3@l uk-child-width-1-2@m uk-child-width-1-1@s uk-text-center uk-flex-center"
                uk-grid="masonry: true">
                { for self.games.iter().map(Game::view) }
            </div>
            </div>
            </Base>
        }
    }
}

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Game {
    pub name: String,
    pub id: String,
    pub players: Vec<(String, String)>,
    pub ended: bool,
    pub winner: Option<String>,
    pub moves_needed: u8,
}

impl Game {
    pub fn view(&self) -> Html {
        let status = if !self.ended {
            "Probíhá".to_owned()
        } else if let Some(winner) = &self.winner {
            format!(
                "{} je výherce",
                self.players
                    .iter()
                    .find(|(_name, id)| id == winner)
                    .unwrap()
                    .0
            )
        } else {
            "Remíza".to_owned()
        };
        html! {
            <div>
                <div class="uk-card uk-card-secondary">
                    <div class="uk-card-body">
                        <h3 class="uk-card-title">{ &self.name }</h3>
                        <p>{ "Tahů k vítěství: " }{ self.moves_needed }<br/>{ "UUID: " }{ &self.id }</p>
                        <h4>{ "Hráči" }</h4>
                        <ul class="uk-list uk-list-divider">
                            {
                                for self.players.iter().map(|(name, id)| html! { <li>
                                    <RouterAnchor<AppRoute> route=AppRoute::Profile(id.clone())>
                                        { name }
                                    </RouterAnchor<AppRoute>>
                                </li> })
                            }
                        </ul>
                    </div>
                    <div class="uk-card-footer">
                        <p>{ "Status: " }{ status }</p>
                        <a class="uk-button uk-button-default">{ "Přejít na hru" }</a>
                    </div>
                </div>
            </div>
        }
    }
}
