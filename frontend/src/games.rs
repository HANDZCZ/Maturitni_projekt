pub struct Games(Props);
use crate::base::{ActiveNav, Base};
use crate::{AppRoute, UserInfo};
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>,
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
                    id: "6352e546-c998-4b99-9f52-e7c2946d6ba9".to_owned(),
                    players: vec![("Hráč 1".to_owned(),"78abfe73-674f-431f-8ffa-e9c28c467f16".to_owned()),("Hráč 2".to_owned(),"17dbcb0c-d741-49ec-ac44-60240c6bc275".to_owned()),("Hráč 3".to_owned(),"c9ae4750-143d-43fa-8017-83e154c0732e".to_owned())],
                    ended: true,
                    winner: Some("78abfe73-674f-431f-8ffa-e9c28c467f16".to_string())
                }.view() }
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
                        <p>{ "UUID: " }{ &self.id }</p>
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
