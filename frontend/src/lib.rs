#![recursion_limit = "4096"]
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew_router::switch::Permissive;
use yew_router::{prelude::*, Switch};

#[wasm_bindgen(inline_js = r#"export function get_domain() { return API_DOMAIN; }"#)]
extern "C" {
    fn get_domain() -> String;
}

lazy_static! {
    pub static ref DOMAIN: String = {
        let res = get_domain();
        if res.is_empty() {
            log::error!("Could not get api domain!");
            panic!();
        }
        res
    };
}
impl std::fmt::Display for DOMAIN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
const USER_INFO_KEY: &str = "user_info";

mod base;
mod edit;
mod game;
mod games;
mod index;
mod invites;
mod login;
mod new_invite;
mod not_found;
mod notifications;
mod profile;
mod regex;
mod register;
mod users;
use edit::Edit;
use game::Game;
use games::Games;
use index::Index;
use invites::Invites;
use login::Login;
use new_invite::NewInvite;
use not_found::NotFound;
use profile::Profile;
use register::Register;
use serde::{Deserialize, Serialize};
use users::Users;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    user_info: Option<UserInfo>,
    link: ComponentLink<Self>,
    storage: StorageService,
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserInfo {
    nick: String,
    uuid: String,
    roles: Vec<Role>,
}

#[roles::get_roles_from_db]
#[derive(
    Clone,
    PartialEq,
    Eq,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr,
    strum::EnumIter,
    strum::ToString,
)]
#[repr(i32)]
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

#[wasm_bindgen(
    inline_js = r#"export function clear_cookies() { document.cookie.split(";").forEach(c => { document.cookie = c.replace(/^ +/, "").replace(/=.*/, "=;expires=" + new Date().toUTCString() + ";path=/;domain=." + location.host); }); }"#
)]
extern "C" {
    fn clear_cookies();
}

#[wasm_bindgen(inline_js = r#"export function has_cookies() { return document.cookie != ""; }"#)]
extern "C" {
    fn has_cookies() -> bool;
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("Storage was disabled");
        let user_info = if has_cookies() {
            match storage.restore(USER_INFO_KEY) {
                Ok(raw_user_info) => {
                    serde_json::from_str::<UserInfo>(&raw_user_info).map_or(None, |data| Some(data))
                }
                _ => None,
            }
        } else {
            None
        };

        Self {
            user_info,
            link,
            storage,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoggedOut => {
                self.user_info = None;
                self.storage.remove(USER_INFO_KEY);
                clear_cookies();
                true
            }
            Msg::LoggedIn(user_info) => {
                self.storage.store(
                    USER_INFO_KEY,
                    Ok(serde_json::to_string(&user_info).unwrap()),
                );
                self.user_info = Some(user_info);
                true
            }
            Msg::Logout => {
                self.link.send_message(Msg::LoggedOut);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let user_info = self.user_info.clone();
        let model_callback = self.link.callback(|msg| msg);
        html! {
            <Router<AppRoute>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Root => html!{<Index user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::PageNotFound(Permissive(page)) => html!{<NotFound page=page user_info=user_info.clone() model_callback=model_callback.clone()/>},
                        AppRoute::Profile(id) => html!{<Profile user_info=user_info.clone() user_id=id model_callback=model_callback.clone()/> },
                        AppRoute::Edit(id) => html!{<Edit user_info=user_info.clone() user_id=id model_callback=model_callback.clone()/> },
                        AppRoute::Game(id) => html!{<Game user_info=user_info.clone() game_id=id model_callback=model_callback.clone()/> },
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
        }
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/profile/edit/{id}"]
    Edit(String),
    #[to = "/profile/{id}"]
    Profile(String),
    #[to = "/game/{id}"]
    Game(String),
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

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
    yew::start_app::<Model>();
}
