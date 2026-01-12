use crate::components::{fragments::start_page, PageComponent, PrintComponent};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Start,
    #[at("/print")]
    Print,
    #[at("/quiz")]
    Page,
}

pub fn route_switch(route: Route) -> Html {
    match route {
        Route::Start => start_page(),
        Route::Page => html! {
            <PageComponent />
        },
        Route::Print => html! {
            <PrintComponent />
        },
    }
}
