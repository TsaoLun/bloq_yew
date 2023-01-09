use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::{home::*, blog::*};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Blog => html! {<Blog />},
    }
}