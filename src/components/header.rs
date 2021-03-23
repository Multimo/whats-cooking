use crate::routes::{AppAnchor, AppRoutes};
use wasm_bindgen::prelude::*;
use web_sys::Window;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Header {
    current_location: String,
}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let window = web_sys::window().expect("Unable to find window");
        let location = window
            .location()
            .pathname()
            .expect("Unable to get pathname from location");
        log::info!("found location {:?}", location);
        Self {
            current_location: location,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let active_classes = "mt-4";
        // self.current_location.contains("recipe") &&
        log::info!("in vuiew: {:?}", self.current_location);
        html! {
          <header class="container m-auto p-4 text-left flex flex-row justify-between items-center">
            <AppAnchor classes="mr-2" route=AppRoutes::Home >
                <h1 class="text-2xl text-secondary font-bold mb-1">{"Whats Cooking"}</h1>
            </AppAnchor>
            <nav>
                <AppAnchor classes=(format!("mr-2 {}", active_classes)) route=AppRoutes::RecipesPage >
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
