use crate::components::styles::button_styles;
use crate::components::Input;
use crate::yew::format::Json;
use crate::yew::InputData;
use serde::{Deserialize, Serialize};
use yew::{
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Children, Component, ComponentLink, Html, Properties, ShouldRender,
};

#[derive(Deserialize)]
struct PostResponse {
    response: String,
}

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct FormData {
    name: String,
    food_group: String,
    decription: Option<String>,
    name_scientific: Option<String>,
    food_subgroup: Option<String>,
}

pub struct NewIngredientsForm {
    link: ComponentLink<Self>,
    form_data: FormData,
    state: States,
    fetch_task: Option<FetchTask>,
}

#[derive(Copy, Clone)]
pub enum FormFieldName {
    Name,
    Description,
    FoodGroup,
    NameScientific,
    FoodSubgroup,
}

pub enum States {
    Initial,
    Invalid,
    Submitting,
    Success,
    Error(anyhow::Error),
}

pub enum Msg {
    UpdateFormField(FormFieldName, String),
    Submit,
    SubmitSuccess,
    SubmitError(anyhow::Error),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for NewIngredientsForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            state: States::Initial,
            form_data: FormData {
                name: String::from(""),
                food_group: String::from(""),
                food_subgroup: None,
                decription: None,
                name_scientific: None,
            },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match self.state {
            States::Initial | States::Invalid | States::Error(_) => match msg {
                Msg::UpdateFormField(field_name, input_data) => match field_name {
                    FormFieldName::Name => self.form_data.name = input_data,
                    FormFieldName::FoodGroup => self.form_data.food_group = input_data,
                    FormFieldName::FoodSubgroup => self.form_data.food_subgroup = Some(input_data),
                    FormFieldName::Description => self.form_data.decription = Some(input_data),
                    FormFieldName::NameScientific => {
                        self.form_data.name_scientific = Some(input_data)
                    }
                },
                Msg::Submit => {
                    log::info!("data: {:?}", self.form_data);

                    if self.form_data.name != "" && self.form_data.food_group != "" {
                        self.state = States::Submitting
                    } else {
                        self.state = States::Invalid
                    }

                    // returning same event so that it triggers next event
                    self.link.send_message(Msg::Submit);
                    // validate data
                    // if valid do this
                    // if invalid do
                    // self.state = States::Invalid
                }
            },
            States::Submitting => match msg {
                Msg::Submit => {
                    // do the fetch
                    log::info!("sending post request");
                    let request = Request::post("http://localhost:8082/ingredients")
                        .header("Content-Type", "application/json")
                        .body(Json(self.form_data))
                        .expect("Could not build request.");
                    // 2. construct a callback
                    let callback = self.link.callback(
                        |response: Response<Json<Result<PostResponse, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            // log::info!("got data {:?}", data);
                            match data {
                                Ok(data) => Msg::SubmitSuccess,
                                Err(error) => Msg::SubmitError(error),
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
                }
                _ => {}
            },
            States::Success => {
                self.form_data = FormData {
                    name: String::from(""),
                    food_group: String::from(""),
                    food_subgroup: None,
                    decription: None,
                    name_scientific: None,
                }
            }
            States::Error(_) => {
                self.form_data = FormData {
                    name: String::from(""),
                    food_group: String::from(""),
                    food_subgroup: None,
                    decription: None,
                    name_scientific: None,
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let handle_change = |field: FormFieldName| {
            return self
                .link
                .callback(move |event: InputData| Msg::UpdateFormField(field, event.value));
        };

        html! {
            <div class="my-6 w-full">
                <h1 class="ml-2">{match &self.state {
                    States::Initial => html! {<div>{"Initial"}</div> },
                    States::Submitting => html! {<div>{"Submitting"}</div> },
                    States::Invalid => html! {<div>{"Invalid"}</div> },
                    States::Success => html! {<div>{"Success"}</div> },
                    States::Error(errorMessage) => html! { <h2>{errorMessage}</h2> }
                }}</h1>
                <h3 class="mb-4 pl-2 uppercase tracking-wide text-sm text-indigo-600 font-bold">{"New Ingredient"}</h3>
                <div class="flex mb-4 items-center space-x-2 w-full mt-2">
                    <Input
                        class="w-1/2"
                        name="name"
                        id="name"
                        label="Ingredient name"
                        input_type="text"
                        on_change=handle_change(FormFieldName::Name)
                    />
                    <Input
                        class="w-1/2"
                        name="group"
                        id="group"
                        label="Ingredient food group"
                        input_type="text"
                        on_change=handle_change(FormFieldName::FoodGroup)
                    />
                </div>

                <Input
                    name="description"
                    id="description"
                    label="Description"
                    input_type="textArea"
                    on_change=handle_change(FormFieldName::Description)
                />
                <div class="flex mt-4 items-center space-x-2 w-full">
                    <Input
                        class="w-1/2"
                        name="name_scientific"
                        id="name_scientific"
                        label="Scientific name"
                        input_type="text"
                        on_change=handle_change(FormFieldName::NameScientific)
                    />
                    <Input
                        class="w-1/2"
                        name="food_subgroup"
                        id="food_subgroup"
                        label="Food Subgroup eg: herbs in herbs and spices"
                        input_type="text"
                        on_change=handle_change(FormFieldName::FoodSubgroup)
                    />
                </div>
                <button onclick=&self.link.callback(|_| Msg::Submit) class=button_styles>{"Submit"}</button>
            </div>
        }
    }
}
