pub struct Invites {
    props: Props,
    link: ComponentLink<Self>,
    invites: Vec<Invite>,
}
use crate::base::ActiveNav;
use crate::base::Base;
use crate::notifications::*;
use crate::{UserInfo, AppRoute};
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    Accept(String),
    Decline(String),
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
        Self {
            props,
            link,
            //invites: Vec::new(),
            invites: vec![
                Invite { uuid:"111".to_owned(), name: "Hra1".into(), moves_needed: 1 },
                Invite { uuid:"222".to_owned(), name: "Hra2".into(), moves_needed: 2 },
                Invite { uuid:"333".to_owned(), name: "Hra3".into(), moves_needed: 3 },
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Decline(uuid) => {
                notification(
                    format!("Decline: {}", uuid),
                    Position::BottomLeft,
                    Status::Success,
                    None,
                );
                false
            }
            Msg::Accept(uuid) => {
                notification(
                    format!("Accept: {}", uuid),
                    Position::BottomLeft,
                    Status::Success,
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
            <Base user_info=&self.props.user_info active_nav=ActiveNav::Invites background_image="" model_callback=self.props.model_callback.clone()>
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

struct Invite {
    uuid: String,
    name: String,
    moves_needed: u8,
}

impl Invite {
    fn view(&self, callback: Callback<Msg>) -> Html {
        let accept = callback.reform({
            let uuid = self.uuid.clone();
            move |_| Msg::Accept(uuid.clone())});
        let decline = callback.reform({
            let uuid = self.uuid.clone();
            move |_| Msg::Decline(uuid.clone())
        });
        html! {
                        <li>
                            <div class="uk-grid">
                                <p class="uk-width-expand">{ &self.name }<br/>{ format!("Tahů k vítěství: {}", self.moves_needed) }</p>
                                <p class="uk-text-muted">{ format!("UUID: {}", &self.uuid) }</p>
                                <a uk-icon="check" style="color: green;" onclick=accept>
        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="check"><polyline fill="none" stroke="#000" stroke-width="1.1" points="4,10 8,15 17,4"></polyline></svg>
                                </a>
                                <a uk-icon="close" style="color: red;" onclick=decline>
        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="close"><path fill="none" stroke="#000" stroke-width="1.06" d="M16,16 L4,4"></path><path fill="none" stroke="#000" stroke-width="1.06" d="M16,4 L4,16"></path></svg>
                                </a>
                            </div>
                        </li>
                }
    }
}
