pub struct Users {
    props: Props,
    link: ComponentLink<Self>,
    users: Vec<User>,

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

impl Component for Users {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Get);
        Self {
            props,
            link,
            users: Vec::new(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Get => {
                if let None = self.ft {
                    notification(
                        "Načítám uživatele".to_owned(),
                        Position::BottomLeft,
                        Status::Primary,
                        None,
                    );
                    let req = Request::get(format!("{}/user/get_all", crate::DOMAIN))
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
                        "Načítání uživatelů stále probíhá".to_owned(),
                        Position::BottomLeft,
                        Status::Warning,
                        None,
                    );
                }
                false
            }
            Msg::Got(data) => {
                self.ft = None;
                self.users = serde_json::from_str(&data).unwrap();
                notification(
                    "Uživatelé načteni".to_owned(),
                    Position::BottomLeft,
                    Status::Success,
                    None,
                );
                true
            }
            Msg::Failed => {
                self.ft = None;
                notification(
                    "Načítání uživatelů selhalo".to_owned(),
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
        let admin = self
            .props
            .user_info
            .as_ref()
            .map_or(false, |u| u.is_admin());
        html! {
        <Base user_info=&self.props.user_info active_nav=ActiveNav::Users background_image="yong-chuan-tan-YlxMenahnB4-unsplash.jpg" model_callback=self.props.model_callback.clone()>
        <div class="uk-container uk-padding-large">
            <div class="uk-child-width-1-3@l uk-child-width-1-2@m uk-child-width-1-1@s uk-text-center uk-flex-center"
                uk-grid="masonry: false">
                { for self.users.iter().map(|u| u.view(admin)) }
            </div>
        </div>
        </Base>
        }
    }
}

#[derive(Deserialize)]
pub struct User {
    nick: String,
    id: String,
    created_at: time::PrimitiveDateTime,
    gender: String,
    victories: u32,
    losses: u32,
    ties: u32,
}

impl User {
    fn view(&self, admin: bool) -> Html {
        html! {
            <div>
                <div class="uk-card uk-card-secondary">
                    <div class="uk-card-body">
                        <h3 class="uk-card-title">
                            <RouterAnchor<AppRoute> route=AppRoute::Profile(self.id.clone())>
                                { &self.nick }
                            </RouterAnchor<AppRoute>>
                        </h3>
                        <p class="uk-text-meta uk-margin-remove">
                            { "Registrace: "}{ &self.created_at.format("%F %T") }<br/>
                            { "Pohlaví: " }{ &self.gender }<br/>
                            { "UUID: " }{ &self.id }
                        </p>
                        <p>{ "Winrate: " }{ format!("{:.2}", self.victories as f32 / self.losses as f32) }</p>
                        <div class="uk-flex uk-flex-column">
                            <a class="uk-button uk-button-default">{ "Výhry: " }{ self.victories }</a>
                            <a class="uk-button uk-button-default">{ "Remízy: " }{ self.ties }</a>
                            <a class="uk-button uk-button-default">{ "Prohry: " }{ self.losses }</a>
                            {
                                if admin {
                                    html! {
                                        <a class="uk-button uk-button-danger">
                                            <RouterAnchor<AppRoute> route=AppRoute::Edit(self.id.clone())>
                                                { "Upravit učet" }
                                            </RouterAnchor<AppRoute>>
                                        </a>
                                    }
                                } else { html! {} }
                            }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
