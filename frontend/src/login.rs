pub struct Login {
    props: Props,
    link: ComponentLink<Self>,
    email: String,
    email_valid: bool,
    password: String,
    ft: Option<FetchTask>,
}
use crate::base::ActiveNav;
use crate::base::Base;
use crate::notifications::*;
use crate::{AppRoute, UserInfo};
use serde::Serialize;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew_router::prelude::*;

pub enum Msg {
    EmailChanged(String),
    PasswordChanged(String),
    Login,
    LoginFailed,
    LoggedIn(UserInfo),
    Submit,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            email: String::new(),
            email_valid: true,
            password: String::new(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoginFailed => {
                self.ft = None;
                false
            }
            Msg::LoggedIn(user_info) => {
                self.ft = None;
                self.props
                    .model_callback
                    .emit(crate::Msg::LoggedIn(user_info));
                false
            }
            Msg::EmailChanged(new_email) => {
                self.email = new_email;
                let new_email_valid = crate::regex::EMAIL_REGEX.is_match(&self.email).unwrap();
                let render = self.email_valid != new_email_valid;
                if render {
                    self.email_valid = new_email_valid;
                }
                render
            }
            Msg::PasswordChanged(new_password) => {
                self.password = new_password;
                false
            }
            Msg::Submit => {
                if !self.email_valid {
                    notification(
                        "Email není validní!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if self.password.len() == 0 || self.email.len() == 0 {
                    notification(
                        "Všechna pole musí být vyplněma.".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else {
                    self.link.send_message(Msg::Login);
                }
                false
            }
            Msg::Login => {
                if let None = self.ft {
                    notification(
                        "Probíhá přihlášení".to_owned(),
                        Position::BottomLeft,
                        Status::Primary,
                        None,
                    );
                    #[derive(Serialize)]
                    struct Data {
                        email: String,
                        password: String,
                    }
                    let data = serde_json::to_string(&Data {
                        email: self.email.clone(),
                        password: self.password.clone(),
                    })
                    .unwrap();
                    let req = Request::post(format!("{}/user/login", crate::DOMAIN))
                        .header("Content-Type", "application/json")
                        .body(Ok(data))
                        .unwrap();
                    self.ft = Some(
                        FetchService::fetch(
                            req,
                            self.link.callback(|response: Response<Result<String, _>>| {
                                let (meta, body) = response.into_parts();
                                match meta.status.as_u16() {
                                    200 => {
                                        if let Ok(body) = body {
                                            notification(
                                                "Přihlášení úspěšné".to_owned(),
                                                Position::BottomLeft,
                                                Status::Success,
                                                None,
                                            );
                                            Msg::LoggedIn(serde_json::from_str(&body).unwrap())
                                        } else {
                                            notification(
                                                "Server neposlal žádnou odpověď".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            notification(
                                                "Přihlášení selhalo".to_owned(),
                                                Position::BottomLeft,
                                                Status::Danger,
                                                None,
                                            );
                                            Msg::LoginFailed
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
                                            notification(
                                                "Přihlášení selhalo".to_owned(),
                                                Position::BottomLeft,
                                                Status::Danger,
                                                None,
                                            );
                                        }
                                        Msg::LoginFailed
                                    }
                                    500 => {
                                        notification(
                                            "Nastala chyba serveru".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                        Msg::LoginFailed
                                    }
                                    _ => {
                                        notification(
                                            "Nastala neimplementovaná chyba".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                        notification(
                                            "Přihlášení selhalo".to_owned(),
                                            Position::BottomLeft,
                                            Status::Danger,
                                            None,
                                        );
                                        Msg::LoginFailed
                                    }
                                }
                            }),
                        )
                        .unwrap(),
                    );
                } else {
                    notification(
                        "Přihlášení stále probíhá".to_owned(),
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
        let email = self
            .link
            .callback(|e: InputData| Msg::EmailChanged(e.value));
        let password = self
            .link
            .callback(|e: InputData| Msg::PasswordChanged(e.value));
        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });
        html! {
            <Base user_info=&self.props.user_info active_nav=ActiveNav::Login background_image="pexels-aleksejs-bergmanis-681335.jpg" model_callback=self.props.model_callback.clone()>
        <div class="uk-container uk-section-secondary uk-padding-large uk-margin-medium-top uk-width-1-2@l">
            <article class="uk-article">
                <h1 class="uk-article-title uk-nav-center uk-margin-medium-bottom">{ "Přihlásit se" }</h1>
                <form class="uk-form-horizontal" onsubmit=submit>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Email" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.email_valid { "uk-input" } else { "uk-input uk-form-danger" } } type="text" value=&self.email oninput=email/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Heslo" }</label>
                            <div class="uk-form-controls">
                                <input class="uk-input" type="password" value=&self.password oninput=password/>
                            </div>
                        </div>
                        <div class="uk-margin uk-nav-center">
                            <RouterAnchor<AppRoute> route=AppRoute::Register>
                                <a class="uk-link-muted">{ "Ještě nemám účet! "}</a>
                            </RouterAnchor<AppRoute>>
                        </div>
                        <button type="submit" class="uk-button uk-button-default uk-align-center uk-margin-remove-bottom">
                            { "Přihlásit" }
                        </button>
                </form>
            </article>
        </div>
            </Base>
        }
    }
}
