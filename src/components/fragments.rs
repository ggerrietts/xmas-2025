use yew::prelude::*;

pub fn four_oh_four() -> Html {
    html! {
        <div>
            <h2>{ "Lost in the Snow" }</h2>
            <p>{ "The page you are looking for does not exist. Try backing up and trying again." }</p>
        </div>
    }
}

pub fn congrats_page(location: &str) -> Html {
    html! {
        <div>
            <h2>{ "Congratulations!" }</h2>
            <p>{ "You've completed the Xmas Quiz. Well done!" }</p>
            <p>{ format!("Your final stop is at {}.", location) }</p>
        </div>
    }
}
