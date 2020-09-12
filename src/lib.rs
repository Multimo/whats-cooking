#![recursion_limit = "1024"]
use wasm_bindgen::prelude::*;
use web_sys::console;
use yew::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// // This is like the `main` function, except for JavaScript.
// #[wasm_bindgen(start)]
// pub fn main_js() -> Result<(), JsValue> {
//     // This provides better error messages in debug mode.
//     // It's disabled in release mode so it doesn't bloat up the file size.
//     #[cfg(debug_assertions)]
//     console_error_panic_hook::set_once();

//     // Your code goes here!
//     console::log_1(&JsValue::from_str("Hello world!"));

//     Ok(())
// }

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
    MinusOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::MinusOne => self.value -= 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
        <div>
            <h1>{"Whats Cooking"}</h1>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            <button onclick=self.link.callback(|_| Msg::MinusOne)>{ "-1" }</button>
            <p>{ self.value }</p>
            <div class="md:flex">
                <div class="md:flex-shrink-0">
                    <img class="rounded-lg md:w-56" src="https://images.unsplash.com/photo-1556740738-b6a63e27c4df?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=448&q=80" width="448" height="299" alt="Woman paying for a purchase" />
                </div>
                <div class="mt-4 md:mt-0 md:ml-6">
                    <div class="uppercase tracking-wide text-sm text-indigo-600 font-bold">{"Marketing"}</div>
                    <a href="#" class="block mt-1 text-lg leading-tight font-semibold text-gray-900 hover:underline">{"Finding customers for your new business"}</a>
                    <p class="mt-2 text-gray-600">{"Getting a new business off the ground is a lot of hard work. Here are five ideas you can use to find your first customers."}</p>
                </div>
            </div>
        </div>
                }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
