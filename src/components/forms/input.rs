use yew::{html, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

pub struct Input {
    value: String,
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    // #[prop_or_default]
    // pub children: Children,
    pub id: String,
    pub name: String,
    pub label: String,
    pub input_type: String,
    pub on_change: yew::Callback<yew::InputData>,
}

impl Component for Input {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            value: String::from(""),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex flex-col h-auto relative ">
                <label
                    for=&self.props.name
                    class="ml-2 mb-2 text-sm text-gray-600 dark:text-gray-400"
                >
                    {&self.props.label}
                </label>
                <input
                    id=&self.props.name
                    name=&self.props.name
                    class="px-2 py-2 border-2 rounded-md border-gray-200"
                    type="text"
                    value=&self.value
                    oninput=&self.props.on_change />
            </div>
        }
    }
}
