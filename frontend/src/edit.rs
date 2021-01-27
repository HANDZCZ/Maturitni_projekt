pub struct Edit {
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

    roles: Vec<crate::Role>,

    use_admin: bool,

    ft: Option<FetchTask>,
}
use crate::base::Base;
use crate::notifications::*;
use crate::UserInfo;
use serde::Serialize;
use std::string::ToString;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew::services::fetch::{FetchOptions, FetchService, FetchTask, Request, Response};

pub enum Msg {
    EmailChanged(String),
    PasswordChanged(String),
    NickChanged(String),
    GenderChanged(String),
    PasswordAgainChanged(String),
    DescriptionChanged(String),
    RoleChanged(crate::Role),
    Submit,
    Edit,
    EditFailed,
    Edited,
    UseAdminChanged,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub user_id: String,
    pub model_callback: Callback<crate::Msg>,
}

impl Component for Edit {
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
            use_admin: false,
            roles: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RoleChanged(role) => {
                if let Some(pos) = self.roles.iter().position(|r| *r == role) {
                    self.roles.remove(pos);
                } else {
                    self.roles.push(role);
                }
                true
            }
            Msg::UseAdminChanged => {
                self.use_admin = !self.use_admin;
                true
            }
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
                if !self.password_again_valid {
                    notification(
                        "Hesla se neschodují!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if self.use_admin {
                    self.link.send_message(Msg::Edit);
                } else if !self.email_valid {
                    notification(
                        "Email není validní!".to_owned(),
                        Position::BottomLeft,
                        Status::Danger,
                        None,
                    );
                } else if !self.password_valid {
                    notification("Heslo musí obsahovat:<br>2 malá písmena<br>2 velká písmena<br>2 číslice<br>2 speciální znaky".to_owned(), Position::BottomLeft, Status::Danger, None);
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
                    self.link.send_message(Msg::Edit);
                }
                false
            }
            Msg::Edit => {
                if let None = self.ft {
                    notification(
                        "Probíhá úprava".to_owned(),
                        Position::BottomLeft,
                        Status::Primary,
                        None,
                    );
                    #[derive(Serialize)]
                    struct Data {
                        id: String,
                        nick: Option<String>,
                        gender: Option<String>,
                        email: Option<String>,
                        password: Option<String>,
                        roles: Vec<crate::Role>,
                        description: Option<String>,
                    }

                    macro_rules! get_field {
                        ($name:ident) => {
                            if self.$name.is_empty() {
                                None
                            } else {
                                Some(self.$name.clone())
                            }
                        };
                    }

                    let data = serde_json::to_string(&Data {
                        id: self.props.user_id.clone(),
                        nick: get_field!(nick),
                        gender: get_field!(gender),
                        email: get_field!(email),
                        password: get_field!(password),
                        description: get_field!(description),
                        roles: self.roles.clone(),
                    })
                    .unwrap();
                    let req = Request::post(format!(
                        "{}/{}user/update",
                        crate::DOMAIN,
                        if self.use_admin
                            || if let Some(user_info) = &self.props.user_info {
                                self.props.user_id != user_info.uuid
                            } else {
                                false
                            }
                        {
                            "admin/"
                        } else {
                            ""
                        }
                    ))
                    .header("Content-Type", "application/json")
                    .body(Ok(data))
                    .unwrap();
                    let options = FetchOptions {
                        credentials: Some(yew::web_sys::RequestCredentials::Include),
                        ..FetchOptions::default()
                    };
                    let editing_self = if let Some(user_info) = &self.props.user_info {
                        self.props.user_id == user_info.uuid
                    } else {
                        false
                    };
                    let model_callback = self.props.model_callback.clone();
                    self.ft = Some(
                        FetchService::fetch_with_options(
                            req,
                            options,
                            self.link
                                .callback(move |response: Response<Result<String, _>>| {
                                    let (meta, body) = response.into_parts();
                                    match meta.status.as_u16() {
                                        200 => Msg::Edited,
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
                                            Msg::EditFailed
                                        }
                                        409 => {
                                            notification(
                                                "Email je již používán".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            Msg::EditFailed
                                        }
                                        410 => {
                                            notification(
                                                "Uživatel neexistuje".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            if editing_self {
                                                model_callback.emit(crate::Msg::LoggedOut);
                                            }
                                            Msg::EditFailed
                                        }
                                        500 => {
                                            notification(
                                                "Nastala chyba serveru".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            Msg::EditFailed
                                        }
                                        _ => {
                                            notification(
                                                "Nastala neimplementovaná chyba".to_owned(),
                                                Position::BottomLeft,
                                                Status::Warning,
                                                None,
                                            );
                                            Msg::EditFailed
                                        }
                                    }
                                }),
                        )
                        .unwrap(),
                    );
                } else {
                    notification(
                        "Úprava stále probíhá".to_owned(),
                        Position::BottomLeft,
                        Status::Warning,
                        None,
                    );
                }
                false
            }
            Msg::Edited => {
                self.ft = None;
                notification(
                    "Úprava byla úspěšná".to_owned(),
                    Position::BottomLeft,
                    Status::Success,
                    None,
                );
                false
            }
            Msg::EditFailed => {
                self.ft = None;
                notification(
                    "Úprava selhala".to_owned(),
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

        let (is_admin, enabled, editing_self) = if let Some(user_info) = &self.props.user_info {
            (
                user_info.is_admin(),
                self.props.user_id == user_info.uuid || user_info.is_admin(),
                self.props.user_id == user_info.uuid,
            )
        } else {
            (false, false, false)
        };

        let admin = if is_admin {
            let callback = self.link.callback(|_| Msg::UseAdminChanged);
            let inner = if editing_self {
                html! {
                    <>
                        <label><input class="uk-checkbox" type="checkbox" oninput=callback checked=self.use_admin/>{" Použít admin práva"}</label>
                        <p class="uk-text-muted uk-margin-remove-top">{"(Vypne skoro všechny kontroly, automaticky zapnuto při úpravě jiného uživatele.)"}</p>
                    </>
                }
            } else {
                html! {
                    <>
                        <label><input class="uk-checkbox" type="checkbox" oninput=callback checked=self.use_admin/>{" Vypnout kontrolu"}</label><br/>
                        <label><input class="uk-checkbox" type="checkbox" checked=true disabled=true/>{" Použít admin práva"}</label>
                        <p class="uk-text-muted uk-margin-remove-top">{"(Automaticky zapnuto, úprava jiného uživatele.)"}</p>
                    </>
                }
            };

            html! {
                <>
                <div class="uk-margin">
                    <label class="uk-form-label">{ "Role" }</label>
                    <div class="uk-grid-small uk-child-width-auto uk-grid">
                        {
                            for crate::Role::iter().map(|r| {
                                let role = r.clone();
                                let callback = self.link.callback(move |_| Msg::RoleChanged(role.clone()));
                                html! {
                                    <label><input class="uk-checkbox" type="checkbox" oninput=callback checked=self.roles.contains(&r)/>{" "}{r.to_string()}</label>
                                }
                            })
                        }
                    </div>
                    <p class="uk-text-muted uk-margin-remove-bottom uk-margin-small-top uk-text-center">{"! Pokud role nejsou vyplněny odstraní se všechny role uživatele !"}</p>
                    <p class="uk-text-muted uk-margin-remove-top uk-text-center">{"Platí jen při zapnutí admin práv."}</p>
                </div>
                <div class="uk-flex-center uk-margin uk-nav-center">
                    { inner }
                </div>
                </>
            }
        } else {
            html! { {""} }
        };
        html! {
            <Base user_info=&self.props.user_info active_nav=None background_image="pexels-aleksandar-pasaric-4201659.jpg" model_callback=self.props.model_callback.clone()>
        <div class="uk-container uk-section-secondary uk-padding-large uk-margin-medium-top uk-width-1-2@l">
            <article class="uk-article">
                <h1 class="uk-article-title uk-nav-center uk-margin-medium-bottom"> { "Upravit profil" }</h1>
                <p class="uk-nav-center uk-text-muted">{"Vynechaná pole nebudou změněna."}</p>
                <form class="uk-form-horizontal" onsubmit=submit>
                    <fieldset class="uk-fieldset">
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Nick" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.nick_valid || self.use_admin { "uk-input" } else { "uk-input uk-form-danger" } } type="text" oninput=nick value=&self.nick disabled=!enabled/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Pohlaví" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.gender_valid || self.use_admin { "uk-input" } else { "uk-input uk-form-danger" } } type="text" oninput=gender value=&self.gender disabled=!enabled/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Email" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.email_valid || self.use_admin { "uk-input" } else { "uk-input uk-form-danger" } } type="text" oninput=email value=&self.email disabled=!enabled/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Nové heslo" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.password_valid || self.use_admin { "uk-input" } else { "uk-input uk-form-danger" } } type="password" oninput=password value=&self.password disabled=!enabled/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Nové heslo znovu" }</label>
                            <div class="uk-form-controls">
                                <input class={ if self.password_again_valid { "uk-input" } else { "uk-input uk-form-danger" } } type="password" oninput=password_again value=&self.password_again disabled=!enabled/>
                            </div>
                        </div>
                        <div class="uk-margin">
                            <label class="uk-form-label">{ "Něco o vás" }</label>
                            <div class="uk-form-controls">
                                <textarea class={ if self.description_valid || self.use_admin { "uk-textarea" } else { "uk-textarea uk-form-danger" } } rows="5" style="resize: vertical;" oninput=description value=&self.description disabled=!enabled></textarea>
                            </div>
                        </div>
                        { admin }
                        <button type="submit" class="uk-button uk-button-default uk-align-center uk-margin-remove-bottom" disabled=!enabled>
                            { "Upravit" }
                        </button>
                    </fieldset>
                </form>
            </article>
        </div>
            </Base>
        }
    }
}
