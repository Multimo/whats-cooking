mod home;
mod ingredients;
mod iss;
mod recipes;

use yew_router::prelude::RouterAnchor;
use yew_router::switch::Permissive;
use yew_router::Switch;
// use yew_router::matcher::MatcherToken;

pub use home::Home;
pub use ingredients::IngredientsPage;
pub use iss::FetchServiceExample;
pub use recipes::RecipesPage;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    #[to = "/iss"]
    FetchServiceExample,
    #[to = "/recipes"]
    RecipesPage,
    #[to = "/ingredients"]
    IngredientsPage,
    #[to = "/404"]
    NotFound(Permissive<String>),
    #[to = "/"]
    Home,
    // #[to = MatcherToken::Exact("/")]
    // Home,
    // #[to = "/{*:any}"]
    // NotFound(Permissive<String>),
}

pub type AppAnchor = RouterAnchor<AppRoutes>;
