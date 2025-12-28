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
                <h2>{ "Welcome to the 2025 Little Orderings Christmas Treasure Hunt!" }</h2>
                <p>{ "Get ready to test your knowledge and have some festive fun!" }</p>
                <p>{ "At stops on the treasure trail, each of you will, in turn, be presented with a question and four possible answers. You must tap the correct answer, then hand the phone to the next player. Every question was selected for exactly that player. Nobody should try to answer anyone else's question. If you get stuck, you can ask for a hint."}</p>
                <p>{ "When everyone has answered their questions, the answers will be checked. If you got them all correct, the quiz will tell you where to find the next clue! If any of the answers were wrong, though, you'll have to try again. The quiz won't tell you which answers were wrong, so work together to figure that out!"}</p>
                <a href="start">{ "Start the Quiz" }</a>
            </div>
        </div>
    }
}
