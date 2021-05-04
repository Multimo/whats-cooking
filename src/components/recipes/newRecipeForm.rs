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
    description: Option<String>,
    link: Option<String>,
    url: Option<String>,
}

pub struct NewRecipesForm {
    link: ComponentLink<Self>,
    form_data: FormData,
    state: States,
    fetch_task: Option<FetchTask>,
}

#[derive(Copy, Clone)]
pub enum FormFieldName {
    Name,
    Description,
    Url,
    Link,
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
    StartSubmit,
    SubmitSuccess,
    SubmitError(anyhow::Error),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for NewRecipesForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            state: States::Initial,
            form_data: FormData {
                name: String::from(""),
                description: Some(String::from("")),
                link: None,
                url: None,
            },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match self.state {
            States::Initial | States::Invalid | States::Error(_) => match msg {
                Msg::UpdateFormField(field_name, input_data) => match field_name {
                    FormFieldName::Name => self.form_data.name = input_data,
                    FormFieldName::Description => self.form_data.description = Some(input_data),
                    FormFieldName::Url => self.form_data.url = Some(input_data),
                    FormFieldName::Link => self.form_data.link = Some(input_data),
                },
                Msg::Submit => {
                    log::info!("data: {:?}", self.form_data);

                    // returning same event so that it triggers next event
                    self.link.send_message(Msg::StartSubmit);
                    // validate data
                    // if valid do this
                    // if invalid do
                    // self.state = States::Invalid
                }
                _ => {}
            },
            States::Submitting => match msg {
                Msg::StartSubmit => {
                    // do the fetch
                    log::info!("sending post request");
                    let request = Request::post("http://localhost:8082/recipes")
                        .header("Content-Type", "application/json")
                        .body(Json(&self.form_data))
                        .expect("Could not build request.");
                    // 2. construct a callback
                    let callback = self.link.callback(
                        |response: Response<Json<Result<PostResponse, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            // log::info!("got data {:?}", data);
                            match data {
                                Ok(_) => Msg::SubmitSuccess,
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
                Msg::SubmitSuccess => self.state = States::Success,
                Msg::SubmitError(error) => self.state = States::Error(error),
                _ => {}
            },
            States::Success => {
                self.form_data = FormData {
                    name: String::from(""),
                    description: Some(String::from("")),
                    url: None,
                    link: None,
                }
            }
            States::Error(_) => {
                self.form_data = FormData {
                    name: String::from(""),
                    description: Some(String::from("")),
                    url: None,
                    link: None,
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
                <h3 class="mb-4 pl-2 uppercase tracking-wide text-sm text-indigo-600 font-bold">{"New Recipe"}</h3>
                <div class="flex mb-4 items-center space-x-2 w-full mt-2">
                    <Input
                        class="w-1/2"
                        name="name"
                        id="name"
                        label="Recipe name"
                        input_type="text"
                        on_change=handle_change(FormFieldName::Name)
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
                        name="url"
                        id="url"
                        label="Photo"
                        input_type="text"
                        on_change=handle_change(FormFieldName::Url)
                    />
                    <Input
                        class="w-1/2"
                        name="link"
                        id="link"
                        label="Link to recipe on external website (if exists)"
                        input_type="text"
                        on_change=handle_change(FormFieldName::Link)
                    />
                </div>
                <button onclick=&self.link.callback(|_| Msg::Submit) class=button_styles>{"Submit"}</button>
            </div>
        }
    }
}
