use crate::components::Input;
use crate::yew::InputData;
use serde::Deserialize;
use yew::{html, Children, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Deserialize, Debug)]
pub struct IIngredient {
    pub name: String,
    pub decription: Option<String>,
    pub food_group: Option<String>,
    pub name_scientific: Option<String>,
    pub food_subgroup: Option<String>,
}

pub struct NewIngredientsForm {
    link: ComponentLink<Self>,
}

pub enum Msg {
    UpdateFormField,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for NewIngredientsForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let handle_change = &self.link.callback(|_: InputData| Msg::UpdateFormField);

        html! {
            <div class="my-6 w-full">
                <h3 class="mb-4 pl-4">{"New Ingredient"}</h3>
                <div class="flex mb-4 items-center space-x-2 w-full mt-4">
                    <Input
                        name="name"
                        id="name"
                        label="Ingredient name"
                        input_type="text"
                        on_change=handle_change
                    />
                    <Input
                        name="group"
                        id="group"
                        label="Ingredient food group"
                        input_type="text"
                        on_change=handle_change
                    />
                </div>

                <Input
                    name="description"
                    id="description"
                    label="Description"
                    input_type="textArea"
                    on_change=handle_change
                />
                <div class="flex mt-4 items-center space-x-2 w-full">
                    <Input
                        name="name_scientific"
                        id="name_scientific"
                        label="Scientific name"
                        input_type="text"
                        on_change=handle_change
                    />
                    <Input
                        name="food_subgroup"
                        id="food_subgroup"
                        label="Food Subgroup eg: herbs in herbs and spices"
                        input_type="text"
                        on_change=handle_change
                    />
                </div>
                <button class="mt-4 pl-4">{"Submit"}</button>
            </div>
        }
    }
}
