mod components;
mod models;
mod router;
mod state;

use router::{route_switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

pub fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={route_switch} />
        </BrowserRouter>
    }
}
