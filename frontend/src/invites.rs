pub struct Invites {
    props: Props,
    link: ComponentLink<Self>,
    invites: Vec<Invite>,

    get_fetching: bool,
    ft: HashMap<String, FetchTask>,
}
use crate::base::ActiveNav;
use crate::base::Base;
use crate::notifications::*;
use crate::{AppRoute, UserInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchService, FetchTask, Request, Response};
use yew_router::prelude::*;

pub enum Msg {
    UpdateInvite(String, bool),
    UpdateFailed(String),
    UpdateDone(String),
    GetInvites,
    FailedGetInvites,
    GotInvites(String),
}

#[derive(Serialize)]
struct UpdateData {
    id: String,
    accepted: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>,
}

impl Component for Invites {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetInvites);
        Self {
            props,
            link,
            invites: Vec::new(),
            get_fetching: true,
            ft: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateInvite(id, accepted) => {
                notification(
                    format!("Aktualizace pozvánky:<br>{}", id),
                    Position::BottomLeft,
                    Status::Primary,
                    None,
                );
                let req = Request::post(format!("{}/game/invite/update", crate::DOMAIN))
                    .header("Content-Type", "application/json")
                    .body(Ok(serde_json::to_string(&UpdateData {
                        id: id.clone(),
                        accepted,
                    })
                    .unwrap()))
                    .unwrap();
                let options = FetchOptions {
                    credentials: Some(yew::web_sys::RequestCredentials::Include),
                    ..FetchOptions::default()
                };
                let task = {
                    let id = id.clone();
                    let model_callback = self.props.model_callback.clone();
                    FetchService::fetch_with_options(
                        req,
                        options,
                        self.link
                            .callback(move |response: Response<Result<String, _>>| {
                                let (meta, body) = response.into_parts();
                                let status = meta.status.as_u16();
                                match status {
                                    200 => Msg::UpdateDone(id.clone()),
                                    400 | 401 => {
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
                                        if status == 401 {
                                            model_callback.emit(crate::Msg::LoggedOut);
                                        }
                                        Msg::UpdateFailed(id.clone())
                                    }
                                    500 => {
                                        notification(
                                            "Nastala chyba serveru".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                        Msg::UpdateFailed(id.clone())
                                    }
                                    _ => {
                                        notification(
                                            "Nastala neimplementovaná chyba".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                        Msg::UpdateFailed(id.clone())
                                    }
                                }
                            }),
                    )
                    .unwrap()
                };
                self.ft.insert(id, task);
                false
            }
            Msg::GetInvites => {
                notification(
                    "Získávání pozvánek".to_owned(),
                    Position::BottomLeft,
                    Status::Primary,
                    None,
                );
                let req = Request::get(format!("{}/game/invite/get", crate::DOMAIN))
                    .body(Nothing)
                    .unwrap();
                let options = FetchOptions {
                    credentials: Some(yew::web_sys::RequestCredentials::Include),
                    ..FetchOptions::default()
                };
                let model_callback = self.props.model_callback.clone();
                let task = FetchService::fetch_with_options(
                    req,
                    options,
                    self.link.callback(move |response: Response<Result<String, _>>| {
                        let (meta, body) = response.into_parts();
                        let status = meta.status.as_u16();
                        match status {
                            200 => {
                                if let Ok(body) = body {
                                    Msg::GotInvites(body)
                                } else {
                                    notification(
                                        "Server neposlal žádnou odpověď".to_owned(),
                                        Position::BottomLeft,
                                        Status::Warning,
                                        None,
                                    );
                                    Msg::FailedGetInvites
                                }
                            }
                            400 | 401 => {
                                if let Ok(body) = body {
                                    notification(body, Position::BottomLeft, Status::Danger, None);
                                } else {
                                    notification(
                                        "Server neposlal žádnou chybovou hlášku.".to_owned(),
                                        Position::BottomLeft,
                                        Status::Warning,
                                        None,
                                    );
                                }
                                if status == 401 {
                                    model_callback.emit(crate::Msg::LoggedOut);
                                }
                                Msg::FailedGetInvites
                            }
                            500 => {
                                notification(
                                    "Nastala chyba serveru".to_owned(),
                                    Position::BottomLeft,
                                    Status::Warning,
                                    None,
                                );
                                Msg::FailedGetInvites
                            }
                            _ => {
                                notification(
                                    "Nastala neimplementovaná chyba".to_owned(),
                                    Position::BottomLeft,
                                    Status::Warning,
                                    None,
                                );
                                Msg::FailedGetInvites
                            }
                        }
                    }),
                )
                .unwrap();
                self.ft.insert("Get".to_owned(), task);
                false
            }
            Msg::GotInvites(invites) => {
                self.get_fetching = false;
                self.ft.remove(&"Get".to_owned());
                self.invites = serde_json::from_str(&invites).unwrap();
                notification(
                    "Získávání pozvánek úspěšné".to_owned(),
                    Position::BottomLeft,
                    Status::Success,
                    None,
                );
                true
            }
            Msg::FailedGetInvites => {
                self.get_fetching = false;
                self.ft.remove(&"Get".to_owned());
                notification(
                    "Získávání pozvánek selhalo".to_owned(),
                    Position::BottomLeft,
                    Status::Danger,
                    None,
                );
                false
            }
            Msg::UpdateDone(id) => {
                self.invites.remove(
                    self.invites
                        .iter()
                        .position(|invite| invite.id == id)
                        .unwrap(),
                );
                self.ft.remove(&id);
                notification(
                    format!("Dokončena aktualizace pozvánky:<br>{}", id),
                    Position::BottomLeft,
                    Status::Success,
                    None,
                );
                true
            }
            Msg::UpdateFailed(id) => {
                self.ft.remove(&id);
                notification(
                    format!("Selhala aktualizace pozvánky:<br>{}", id),
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
        let callback = self.link.callback(|msg| msg);
        html! {
            <Base user_info=&self.props.user_info active_nav=ActiveNav::Invites background_image="pexels-sanaan-mazhar-3075993.jpg" model_callback=self.props.model_callback.clone()>
        <div class="uk-container uk-section-secondary uk-padding-large uk-margin-medium-top">
            <ul class="uk-list uk-list-divider uk-list-large">
                <li class="uk-nav-center">
                    <RouterAnchor<AppRoute> route=AppRoute::NewInvite>
                        <a class="uk-button uk-button-default">{ "Nová pozvánka" }</a>
                    </RouterAnchor<AppRoute>>
                </li>
            {
                for self.invites.iter().map(|i| i.view(callback.clone()))
            }
            </ul>
        </div>
            </Base>
        }
    }
}

#[derive(Deserialize)]
struct Invite {
    id: String,
    name: String,
    moves_needed: u8,
}

impl Invite {
    fn view(&self, callback: Callback<Msg>) -> Html {
        let accept = callback.reform({
            let id = self.id.clone();
            move |_| Msg::UpdateInvite(id.clone(), true)
        });
        let decline = callback.reform({
            let id = self.id.clone();
            move |_| Msg::UpdateInvite(id.clone(), false)
        });
        html! {
                        <li>
                            <div class="uk-grid">
                                <p class="uk-width-expand">{ &self.name }<br/>{ format!("Tahů k vítěství: {}", self.moves_needed) }</p>
                                <p class="uk-text-muted">{ format!("UUID: {}", &self.id) }</p>
                                <a uk-icon="check" style="color: green;" onclick=accept>
        <svg width="30" height="auto" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="check"><polyline fill="none" stroke="#000" stroke-width="1.1" points="4,10 8,15 17,4"></polyline></svg>
                                </a>
                                <a uk-icon="close" style="color: red;" onclick=decline>
        <svg width="30" height="auto" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="close"><path fill="none" stroke="#000" stroke-width="1.06" d="M16,16 L4,4"></path><path fill="none" stroke="#000" stroke-width="1.06" d="M16,4 L4,16"></path></svg>
                                </a>
                            </div>
                        </li>
                }
    }
}
