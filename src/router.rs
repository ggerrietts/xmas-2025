use crate::components::PageComponent;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Start,
    #[at("/*page")]
    Page { page: String },
}

pub fn route_switch(route: Route) -> Html {
    match route {
        Route::Start => html! {
            <div>
                <h1>{ "Welcome to the Xmas Quiz!" }</h1>
                <p>{ "Get ready to test your knowledge and have some festive fun!" }</p>
                <a href="/happyholly">{ "Start the Quiz" }</a>
            </div>
        },
        Route::Page { page } => html! {
            <PageComponent page_url={page} />
        },
    }
}
