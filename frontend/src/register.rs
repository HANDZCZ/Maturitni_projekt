pub struct Register {
    props: Props,
    link: ComponentLink<Self>,

    email: String,
    email_valid: bool,

    password: String,
    password_valid: bool,

    password_again: String,
    password_again_valid: bool,

    nick: String,
    nick_valid: bool,

    description: String,
    description_valid: bool,

    gender: String,
    gender_valid: bool,

    ft: Option<FetchTask>,
}
use crate::base::Base;
use crate::notifications::*;
use crate::UserInfo;
use serde::Serialize;
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchService, FetchTask, Request, Response};

pub enum Msg {
    EmailChanged(String),
    PasswordChanged(String),
    NickChanged(String),
    GenderChanged(String),
    PasswordAgainChanged(String),
    DescriptionChanged(String),
    Submit,
    Register,
    RegisterFailed,
    Registred,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>,
}

impl Component for Register {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            email: String::new(),
            email_valid: true,
            password: String::new(),
            gender: String::new(),
            description: String::new(),
            nick: String::new(),
            password_again: String::new(),
            gender_valid: true,
            description_valid: true,
            nick_valid: true,
            password_again_valid: true,
            password_valid: true,
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EmailChanged(new) => {
                self.email = new;
                let new_valid = crate::regex::EMAIL_REGEX.is_match(&self.email).unwrap();
                let render = self.email_valid != new_valid;
                if render {
                    self.email_valid = new_valid;
                }
                render
            }
            Msg::PasswordChanged(new) => {
                self.password = new;
                let new_valid = crate::regex::PASSWORD_REGEX
                    .is_match(&self.password)
                    .unwrap();
                let render = self.password_valid != new_valid;
                if render {
                    self.password_valid = new_valid;
                }
                self.link
                    .send_message(Msg::PasswordAgainChanged(self.password_again.clone()));
                render
            }
            Msg::NickChanged(new) => {
                self.nick = new;
                let new_valid = crate::regex::NICK_REGEX.is_match(&self.nick).unwrap();
                let render = self.nick_valid != new_valid;
                if render {
                    self.nick_valid = new_valid;
                }
                render
            }
            Msg::GenderChanged(new) => {
                self.gender = new;
                let new_valid = crate::regex::GENDER_REGEX.is_match(&self.gender).unwrap();
                let render = self.gender_valid != new_valid;
                if render {
                    self.gender_valid = new_valid;
                }
                render
            }
            Msg::DescriptionChanged(new) => {
                self.description = new;
                let new_valid = crate::regex::DESCRIPTION_REGEX
                    .is_match(&self.description)
                    .unwrap();
                let render = self.description_valid != new_valid;
                if render {
                    self.description_valid = new_valid;
                }
                render
            }
            Msg::PasswordAgainChanged(new) => {
                self.password_again = new;
                let new_valid = self.password == self.password_again;
                let render = self.password_again_valid != new_valid;
                if render {
                    self.password_again_valid = new_valid;
                }
                render
            }
            Msg::Submit => {
                if !self.email_valid {
                    notification(
                        "Email není validní!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if !self.password_valid {
                    notification("Heslo musí obsahovat:<br>2 malá písmena<br>2 velká písmena<br>2 číslice<br>2 speciální znaky".to_owned(), Position::BottomLeft, Status::Danger, None);
                } else if !self.password_again_valid {
                    notification(
                        "Hesla se neschodují!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if self.nick.len() == 0
                    || self.email.len() == 0
                    || self.password.len() == 0
                    || self.gender.len() == 0
                    || self.password_again.len() == 0
                    || self.description.len() == 0
                {
                    notification(
                        "Všechna pole musí být vyplněma.".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if !self.nick_valid {
                    notification(
                        "Nick není validní!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if !self.gender_valid {
                    notification(
                        "Pohláví není validní!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if !self.description_valid {
                    notification(
                        "Popis není validní!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else {
                    self.link.send_message(Msg::Register);
                }
                false
            }
            Msg::Register => {
                if let None = self.ft {
                    notification(
                        "Probíhá registrace".to_owned(),
                        Position::BottomLeft,
                        Status::Primary,
                        None,
                    );
                    #[derive(Serialize)]
                    struct Data {
                        nick: String,
                        gender: String,
                        email: String,
                        password: String,
                        description: String,
                    }
                    let data = serde_json::to_string(&Data {
                        nick: self.nick.clone(),
                        gender: self.gender.clone(),
                        description: self.description.clone(),
                        email: self.email.clone(),
                        password: self.password.clone(),
                    })
                    .unwrap();
                    let req = Request::post(format!("{}/user/register", crate::DOMAIN))
                        .header("Content-Type", "application/json")
                        .body(Ok(data))
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
                                        notification(
                                            "Registrace byla úspěšná".to_owned(),
                                            Position::BottomLeft,
                                            Status::Success,
                                            None,
                                        );
                                        Msg::Registred
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
                                                "Registrace selhala".to_owned(),
                                                Position::BottomLeft,
                                                Status::Danger,
                                                None,
                                            );
                                        }
                                        Msg::RegisterFailed
                                    }
                                    409 => {
                                        notification(
                                            "Email je již používán.".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                        Msg::RegisterFailed
                                    }
                                    500 => {
                                        notification(
                                            "Nastala chyba serveru".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                        Msg::RegisterFailed
                                    }
                                    _ => {
                                        notification(
                                            "Nastala neimplementovaná chyba".to_owned(),
                                            Position::BottomLeft,
                                            Status::Warning,
                                            None,
                                        );
                                        notification(
                                            "Registrace selhala".to_owned(),
                                            Position::BottomLeft,
                                            Status::Danger,
                                            None,
                                        );
                                        Msg::RegisterFailed
                                    }
                                }
                            }),
                        )
                        .unwrap(),
                    );
                } else {
                    notification(
                        "Registrace stále probíhá".to_owned(),
                        Position::BottomLeft,
                        Status::Warning,
                        None,
                    );
                }
                false
            }
            Msg::Registred => {
                self.ft = None;
                false
            }
            Msg::RegisterFailed => {
                self.ft = None;
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
        let gender = self
            .link
            .callback(|e: InputData| Msg::GenderChanged(e.value));
        let nick = self.link.callback(|e: InputData| Msg::NickChanged(e.value));
        let email = self
            .link
            .callback(|e: InputData| Msg::EmailChanged(e.value));
        let password = self
            .link
            .callback(|e: InputData| Msg::PasswordChanged(e.value));
        let password_again = self
            .link
            .callback(|e: InputData| Msg::PasswordAgainChanged(e.value));
        let description = self
            .link
            .callback(|e: InputData| Msg::DescriptionChanged(e.value));
        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });
        html! {
            <Base user_info=&self.props.user_info active_nav=None background_image="pexels-felix-mittermeier-957002.jpg" model_callback=self.props.model_callback.clone()>
        <div class="uk-container uk-section-secondary uk-padding-large uk-margin-medium-top uk-width-1-2@l">
            <article class="uk-article">
                <h1 class="uk-article-title uk-nav-center uk-margin-medium-bottom"> { "Registrovat se" }</h1>
                <form class="uk-form-horizontal" onsubmit=submit>
                    <fieldset class="uk-fieldset">
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Nick" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.nick_valid { "uk-input" } else { "uk-input uk-form-danger" } } type="text" oninput=nick value=&self.nick/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Pohlaví" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.gender_valid { "uk-input" } else { "uk-input uk-form-danger" } } type="text" oninput=gender value=&self.gender/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Email" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.email_valid { "uk-input" } else { "uk-input uk-form-danger" } } type="text" oninput=email value=&self.email/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Heslo" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.password_valid { "uk-input" } else { "uk-input uk-form-danger" } } type="password" oninput=password value=&self.password/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Heslo znovu" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.password_again_valid { "uk-input" } else { "uk-input uk-form-danger" } } type="password" oninput=password_again value=&self.password_again/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Něco o vás" }</label>
                            <div class="uk-form-controls">
                                <textarea class={ if self.description_valid { "uk-textarea" } else { "uk-textarea uk-form-danger" } } rows="5" style="resize: vertical;" oninput=description value=&self.description></textarea>
                            </div>
                        </div>
                        <button type="submit" class="uk-button uk-button-default uk-align-center uk-margin-remove-bottom">
                            { "Registrovat" }
                        </button>
                    </fieldset>
                </form>
            </article>
        </div>
            </Base>
        }
    }
}
