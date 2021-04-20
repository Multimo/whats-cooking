use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Modal {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub title: String,
    pub description: String,
    pub on_confirm: yew::Callback<yew::MouseEvent>,
    pub on_cancel: yew::Callback<yew::MouseEvent>,
}

impl Component for Modal {
    type Message = ();
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
        <div class="absolute inset-0 flex justify-center items-center w-screen h-screen bg-gradient-to-r from-yellow-400 via-red-500 to-pink-500">
            <div class="md:w-1/3 sm:w-full rounded-lg shadow-lg bg-white my-3 px-4 p-2">
                <div class="flex justify-between border-b border-gray-100 py-4">
                  <div>
                    <i class="fa fa-exclamation-triangle text-orange-500"></i>
                    <span class="font-bold text-gray-700 text-lg">{&self.props.title}</span>
                  </div>
                  <div>
                    <button>
                      <i class="fa fa-times-circle text-red-500 hover:text-red-600 transition duration-150"></i>
                    </button>
                  </div>
                </div>
                <div class="py-5 text-gray-600">
                  {&self.props.description}
                </div>
                <div class="py-4 flex justify-between">
                  <button onclick=&self.props.on_cancel class="text-sm py-2 px-3 text-gray-500 hover:text-gray-600 transition duration-150">
                  {"Cancel"}
                  </button>
                  <button onclick=&self.props.on_confirm class="bg-orange-500 mr-1 rounded text-sm py-2 px-3 text-white hover:bg-orange-600 transition duration-150">
                  {"Confirm"}
                  </button>
                </div>
            </div>
        </div>
        }
    }
}
