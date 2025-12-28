use yew::prelude::*;

pub fn four_oh_four() -> Html {
    html! {
        <div class="page-xmas">
            <div class="page-content">
                <h2>{ "Lost in the Snow" }</h2>
                <p>{ "The page you are looking for does not exist. Try backing up and trying again." }</p>
            </div>
        </div>
    }
}

pub fn congrats_page(location: &str) -> Html {
    html! {
        <div class="page-xmas">
            <div class="page-content">
                <h2>{ "Congratulations!" }</h2>
                <p>{ "You've completed the Xmas Quiz. Well done!" }</p>
                <p>{ format!("Your final stop is {}.", location) }</p>
            </div>
        </div>
    }
}

pub fn start_page() -> Html {
    html! {
        <div class="page-xmas">
            <div class="page-content">
                <h2>{ "Welcome to the Xmas Quiz!" }</h2>
                <p>{ "Get ready to test your knowledge and have some festive fun!" }</p>
                <a href="/start">{ "Start the Quiz" }</a>
            </div>
        </div>
    }
}
