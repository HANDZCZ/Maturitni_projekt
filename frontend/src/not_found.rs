pub struct NotFound(Props);
use crate::base::Base;
use crate::UserInfo;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub page: Option<String>,
    pub model_callback: Callback<crate::Msg>,
}

impl Component for NotFound {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.0.user_info != props.user_info || self.0.page != props.page;
        if changed {
            self.0 = props;
        }
        changed
    }

    fn view(&self) -> Html {
        html! {
            <Base user_info=&self.0.user_info active_nav=None background_image="alexander-slattery-LI748t0BK8w-unsplash.jpg" model_callback=self.0.model_callback.clone()>
            <div class="uk-container uk-section-secondary uk-padding-large uk-margin-medium-top">
                <article class="uk-article">
                    <h1 class="uk-article-title">{ "404" }</h1>
                    {
                        if let Some(page) = &self.0.page {
                            html! {
                                <p class="uk-article-meta">
                                    { page }
                                </p>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <p>{ "Stránka nenalezena" }</p>
                </article>
            </div>
            </Base>
        }
    }
}
