pub struct Users(Props);
use crate::base::{ActiveNav, Base};
use crate::UserInfo;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>
}

impl Component for Users {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.0.user_info != props.user_info;
        if changed {
            self.0 = props;
        }
        changed
    }

    fn view(&self) -> Html {
        html! {
        <Base user_info=&self.0.user_info active_nav=ActiveNav::Users background_image="yong-chuan-tan-YlxMenahnB4-unsplash.jpg" model_callback=self.0.model_callback.clone()>
        <div class="uk-container uk-padding-large">
            <div class="uk-child-width-1-3@l uk-child-width-1-2@m uk-child-width-1-1@s uk-text-center uk-flex-center"
                uk-grid="masonry: true">
                {
                    User{
                        nick: "Nick123".to_owned(),
                        uuid: "6c44f19d-ad02-47ca-9db6-d51e4ae51764".to_owned(),
                        gender: "caveman".to_owned(),
                        created_at: "17.12.2020 17:18".to_owned(),
                        victories: 17,
                        losses: 3,
                        ties: 5
                    }.view(match &self.0.user_info {
                        Some(user_info) => user_info.is_admin(),
                        None => false,
                    })
                }
            </div>
        </div>
        </Base>
        }
    }
}

pub struct User {
    nick: String,
    uuid: String,
    created_at: String,
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
                        <h3 class="uk-card-title">{ &self.nick }</h3>
                        <p class="uk-text-meta uk-margin-remove">
                            { "Registrace: "}{ &self.created_at }<br/>
                            { "Pohlaví: " }{ &self.gender }<br/>
                            { "UUID: " }{ &self.uuid }
                        </p>
                        <p>{ "Winrate: " }{ format!("{:.2}", self.victories as f32 / self.losses as f32) }</p>
                        <div class="uk-flex uk-flex-column">
                            <a href="#" class="uk-button uk-button-default">{ "Výhry: " }{ self.victories }</a>
                            <a href="#" class="uk-button uk-button-default">{ "Remízy: " }{ self.ties }</a>
                            <a href="#" class="uk-button uk-button-default">{ "Prohry: " }{ self.losses }</a>
                            {
                                if admin {
                                    html! {<a href="#" class="uk-button uk-button-danger">{ "Upravit učet" }</a>}
                                } else { html! {} }
                            }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
