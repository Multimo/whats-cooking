use crate::routes::{AppAnchor, AppRoutes};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <header class="container m-auto p-4 text-left flex flex-row justify-between items-center">
            <h1 class="text-2xl text-secondary font-bold mb-1">{"Whats Cooking"}</h1>
            <nav>
                <AppAnchor classes="mr-2" route=AppRoutes::RecipesPage >
                    {"Recipes"}
                </AppAnchor>
                <AppAnchor route=AppRoutes::IngredientsPage>
                    {"Ingredient"}
                </AppAnchor>
            </nav>
          </header>
        }
    }
}
