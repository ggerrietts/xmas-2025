mod models;
mod config;
mod router;
mod components;

use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();
    let element = document.get_element_by_id("main").unwrap();
    yew::Renderer::<App>::with_root(element).render();
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <router::Router />
        </div>
    }
}
