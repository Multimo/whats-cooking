use crate::components::{button_styles, Ingredient, Input, NewIngredientsForm};
use serde::Deserialize;

use yew::{
    format::{Json, Nothing},
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink, Html, InputData, ShouldRender,
};

#[derive(Deserialize, Debug)]
enum States {
    Initial,
    Fetching,
    Success { ingredients: Vec<IIngredient> },
    Error(String),
}

#[derive(Deserialize, Debug)]
pub struct IIngredient {
    pub id: i32,
    pub name: String,
    pub name_scientific: Option<String>,
    pub decription: Option<String>,
    pub food_group: Option<String>,
    pub food_subgroup: Option<String>,
}

#[derive(Deserialize, Debug)]
struct IngredientResponse {
    response: Vec<IIngredient>,
}

pub struct IngredientsPage {
    state: States,
    current_filter: String,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    FetchIngredients,
    FetchIngredientsSuccess(Vec<IIngredient>),
    FetchIngredientsError(anyhow::Error),
    UpdateFilter(String),
}

impl IngredientsPage {
    fn debounce_filter_update(&self, new_filter: String) {
        self.link.send_message(Msg::UpdateFilter(new_filter))
    }
}

impl Component for IngredientsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: States::Initial,
            current_filter: String::from(""),
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
                    self.state = States::Success { ingredients: data };
                    true
                }
                Msg::FetchIngredientsError(error) => {
                    log::info!("got found error {:?}", error);
                    // let error = String::from("There was an error");
                    self.state = States::Error(error.to_string());
                    self.fetch_task = None;
                    true
                }
                _ => false,
            },
            States::Success { ingredients: _ } => match msg {
                Msg::UpdateFilter(value) => {
                    self.current_filter = value;
                    true
                }
                _ => false,
            },
            States::Error(_) => match msg {
                Msg::FetchIngredients => {
                    self.state = States::Initial;
                    self.link.send_message(Msg::FetchIngredients);
                    true
                }
                _ => false,
            },
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render == true {
            self.link.send_message(Msg::FetchIngredients)
        }
    }

    fn view(&self) -> Html {
        let handle_change = &self
            .link
            .callback(|event: InputData| Msg::UpdateFilter(event.value));

        html! {
            <div>
                <div class="md:flex">
                   <NewIngredientsForm />
                </div>

                <div class="flex flex-col my-3 h-auto mb-5">
                    <Input
                        name="search"
                        id="search"
                        label="Filter Ingredients"
                        input_type="text"
                        on_change=handle_change
                    />
                </div>

                {match &self.state {
                    States::Initial => html! { <h1> {"Initial"} </h1> },
                    States::Fetching => html! { <h1> {"Fetching"} </h1>},
                    States::Success {
                        ingredients
                    } => ingredients
                    .iter()
                    .filter(|ingredient: &&IIngredient| {
                        if &self.current_filter == "" || &self.current_filter.len() <= &3 {
                            return true;
                        }
                        let group = match &ingredient.food_group {
                            Some(v) => &v,
                            None => "."
                        };

                        let contains_name: bool = ingredient.name.to_lowercase().contains(&self.current_filter.to_lowercase());
                        let contains_group: bool =  group.to_lowercase().contains(&self.current_filter.to_lowercase());

                        if (contains_name) {
                            log::info!("{} is contained in cn {}", &self.current_filter.len(), ingredient.name)
                        }
                        if (contains_group) {
                            log::info!("{} is contained in group: {}", &self.current_filter, group)
                        }
                        return contains_name || contains_group;
                    })
                    .map(|ingredient: &IIngredient| {
                        html! {
                            <Ingredient
                                key={ingredient.id}
                                name=&ingredient.name
                                group=match &ingredient.food_group {
                                    Some(v) => &v,
                                    None => "."
                                }
                                description=&ingredient.decription
                            />
                        }
                    })
                    .collect(),
                    States::Error(error) => html! {
                        <>
                            <h1 class="text-red-600 ">{error}</h1>
                            <button
                                class=button_styles
                                onclick=self.link.callback(|_| Msg::FetchIngredients)
                            >
                                {"Click to retry"}
                            </button>
                        </>
                    }
                }}
            </div>
        }
    }
}
