pub struct Base {
    props: Props,
    link: ComponentLink<Self>,
}
use crate::{AppRoute, UserInfo};
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub active_nav: Option<ActiveNav>,
    pub background_image: String,
    pub children: Children,
    pub model_callback: Callback<crate::Msg>
}

#[derive(Clone, PartialEq, Eq)]
pub enum ActiveNav {
    Profile,
    Games,
    Users,
    Login,
    Invites,
}

pub enum Msg {
    HamburgerMenuShow
}

impl Component for Base {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HamburgerMenuShow => {
                hamburger_menu_show();
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props.user_info != props.user_info
            || self.props.active_nav != props.active_nav
            || self.props.children != props.children;
            || self.props.background_image != props.background_image;
        if changed {
            self.props = props;
        }
        changed
    }

    fn view(&self) -> Html {
        let logout = self.props.model_callback.reform(|_| crate::Msg::Logout);
        let hamburger_menu_show = self.link.callback(|_| Msg::HamburgerMenuShow);
        html! {
        <>
            <div class="uk-background-fixed uk-height-viewport uk-background-cover uk-background-norepeat"
                style={ format!("{}{}{}", "background-image: url('/background_images/", self.props.background_image, "')") }>
                <div class="uk-offcanvas-content">
                    <nav class="uk-navbar-container uk-light" uk-navbar="">
                        <div class="uk-navbar-center">
                            <div class="uk-navbar-center-left">
                                <ul class="uk-navbar-nav uk-visible@s">
                                    <li class={
                                        if let Some(ActiveNav::Games) = &self.props.active_nav {
                                            "uk-active"
                                        } else {
                                            "uk-parent"
                                        }
                                    }>
                                    <RouterAnchor<AppRoute> route=AppRoute::Games>
                                        <span class="uk-margin-small-right" uk-icon="icon: grid">
                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="grid"><rect x="2" y="2" width="3" height="3"></rect><rect x="8" y="2" width="3" height="3"></rect><rect x="14" y="2" width="3" height="3"></rect><rect x="2" y="8" width="3" height="3"></rect><rect x="8" y="8" width="3" height="3"></rect><rect x="14" y="8" width="3" height="3"></rect><rect x="2" y="14" width="3" height="3"></rect><rect x="8" y="14" width="3" height="3"></rect><rect x="14" y="14" width="3" height="3"></rect></svg>
                                        </span>
                                        {"Hry"}
                                    </RouterAnchor<AppRoute>>
                                    </li>
                                    <li><a href="#"></a></li>
                                </ul>
                            </div>

                            <RouterAnchor<AppRoute> route=AppRoute::Root>
                                <a class="uk-navbar-item uk-logo">{"Piškvorky"}</a>
                            </RouterAnchor<AppRoute>>

                            <div class="uk-navbar-center-right">
                                <ul class="uk-navbar-nav uk-visible@s">
                                    <li><a href="#"></a></li>
                                        <li class={
                                            if let Some(ActiveNav::Users) = &self.props.active_nav {
                                                "uk-active"
                                            } else {
                                                "uk-parent"
                                            }
                                        }>
                                        <RouterAnchor<AppRoute> route=AppRoute::Users>
                                            <span class="uk-margin-small-right" uk-icon="icon: users">
                                            <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="users"><circle fill="none" stroke="#000" stroke-width="1.1" cx="7.7" cy="8.6" r="3.5"></circle><path fill="none" stroke="#000" stroke-width="1.1" d="M1,18.1 C1.7,14.6 4.4,12.1 7.6,12.1 C10.9,12.1 13.7,14.8 14.3,18.3"></path><path fill="none" stroke="#000" stroke-width="1.1" d="M11.4,4 C12.8,2.4 15.4,2.8 16.3,4.7 C17.2,6.6 15.7,8.9 13.6,8.9 C16.5,8.9 18.8,11.3 19.2,14.1"></path></svg>
                                            </span>
                                            {"Uživatelé"}
                                        </RouterAnchor<AppRoute>>
                                    </li>
                                </ul>
                            </div>
                        </div>

                        <div class="uk-navbar-right">
                            <ul class="uk-navbar-nav uk-visible@s">
                                <li class={
                                    if let Some(ActiveNav::Login) = &self.props.active_nav {
                                        "uk-active"
                                    } else {
                                        "uk-parent"
                                    }
                                }>
                                {
                                    if let Some(user_info) = &self.props.user_info {
                                        html!{
                                        <>
                                        <a>
                                            <span class="uk-margin-small-right" uk-icon="icon: user">
                                            <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="user"><circle fill="none" stroke="#000" stroke-width="1.1" cx="9.9" cy="6.4" r="4.4"></circle><path fill="none" stroke="#000" stroke-width="1.1" d="M1.5,19 C2.3,14.5 5.8,11.2 10,11.2 C14.2,11.2 17.7,14.6 18.5,19.2"></path></svg>
                                            </span>
                                            { user_info.nick.as_str() }
                                        </a>
                                        <div class="uk-light" uk-dropdown="">
                                            <ul class="uk-nav uk-dropdown-nav">
                                                <li class={
                                                    if let Some(ActiveNav::Profile) = &self.props.active_nav {
                                                        "uk-active"
                                                    } else {
                                                        ""
                                                    }
                                                }>
                                                    <RouterAnchor<AppRoute> route=AppRoute::Profile>
                                                        <span class="uk-margin-small-right" uk-icon="icon: cog">
                                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="cog"><circle fill="none" stroke="#000" cx="9.997" cy="10" r="3.31"></circle><path fill="none" stroke="#000" d="M18.488,12.285 L16.205,16.237 C15.322,15.496 14.185,15.281 13.303,15.791 C12.428,16.289 12.047,17.373 12.246,18.5 L7.735,18.5 C7.938,17.374 7.553,16.299 6.684,15.791 C5.801,15.27 4.655,15.492 3.773,16.237 L1.5,12.285 C2.573,11.871 3.317,10.999 3.317,9.991 C3.305,8.98 2.573,8.121 1.5,7.716 L3.765,3.784 C4.645,4.516 5.794,4.738 6.687,4.232 C7.555,3.722 7.939,2.637 7.735,1.5 L12.263,1.5 C12.072,2.637 12.441,3.71 13.314,4.22 C14.206,4.73 15.343,4.516 16.225,3.794 L18.487,7.714 C17.404,8.117 16.661,8.988 16.67,10.009 C16.672,11.018 17.415,11.88 18.488,12.285 L18.488,12.285 Z"></path></svg>
                                                        </span>
                                                        {"Profil"}
                                                    </RouterAnchor<AppRoute>>
                                                </li>
                                                <li class={
                                                    if let Some(ActiveNav::Invites) = &self.props.active_nav {
                                                        "uk-active"
                                                    } else {
                                                        ""
                                                    }
                                                }>
                                                    <RouterAnchor<AppRoute> route=AppRoute::Invites>
                                                        <span class="uk-margin-small-right" uk-icon="icon: bell">
                                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="bell"><path fill="none" stroke="#000" stroke-width="1.1" d="M17,15.5 L3,15.5 C2.99,14.61 3.79,13.34 4.1,12.51 C4.58,11.3 4.72,10.35 5.19,7.01 C5.54,4.53 5.89,3.2 7.28,2.16 C8.13,1.56 9.37,1.5 9.81,1.5 L9.96,1.5 C9.96,1.5 11.62,1.41 12.67,2.17 C14.08,3.2 14.42,4.54 14.77,7.02 C15.26,10.35 15.4,11.31 15.87,12.52 C16.2,13.34 17.01,14.61 17,15.5 L17,15.5 Z"></path><path fill="none" stroke="#000" d="M12.39,16 C12.39,17.37 11.35,18.43 9.91,18.43 C8.48,18.43 7.42,17.37 7.42,16"></path></svg>
                                                        </span>
                                                        {"Pozvánky"}
                                                    </RouterAnchor<AppRoute>>
                                                </li>
                                                <li>
                                                    <a onclick=logout.clone()>
                                                        <span class="uk-margin-small-right" uk-icon="icon: sign-out">
                                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="sign-out"><polygon points="13.1 13.4 12.5 12.8 15.28 10 8 10 8 9 15.28 9 12.5 6.2 13.1 5.62 17 9.5"></polygon><polygon points="13 2 3 2 3 17 13 17 13 16 4 16 4 3 13 3"></polygon></svg>
                                                        </span>
                                                        {"Odhlásit se"}
                                                    </a>
                                                </li>
                                            </ul>
                                        </div>
                                        </>
                                        }
                                    } else {
                                        html!{
                                            <RouterAnchor<AppRoute> route=AppRoute::Login>
                                                <span class="uk-margin-small-right" uk-icon="icon: sign-in">
                                                <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="sign-in"><polygon points="7 2 17 2 17 17 7 17 7 16 16 16 16 3 7 3"></polygon><polygon points="9.1 13.4 8.5 12.8 11.28 10 4 10 4 9 11.28 9 8.5 6.2 9.1 5.62 13 9.5"></polygon></svg>
                                                </span>
                                                {"Přihlásit se"}
                                            </RouterAnchor<AppRoute>>
                                        }
                                    }
                                }
                                </li>
                                <li><a href="#"></a></li>
                            </ul>
                            <ul class="uk-navbar-nav uk-hidden@s">
                                <li>
                                    <a class="uk-navbar-toggle" uk-navbar-toggle-icon="" onclick=hamburger_menu_show></a>
                                </li>
                            </ul>
                        </div>
                    </nav>

                    <div id="mobile-navbar" uk-offcanvas="mode: slide; flip: false;">
                        <div class="uk-offcanvas-bar">
                            <button class="uk-offcanvas-close" type="button" uk-close=""></button>
                            <ul class="uk-nav-default uk-nav-parent-icon" uk-nav="">
                                <li class="uk-text-center uk-padding-small">
                                    <RouterAnchor<AppRoute> route=AppRoute::Root>
                                        <a class="uk-logo">{"Piškvorky"}</a>
                                    </RouterAnchor<AppRoute>>
                                </li>
                                <li>
                                    <hr/>
                                </li>
                                <li class={
                                    if let Some(ActiveNav::Games) = &self.props.active_nav {
                                        "uk-active"
                                    } else {
                                        ""
                                    }
                                }>
                                    <RouterAnchor<AppRoute> route=AppRoute::Games>
                                        <span class="uk-margin-small-right" uk-icon="icon: grid">
                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="grid"><rect x="2" y="2" width="3" height="3"></rect><rect x="8" y="2" width="3" height="3"></rect><rect x="14" y="2" width="3" height="3"></rect><rect x="2" y="8" width="3" height="3"></rect><rect x="8" y="8" width="3" height="3"></rect><rect x="14" y="8" width="3" height="3"></rect><rect x="2" y="14" width="3" height="3"></rect><rect x="8" y="14" width="3" height="3"></rect><rect x="14" y="14" width="3" height="3"></rect></svg>
                                        </span>
                                        {"Hry"}
                                    </RouterAnchor<AppRoute>>
                                </li>
                                <li class={
                                    if let Some(ActiveNav::Users) = &self.props.active_nav {
                                        "uk-active"
                                    } else {
                                        ""
                                    }
                                }>
                                    <RouterAnchor<AppRoute> route=AppRoute::Users>
                                        <span class="uk-margin-small-right" uk-icon="icon: users">
                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="users"><circle fill="none" stroke="#000" stroke-width="1.1" cx="7.7" cy="8.6" r="3.5"></circle><path fill="none" stroke="#000" stroke-width="1.1" d="M1,18.1 C1.7,14.6 4.4,12.1 7.6,12.1 C10.9,12.1 13.7,14.8 14.3,18.3"></path><path fill="none" stroke="#000" stroke-width="1.1" d="M11.4,4 C12.8,2.4 15.4,2.8 16.3,4.7 C17.2,6.6 15.7,8.9 13.6,8.9 C16.5,8.9 18.8,11.3 19.2,14.1"></path></svg>
                                        </span>
                                        {"Uživatelé"}
                                    </RouterAnchor<AppRoute>>
                                </li>
                                <li><a href="#"></a></li>
                                <li>
                                    <hr/>
                                </li>
                                {
                                    if let Some(_) = &self.props.user_info {
                                        html!{
                                        <>
                                <li class={
                                    if let Some(ActiveNav::Profile) = &self.props.active_nav {
                                        "uk-active"
                                    } else {
                                        ""
                                    }
                                }>
                                    <RouterAnchor<AppRoute> route=AppRoute::Profile>
                                        <span class="uk-margin-small-right" uk-icon="icon: cog">
                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="cog"><circle fill="none" stroke="#000" cx="9.997" cy="10" r="3.31"></circle><path fill="none" stroke="#000" d="M18.488,12.285 L16.205,16.237 C15.322,15.496 14.185,15.281 13.303,15.791 C12.428,16.289 12.047,17.373 12.246,18.5 L7.735,18.5 C7.938,17.374 7.553,16.299 6.684,15.791 C5.801,15.27 4.655,15.492 3.773,16.237 L1.5,12.285 C2.573,11.871 3.317,10.999 3.317,9.991 C3.305,8.98 2.573,8.121 1.5,7.716 L3.765,3.784 C4.645,4.516 5.794,4.738 6.687,4.232 C7.555,3.722 7.939,2.637 7.735,1.5 L12.263,1.5 C12.072,2.637 12.441,3.71 13.314,4.22 C14.206,4.73 15.343,4.516 16.225,3.794 L18.487,7.714 C17.404,8.117 16.661,8.988 16.67,10.009 C16.672,11.018 17.415,11.88 18.488,12.285 L18.488,12.285 Z"></path></svg>
                                        </span>
                                        {"Profil"}
                                    </RouterAnchor<AppRoute>>
                                </li>
                                <li class={
                                    if let Some(ActiveNav::Invites) = &self.props.active_nav {
                                        "uk-active"
                                    } else {
                                        ""
                                    }
                                }>
                                    <RouterAnchor<AppRoute> route=AppRoute::Invites>
                                        <span class="uk-margin-small-right" uk-icon="icon: bell">
                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="bell"><path fill="none" stroke="#000" stroke-width="1.1" d="M17,15.5 L3,15.5 C2.99,14.61 3.79,13.34 4.1,12.51 C4.58,11.3 4.72,10.35 5.19,7.01 C5.54,4.53 5.89,3.2 7.28,2.16 C8.13,1.56 9.37,1.5 9.81,1.5 L9.96,1.5 C9.96,1.5 11.62,1.41 12.67,2.17 C14.08,3.2 14.42,4.54 14.77,7.02 C15.26,10.35 15.4,11.31 15.87,12.52 C16.2,13.34 17.01,14.61 17,15.5 L17,15.5 Z"></path><path fill="none" stroke="#000" d="M12.39,16 C12.39,17.37 11.35,18.43 9.91,18.43 C8.48,18.43 7.42,17.37 7.42,16"></path></svg>
                                        </span>
                                        {"Pozvánky"}
                                    </RouterAnchor<AppRoute>>
                                </li>
                                <li>
                                    <a onclick=logout>
                                        <span class="uk-margin-small-right" uk-icon="icon: sign-out">
                                        <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="sign-out"><polygon points="13.1 13.4 12.5 12.8 15.28 10 8 10 8 9 15.28 9 12.5 6.2 13.1 5.62 17 9.5"></polygon><polygon points="13 2 3 2 3 17 13 17 13 16 4 16 4 3 13 3"></polygon></svg>
                                        </span>
                                        {"Odhlásit se"}
                                    </a>
                                </li>
                                </>
                                        }
                                    } else {
                                        html!{
                                            <li class={
                                                if let Some(ActiveNav::Profile) = &self.props.active_nav {
                                                    "uk-active"
                                                } else {
                                                    ""
                                                }
                                            }>
                                                <RouterAnchor<AppRoute> route=AppRoute::Login>
                                                    <span class="uk-margin-small-right" uk-icon="icon: sign-in">
                                                    <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-svg="sign-in"><polygon points="7 2 17 2 17 17 7 17 7 16 16 16 16 3 7 3"></polygon><polygon points="9.1 13.4 8.5 12.8 11.28 10 4 10 4 9 11.28 9 8.5 6.2 9.1 5.62 13 9.5"></polygon></svg>
                                                    </span>
                                                    {"Přihlásit se"}
                                                </RouterAnchor<AppRoute>>
                                            </li>
                                        }
                                    }
                                }
                            </ul>
                        </div>
                    </div>
                </div>

                { self.props.children.clone() }

            </div>
        </>
        }
    }
}

#[wasm_bindgen(
    inline_js = "export function hamburger_menu_show() { let menu = document.querySelector(\"#mobile-navbar\"); UIkit.offcanvas(menu).hide();UIkit.offcanvas(menu).show(); }"
)]
extern "C" {
    fn hamburger_menu_show();
}
