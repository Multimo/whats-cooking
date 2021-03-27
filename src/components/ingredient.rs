use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Ingredient {
    props: Props,
    is_open: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub name: String,
    pub group: String,
    #[prop_or_default]
    pub description: Option<String>,
    #[prop_or_default]
    pub current_filter: String,
}

impl Component for Ingredient {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            is_open: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => self.is_open = !self.is_open,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.current_filter != props.current_filter {
            self.props.current_filter = props.current_filter;
            true
        } else {
            false
        }
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

        let handle_toggle = self.link.callback(|_| Msg::Toggle);
        let show = if self.props.current_filter == "" {
            false
        } else {
            let contains_name = self
                .props
                .name
                .to_lowercase()
                .contains(&self.props.current_filter.to_lowercase());
            let contains_group = self
                .props
                .group
                .to_lowercase()
                .contains(&self.props.current_filter.to_lowercase());

            if (contains_name) {
                log::info!(
                    "{} is contained in cn {}",
                    &self.props.current_filter,
                    self.props.name
                )
            }
            if (contains_group) {
                log::info!(
                    "{} is contained in group: {}",
                    &self.props.current_filter,
                    self.props.group
                )
            }

            contains_name || contains_group
        };

        let root_class = format!(
            "container flex flex-col m-auto p-2 {}",
            match show {
                true => "",
                false => "hidden",
            }
        );

        html! {
          <div class=root_class >
            <div class="flex justify-between">
                <div class="flex">
                    <div class="mr-2">{food_emoji}</div>
                    <h3 class="p4">{&self.props.name}</h3>
                </div>
                <button onclick=handle_toggle>
                    {match &self.is_open {
                        true => "👆",
                        false => "👇"
                    }}
                </button>
            </div>
            {match &self.is_open {
                true => html! {
                    <div class="mt-2 space-y-1">
                        <p>{"type: "}{&self.props.group}</p>
                        <p>{"description: "}{match &self.props.description {
                            Some(s) => s,
                            None => ""
                        }}</p>
                    </div>
                },
                false => html! {
                    <div>{""}</div>
                }
            }}
          </div>
        }
    }
}
