use serde::Deserialize;
use yew::format::Json;
use yew::format::Nothing;
use yew::services::{
    fetch::{FetchTask, Request, Response},
    FetchService,
};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Ingredient {
    props: Props,
    is_open: bool,
    state: States,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

pub enum States {
    Initial,
    Deleting,
    Deleted,
}

#[derive(Deserialize)]
struct DeleteResponse {
    response: String,
}

#[derive(PartialEq)]
pub enum Msg {
    Toggle,
    DeleteIngredient,
    DeleteSuccess,
    DeleteError,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32,
    #[prop_or_default]
    pub name: String,
    pub group: String,
    #[prop_or_default]
    pub description: Option<String>,
}

impl Component for Ingredient {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            is_open: false,
            fetch_task: None,
            state: States::Initial,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if msg == Msg::Toggle {
            self.is_open = !self.is_open;
            return true;
        }
        match self.state {
            States::Initial => match msg {
                Msg::DeleteIngredient => {
                    log::info!("deleting ingredients: {}", self.props.id);
                    let request = Request::delete(format!(
                        "http://localhost:8082/ingredients/{}",
                        self.props.id
                    ))
                    .body(Nothing)
                    .expect("Could not build request.");
                    // 2. construct a callback
                    let callback = self.link.callback(
                        |response: Response<Json<Result<DeleteResponse, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            match data {
                                Ok(_) => Msg::DeleteSuccess,
                                Err(_) => Msg::DeleteError,
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
                    self.state = States::Deleting;
                }
                Msg::DeleteSuccess => {
                    self.state = States::Deleted;
                }
                Msg::DeleteError => {
                    self.state = States::Initial;
                }
                _ => {}
            },
            States::Deleting => match msg {
                Msg::DeleteSuccess => {
                    self.state = States::Deleted;
                    self.link.send_message(Msg::DeleteSuccess);
                }
                Msg::DeleteError => {
                    self.state = States::Initial;
                    self.link.send_message(Msg::DeleteSuccess);
                }
                _ => {}
            },
            States::Deleted => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let food_emoji = match &self.props.group[..] {
            "Fruits" => "🍌",
            "Vegetables" => "🥦",
            "Herbs and Spices" => "🧂",
            "Nuts" => "🥜",
            "Cereals and cereal products" => "🌾",
            "Gourds" => "🥒",
            "Soy" => "🌾",
            "Pulses" => "🌾",
            "Aquatic foods" => "🐟",
            "Animal foods" => "🥩",
            "Beverages" => "🍹",
            "Confectioneries" => "🍬",
            "Baking goods" => "🍰",
            "Milk and milk products" => "🧀",
            "Fats and oils" => "🛢",
            "Cocoa and cocoa products" => "🍫",
            "Dishes" => "🍴",
            "Snack foods" => "🍱",
            "Teas" => "☕",
            "Eggs" => "🥚",
            "Baby foods" => "👶",
            "Coffee and coffee products" => "☕",
            _ => "❌",
        };

        return match &self.state {
            States::Initial | States::Deleting => html! {
                <div class="container flex flex-col m-auto p-2" >
                  <div class="flex justify-between">
                      <div class="flex">
                          <div class="mr-2 w-6 text-center">{food_emoji}</div>
                          <h3 class="p4">{&self.props.name}</h3>
                      </div>
                      <button onclick=self.link.callback(|_| Msg::Toggle)>
                          {match &self.is_open {
                              true => "👆",
                              false => "👇"
                          }}
                      </button>
                  </div>
                  {match &self.is_open {
                      true => html! {
                          <div class="mt-2 space-y-1 flex">
                              <div>
                                  <p>{"type: "}{&self.props.group}</p>
                                  <p>{"description: "}{match &self.props.description {
                                      Some(s) => s,
                                      None => ""
                                  }}</p>
                              </div>
                              {match &self.state {
                                  States::Initial => html! {
                                      <button class="flex" onclick=&self.link.callback(|_| Msg::DeleteIngredient)>{"❌"}</button>
                                  },
                                  States::Deleting | States::Deleted => html! {
                                      <div>{'🍤'}</div>
                                  }
                              }}
                          </div>
                      },
                      false => html! {
                          <div>{""}</div>
                      }
                  }}
                </div>
            },
            States::Deleted => html! {
                <div>{'🍤'}</div>
            },
        };
    }
}
