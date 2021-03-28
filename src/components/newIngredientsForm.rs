use serde::Deserialize;
use yew::{html, Children, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Deserialize, Debug)]
pub struct IIngredient {
    pub name: String,
    pub decription: Option<String>,
    pub food_group: Option<String>,
    pub name_scientific: Option<String>,
    pub food_subgroup: Option<String>,
}

pub struct NewIngredientsForm {}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for NewIngredientsForm {
    type Message = Msg;
    type Properties = Props;

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
          <div class="container m-auto p-4">
            <h1>{"hit ehre"}</h1>
            <button>{"Click me"}</button>
          </div>
        }
    }
}
