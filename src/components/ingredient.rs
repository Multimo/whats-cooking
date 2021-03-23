use yew::{html, Children, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Ingredient {
    props: Props,
    isOpen: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub name: String,
    pub group: String,
}

impl Component for Ingredient {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            isOpen: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => self.isOpen = !self.isOpen,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div class="container m-auto p-4" onclick=self.link.callback(|_| Msg::Toggle)>
            <h1>{&self.props.name}</h1>
            <p>{&self.props.group}</p>
            {match &self.isOpen {
                true => "Opened",
                false => "Closed"
            }}
          </div>
        }
    }
}
