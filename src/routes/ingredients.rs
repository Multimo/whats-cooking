use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink, Html, ShouldRender,
};
#[derive(Deserialize, Debug)]
enum States {
    Initial,
    Fetching,
    Success,
    Error,
}

#[derive(Deserialize, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub name_scientific: Option<String>,
    pub decription: Option<String>,
    pub food_group: Option<String>,
    pub food_subgroup: Option<String>,
}

#[derive(Deserialize, Debug)]
struct IngredientResponse {
    response: Vec<Ingredient>,
}

// #[derive(Deserialize)]
pub struct IngredientsPage {
    state: States,
    ingredients: Option<Vec<Ingredient>>,
    error: Option<String>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    FetchIngredients,
    FetchIngredientsSuccess(Vec<Ingredient>),
    FetchIngredientsError(anyhow::Error),
}

impl Component for IngredientsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: States::Initial,
            error: None,
            ingredients: None,
            fetch_task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match self.state {
            States::Initial => match msg {
                Msg::FetchIngredients => {
                    // do the fetch
                    log::info!("getting location");
                    let request = Request::get("http://localhost:8082/ingredients")
                        .body(Nothing)
                        .expect("Could not build request.");
                    // 2. construct a callback
                    let callback = self.link.callback(
                        |response: Response<Json<Result<IngredientResponse, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            log::info!("got data {:?}", data);
                            match data {
                                Ok(data) => Msg::FetchIngredientsSuccess(data.response),
                                Err(error) => Msg::FetchIngredientsError(error),
                            }
                        },
                    );
                    // 3. pass the request and callback to the fetch service
                    let task =
                        FetchService::fetch(request, callback).expect("failed to start request");
                    // 4. store the task so it isn't canceled immediately
                    self.fetch_task = Some(task);
                    // we want to redraw so that the page displays a 'fetching...' message to the user
                    // so return 'true'
                    self.state = States::Fetching;
                    true
                }
                _ => false,
            },
            States::Fetching => match msg {
                Msg::FetchIngredientsSuccess(data) => {
                    log::info!("in success data {:?}", data);
                    self.ingredients = Some(data);
                    self.state = States::Success;
                    true
                }
                Msg::FetchIngredientsError(_error) => {
                    log::info!("got found error {:?}", _error);
                    let error = String::from("There was an error");
                    self.state = States::Error;
                    self.error = Some(error);
                    self.fetch_task = None;
                    true
                }
                _ => false,
            },
            States::Error => match msg {
                Msg::FetchIngredients => {
                    // do the fetch
                    self.error = None;
                    self.state = States::Initial;
                    Msg::FetchIngredients;
                    true
                }
                _ => false,
            },
            States::Success => false,
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let ingredients = match &self.ingredients {
            Some(ingredients) => ingredients
                .iter()
                .map(|ingredient: &Ingredient| {
                    html! {
                       <h1>{&ingredient.name}</h1>
                    }
                })
                .collect(),
            None => html! {
               <h1>{"hallo"}</h1>
            },
        };

        html! {
            <div>
                // <h2>{match &self.error {
                //     Some(e) => e,
                //     None => &String::from("None")
                // }}</h2>
                <h1>{"Ingredients page"} {match self.state {
                    States::Initial => "Initial",
                    States::Fetching => "Fetching",
                    States::Success => "Success",
                    States::Error => "Error"
                }}</h1>
                <div class="md:flex">
                    <div class="md:flex-shrink-0">
                        <img class="rounded-lg md:w-56" src="https://images.unsplash.com/photo-1556740738-b6a63e27c4df?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=448&q=80" width="448" height="299" alt="Woman paying for a purchase" />
                    </div>
                    <div class="mt-4 md:mt-0 md:ml-6">
                        <div class="uppercase tracking-wide text-sm text-indigo-600 font-bold">{"Marketing"}</div>
                        <a href="/iss" class="block mt-1 text-lg leading-tight font-semibold text-gray-900 hover:underline">{"Finding customers for your new business"}</a>
                        <p class="mt-2 text-gray-600">{"Getting a new business off the ground is a lot of hard work. Here are five ideas you can use to find your first customers."}</p>
                    </div>
                </div>
                <div class="md:flex">
                    <button onclick=self.link.callback(|_| Msg::FetchIngredients)>{"Hello"}</button>
                </div>

                {ingredients}
            </div>
        }
    }
}
