use crate::components::styles::button_styles;
use crate::components::Input;
use crate::yew::InputData;
use yew::{html, Children, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
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
}

#[derive(Copy, Clone)]
pub enum FormFieldName {
    Name,
    Description,
    FoodGroup,
    NameScientific,
    FoodSubgroup,
}

pub enum Msg {
    UpdateFormField(FormFieldName, String),
    Submit,
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
        match msg {
            Msg::UpdateFormField(field_name, input_data) => match field_name {
                FormFieldName::Name => self.form_data.name = input_data,
                FormFieldName::FoodGroup => self.form_data.food_group = input_data,
                FormFieldName::FoodSubgroup => self.form_data.food_subgroup = Some(input_data),
                FormFieldName::Description => self.form_data.decription = Some(input_data),
                FormFieldName::NameScientific => self.form_data.name_scientific = Some(input_data),
            },
            Msg::Submit => log::info!("data: {:?}", self.form_data),
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
