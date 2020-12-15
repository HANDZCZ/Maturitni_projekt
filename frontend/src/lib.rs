#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};
use yew_router::switch::{AllowMissing, Permissive};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <header>
                    <nav class="menu">
                        <RouterAnchor<AppRoute> route=AppRoute::Admin(AdminRoute::Root)><a class={"waves-effect waves-light btn"}>{"admin panel"}</a></RouterAnchor<AppRoute>>
                    </nav>
                </header>
                <main>
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Admin(AdminRoute::Root) => html!{"Admin root"},
                                AppRoute::Admin(AdminRoute::Users) => html!{"Admin user dashboard"},
                                AppRoute::Root => html!{"Root bro"},
                                AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </main>
                <footer></footer>
            </>
        }
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/admin{*:rest}"]
    Admin(AdminRoute),
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Root,
}

#[derive(Debug, Switch, Clone)]
pub enum AdminRoute{
    #[to = "/users"]
    Users,
    #[to = "/"]
    Root,
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
