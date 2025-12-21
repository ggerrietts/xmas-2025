use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ResultComponentProps {
    pub success: bool,
    pub next_destination: String,
    pub on_try_again: Callback<()>,
}

pub struct ResultComponent;

impl Component for ResultComponent {
    type Message = ();
    type Properties = ResultComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let success = ctx.props().success;
        let next_destination = ctx.props().next_destination.clone();
        let on_try_again = ctx.props().on_try_again.clone();
        
        if success {
            html! {
                <div class="result-component success">
                    <h2>{ "Congratulations!" }</h2>
                    <p class="success-message">{ "All answers are correct!" }</p>
                    <p class="next-location">
                        { format!("The next QR code can be found at: {}", next_destination) }
                    </p>
                </div>
            }
        } else {
            html! {
                <div class="result-component failure">
                    <h2>{ "Sorry, try again!" }</h2>
                    <p class="failure-message">
                        { "Some answers were incorrect. Please try again from the beginning." }
                    </p>
                    <button
                        class="try-again-button"
                        onclick={move |_| on_try_again.emit(())}
                    >
                        { "Try Again" }
                    </button>
                </div>
            }
        }
    }
}
