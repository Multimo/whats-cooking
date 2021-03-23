use yew::{html, Children, Component, ComponentLink, Html, Properties, ShouldRender};

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
          </div>
        }
    }
}
