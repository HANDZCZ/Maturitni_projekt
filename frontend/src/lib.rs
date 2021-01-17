#![recursion_limit = "4096"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::switch::{AllowMissing, Permissive};
use yew_router::{prelude::*, Switch};

const DOMAIN: &str = "http://api.mp.loc";

mod base;
mod games;
mod index;
mod users;
mod not_found;
mod profile;
mod login;
mod regex;
mod notifications;
mod register;
mod invites;
mod new_invite;
use new_invite::NewInvite;
use invites::Invites;
use register::Register;
use login::Login;
use profile::Profile;
use not_found::NotFound;
use games::Games;
use index::Index;
use users::Users;
use serde::Deserialize;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    user_info: Option<UserInfo>,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct UserInfo {
    nick: String,
    uuid: String,
    roles: Vec<Role>,
}

#[roles::get_roles_from_db]
#[derive(Clone, PartialEq, Eq, serde_repr::Deserialize_repr)]
#[repr(i16)]
pub enum Role {
    Admin,
    Banned,
}

impl UserInfo {
    #[inline]
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Role::Admin)
    }
}

pub enum Msg {
    LoggedIn(UserInfo),
    LoggedOut,
    Logout,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            user_info: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoggedOut => {
                self.user_info = None;
                true
            },
            Msg::LoggedIn(user_info) => {
                self.user_info = Some(user_info);
                true
            },
            Msg::Logout => {
                self.link.send_message(Msg::LoggedOut);
                false
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let user_info = self.user_info.clone();
        let model_callback = self.link.callback(|msg| msg);
        html! {
            <>
            <Router<AppRoute>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Admin(AdminRoute::Root) => html!{"Admin root"},
                        AppRoute::Admin(AdminRoute::Users) => html!{"Admin user dashboard"},
                        AppRoute::Root => html!{<Index user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::PageNotFound(Permissive(page)) => html!{<NotFound page=page user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::Profile => html!{<Profile user_info=user_info.clone() user=crate::profile::User {
                            nick: "Nickovič".to_owned(),
                            uuid: "6c44f19d-ad02-47ca-9db6-d51e4ae51764".to_owned(),
                            gender: "caveman".to_owned(),
                            created_at: "17.12.2020 17:18".to_owned(),
                            victories: 28,
                            losses: 19,
                            ties: 66,
                            description: "Popis nějaky...".to_owned(),
                        } games=vec![crate::games::Game {
                            name: "Hra 1".to_owned(),
                            uuid: "6352e546-c998-4b99-9f52-e7c2946d6ba9".to_owned(),
                            players: vec![("Hráč 1".to_owned(),"78abfe73-674f-431f-8ffa-e9c28c467f16".to_owned()),("Hráč 2".to_owned(),"17dbcb0c-d741-49ec-ac44-60240c6bc275".to_owned()),("Hráč 3".to_owned(),"c9ae4750-143d-43fa-8017-83e154c0732e".to_owned())],
                            status: "Hráč 2 vyhrál".to_owned()
                        }] model_callback=model_callback.clone()/> },
                        AppRoute::Users => html!{<Users user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::Games => html!{<Games user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::Login => html!{<Login user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::Register => html!{<Register user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::Invites => html!{<Invites user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::NewInvite => html!{<NewInvite user_info=user_info.clone() model_callback=model_callback.clone()/>},
                    }
                })
                redirect = Router::redirect(|route: Route| {
                    AppRoute::PageNotFound(Permissive(Some(route.route)))
                })
            />
            <button onclick=self.link.callback(|_| Msg::LoggedIn(UserInfo {
                nick: "Pepíno".to_owned(),
                uuid: "6c44f19d-ad02-47ca-9db6-d51e4ae51764".to_owned(),
                roles: vec![],
            }))>{ "LogUser" }</button>
            <button onclick=self.link.callback(|_| Msg::LoggedIn(UserInfo {
                nick: "Domino".to_owned(),
                uuid: "c9ae4750-143d-43fa-8017-83e154c0732e".to_owned(),
                roles: vec![],
            }))>{ "LogUser2" }</button>
            <button onclick=self.link.callback(|_| Msg::LoggedIn(UserInfo {
                nick: "Admino".to_owned(),
                uuid: "6c44f19d-ad02-47ca-9db6-d51e4ae51764".to_owned(),
                roles: vec![Role::Admin],
            }))>{ "LogAdmin" }</button>
            </>
        }
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/admin{*:rest}"]
    Admin(AdminRoute),
    #[to = "/profile!"]
    Profile,
    #[to = "/users!"]
    Users,
    #[to = "/games!"]
    Games,
    #[to = "/login!"]
    Login,
    #[to = "/register!"]
    Register,
    #[to = "/invites!"]
    Invites,
    #[to = "/new_invite!"]
    NewInvite,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Root,
}

#[derive(Debug, Switch, Clone)]
pub enum AdminRoute {
    #[to = "/users!"]
    Users,
    #[to = ""]
    Root,
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
