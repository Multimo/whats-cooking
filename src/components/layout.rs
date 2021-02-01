use crate::components::Header;
use yew::{html, Children, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Layout {
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Layout {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
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
            <Header />
            <main class="container m-auto p-4">
                { self.props.children.clone() }
            </main>
          </div>
        }
    }
}
