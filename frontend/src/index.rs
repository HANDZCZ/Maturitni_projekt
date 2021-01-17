pub struct Index(Props);
use yew::prelude::*;
use crate::base::Base;
use crate::UserInfo;

#[derive(Properties, Clone)]
pub struct Props {
    pub user_info: Option<UserInfo>,
    pub model_callback: Callback<crate::Msg>
}

impl Component for Index {
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
            <Base user_info=&self.0.user_info active_nav=None background_image="fabio-sasso-fFmCevlfWlo-unsplash.jpg" model_callback=self.0.model_callback.clone()>
            <div class="uk-container uk-section-secondary uk-padding-large uk-margin-medium-top">
                <article class="uk-article">
                    <h1 class="uk-article-title">{"Vítejte"}</h1>
                    <p class="uk-article-meta">
                        {"Pokračováním na jinou stránku automaticky souhlasíte s podmínkami a soubory cookies."}
                    </p>
                    <p>
                        <ol>
                            <li>
                                {"Vaše data můžou být prodána či sdílena třetím stranám."}
                            </li>
                            <li>
                                {"Vyhrazujeme si právo váš účet jakkoliv upravovat či smazat, bez uvedení důvodu."}
                            </li>
                        </ol>
                    </p>
                </article>
            </div>
            </Base>
        }
    }
}
