pub struct Profile(Props);
use yew::prelude::*;
use crate::base::{Base, ActiveNav};
use crate::UserInfo;
use crate::games::Game;

#[derive(Clone, PartialEq, Eq)]
pub struct User {
    pub nick: String,
    pub uuid: String,
    pub created_at: String,
    pub gender: String,
    pub victories: u32,
    pub losses: u32,
    pub ties: u32,
    pub description: String,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub user: User,
    pub games: Vec<Game>,
    pub model_callback: Callback<crate::Msg>
}

impl Component for Profile {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.0.user_info != props.user_info || self.0.user != props.user || self.0.games != props.games;
        if changed {
            self.0 = props;
        }
        changed
    }

    fn view(&self) -> Html {
        html! {
            <Base user_info=&self.0.user_info active_nav={ 
                if let Some(user_info) = &self.0.user_info {
                    if user_info.uuid == self.0.user.uuid {
                        Some(ActiveNav::Profile)
                    } else { None }
                } else { None }
            } background_image="jonas-elia-x6HHgq2zDvI-unsplash.jpg" model_callback=self.0.model_callback.clone()>
        <div class="uk-container uk-margin-medium-top">
            <div class="uk-card uk-card-secondary">
                <div class="uk-card-header">
                    <div class="uk-grid-small uk-flex-middle" uk-grid="">
                        <div class="uk-width-auto">
                            <span uk-icon="icon: user; ratio: 5"></span>
                        </div>
                        <div class="uk-width-expand">
                            <h3 class="uk-card-title uk-margin-remove-bottom">{ &self.0.user.nick }</h3>
                            <p class="uk-text-meta uk-margin-remove-top">
                                { "Registrace: "}{ &self.0.user.created_at }<br/>
                                { "Pohlaví: " }{ &self.0.user.gender }<br/>
                                { "UUID: " }{ &self.0.user.uuid }
                            </p>
                        </div>
                    </div>
                </div>
                <div class="uk-card-body">
                    { &self.0.user.description }
                </div>
                <div class="uk-card-footer">
                    <p>{ "Winrate: " }{ format!("{:.2}", self.0.user.victories as f32 / self.0.user.losses as f32) }</p>
                    <div class="uk-flex uk-flex-wrap">
                        <a href="#" class="uk-width-1-1@s uk-width-1-6@m uk-button uk-button-default">{ "Výhry: " }{ &self.0.user.victories }</a>
                        <a href="#" class="uk-width-1-1@s uk-width-1-6@m uk-button uk-button-default">{ "Remízy: " }{ &self.0.user.ties }</a>
                        <a href="#" class="uk-width-1-1@s uk-width-1-6@m uk-button uk-button-default">{ "Prohry: " }{ &self.0.user.losses }</a>
                        {
                            if let Some(user_info) = &self.0.user_info {
                                if user_info.is_admin() || self.0.user.uuid == user_info.uuid {
                                    html! { <a href="#" class="uk-margin-auto-left uk-width-1-1@s uk-width-1-6@m uk-button uk-button-danger">{ "Upravit učet" }</a> }
                                } else { html! {} }
                            } else { html! {} }
                        }
                    </div>
                </div>
            </div>
        </div>

        <div class="uk-container uk-padding-large">
            <div class="uk-child-width-1-3@l uk-child-width-1-2@m uk-child-width-1-1@s uk-text-center uk-flex-center"
                uk-grid="masonry: true">
                { for self.0.games.iter().map(Game::view) }
            </div>
        </div>
            </Base>
        }
    }
}
