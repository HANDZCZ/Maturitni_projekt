pub struct NewInvite {
    props: Props,
    link: ComponentLink<Self>,

    name: String,
    name_valid: bool,

    moves: u8,

    users: String,
    users_valid: bool,

    ft: Option<FetchTask>,
}
use crate::base::Base;
use crate::notifications::*;
use crate::UserInfo;
use serde::Serialize;
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchService, FetchTask, Request, Response};

pub enum Msg {
    NameChanged(String),
    MovesChanged(String),
    UsersChanged(String),
    Create,
    Created,
    Failed,
    Submit,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>,
}

impl Component for NewInvite {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            users: String::new(),
            users_valid: true,
            moves: 5,
            name: String::new(),
            name_valid: true,
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NameChanged(new) => {
                self.name = new;
                let new_valid = crate::regex::GAME_NAME_REGEX.is_match(&self.name).unwrap();
                let render = self.name_valid != new_valid;
                if render {
                    self.name_valid = new_valid;
                }
                render
            }
            Msg::MovesChanged(new) => {
                let res = new.parse::<u8>();
                if let Ok(val) = res {
                    let new_valid = val >= 3 && 8 >= val;
                    if new_valid {
                        self.moves = val;
                    }
                }
                true
            }
            Msg::UsersChanged(new) => {
                self.users = new;
                false
            }
            Msg::Submit => {
                if !self.name_valid {
                    notification(
                        "Jméno není validní!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if self.name.len() == 0 || self.users.len() == 0 {
                    notification(
                        "Všechna pole musí být vyplněma.".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else {
                    self.link.send_message(Msg::Create);
                }
                false
            }
            Msg::Created => {
                self.ft = None;
                false
            }
            Msg::Failed => {
                self.ft = None;
                false
            }
            Msg::Create => {
                if let None = self.ft {
                    notification(
                        "Vytvářim pozvánku".to_owned(),
                        Position::BottomLeft,
                        Status::Primary,
                        None,
                    );
                    #[derive(Serialize)]
                    struct Data {
                        name: String,
                        users_id: Vec<String>,
                        moves_needed: u8,
                    }
                    let data = serde_json::to_string(&Data {
                        name: self.name.clone(),
                        users_id: self.users.split('\n').map(|s| s.to_string()).collect(),
                        moves_needed: self.moves,
                    })
                    .unwrap();
                    let req = Request::post(format!("{}/game/invite/new", crate::DOMAIN))
                        .header("Content-Type", "application/json")
                        .body(Ok(data))
                        .unwrap();
                    let options = FetchOptions {
                        credentials: Some(yew::web_sys::RequestCredentials::Include),
                        ..FetchOptions::default()
                    };
                    let model_callback = self.props.model_callback.clone();
                    self.ft = Some(
                        FetchService::fetch_with_options(
                            req,
                            options,
                            self.link.callback(move |response: Response<Result<String, _>>| {
                                let (meta, body) = response.into_parts();
                                let status = meta.status.as_u16();
                                match status {
                                    200 => {
                                        notification(
                                            "Pozvánka vytvořena".to_owned(),
                                            Position::BottomLeft,
                                            Status::Success,
                                            None,
                                        );
                                        Msg::Created
                                    }
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
                                            notification(
                                                "Vytváření pozvánky selhalo".to_owned(),
                                                Position::BottomLeft,
                                                Status::Danger,
                                                None,
                                            );
                                        }
                                        if status == 401 {
                                            model_callback.emit(crate::Msg::LoggedOut);
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
                                        notification(
                                            "Vytváření pozvánky selhalo".to_owned(),
                                            Position::BottomLeft,
                                            Status::Danger,
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
                        "Vytváření pozvánky stále probíhá".to_owned(),
                        Position::BottomLeft,
                        Status::Warning,
                        None,
                    );
                }
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
        let name = self.link.callback(|e: InputData| Msg::NameChanged(e.value));
        let moves = self
            .link
            .callback(|e: InputData| Msg::MovesChanged(e.value));
        let users = self
            .link
            .callback(|e: InputData| Msg::UsersChanged(e.value));
        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });
        html! {
            <Base user_info=&self.props.user_info active_nav=None background_image="" model_callback=self.props.model_callback.clone()>
        <div class="uk-container uk-section-secondary uk-padding-large uk-margin-medium-top uk-width-1-2@l">
            <article class="uk-article">
                <h1 class="uk-article-title uk-nav-center uk-margin-medium-bottom">{ "Nová pozvánka" }</h1>
                <form class="uk-form-horizontal" onsubmit=submit>
                    <fieldset class="uk-fieldset">
                        <div class="uk-margin">
                            <label class="uk-form-label">{"Jméno"}</label>
                            <div class="uk-form-controls">
                                <input class={ if self.name_valid { "uk-input" } else { "uk-input uk-form-danger" } } type="text" oninput=name value=&self.name/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{"Počet tahů k vítězství"}</label>
                            <div class="uk-form-controls">
                                <p class="uk-text-center">{self.moves}</p>
                                <input class="uk-range" type="range" value=self.moves min="3" max="8" step="1" oninput=moves/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{"UUID uživatelů"}</label>
                            <div class="uk-form-controls">
                                <textarea class={ if self.users_valid { "uk-textarea" } else { "uk-textarea uk-form-danger" } } rows="5" style="resize: vertical;" oninput=users value=&self.users></textarea>
                            </div>
                        </div>
                        <button type="submit" class="uk-button uk-button-default uk-align-center uk-margin-remove-bottom">
                            {"Vytvořit"}
                        </button>
                    </fieldset>
                </form>
            </article>
        </div>
            </Base>
        }
    }
}
