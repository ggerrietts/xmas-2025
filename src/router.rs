use crate::components::{fragments::start_page, PageComponent, PrintComponent};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Start,
    #[at("/print")]
    Print,
    #[at("/*page")]
    Page { page: String },
}

pub fn route_switch(route: Route) -> Html {
    match route {
        Route::Start => start_page(),
        Route::Page { page } => html! {
            <PageComponent page_url={page} />
        },
        Route::Print => html! {
            <PrintComponent />
        },
    }
}
