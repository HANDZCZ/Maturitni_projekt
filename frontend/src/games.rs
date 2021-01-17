pub struct Games(Props);
use yew::prelude::*;
use crate::base::{Base, ActiveNav};
use crate::UserInfo;

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>
}

impl Component for Games {
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
            <Base user_info=&self.0.user_info active_nav=ActiveNav::Games background_image="toyamakanna-H_D-Kscai28-unsplash.jpg" model_callback=self.0.model_callback.clone()>
            <div class="uk-container uk-padding-large">
            <div class="uk-child-width-1-3@l uk-child-width-1-2@m uk-child-width-1-1@s uk-text-center uk-flex-center"
                uk-grid="masonry: true">
                { Game {
                    name: "Hra".to_owned(),
                    uuid: "6352e546-c998-4b99-9f52-e7c2946d6ba9".to_owned(),
                    players: vec![("Hráč 1".to_owned(),"78abfe73-674f-431f-8ffa-e9c28c467f16".to_owned()),("Hráč 2".to_owned(),"17dbcb0c-d741-49ec-ac44-60240c6bc275".to_owned()),("Hráč 3".to_owned(),"c9ae4750-143d-43fa-8017-83e154c0732e".to_owned())],
                    status: "Hráč 1 vyhrál".to_owned()
                }.view() }
            </div>
            </div>
            </Base>
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Game {
    pub name: String,
    pub uuid: String,
    pub players: Vec<(String, String)>,
    pub status: String
}

impl Game {
    pub fn view(&self) -> Html {
        html! {
            <div>
                <div class="uk-card uk-card-secondary">
                    <div class="uk-card-body">
                        <h3 class="uk-card-title">{ &self.name }</h3>
                        <p>{ "UUID: " }{ &self.uuid }</p>
                        <h4>{ "Hráči" }</h4>
                        <ul class="uk-list uk-list-divider">
                            {
                                for self.players.iter().map(|(name, uuid)| html! { <li><a href="#">{ name }</a></li> })
                            }
                        </ul>
                    </div>
                    <div class="uk-card-footer">
                        <p>{ "Status: " }{ &self.status }</p>
                        <a href="#" class="uk-button uk-button-default">{ "Přejít na hru" }</a>
                    </div>
                </div>
            </div>
        }
    }
}
