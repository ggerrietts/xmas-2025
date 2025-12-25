use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct RetryProps {
    pub on_retry: Callback<()>,
}

#[component]
pub fn RetryComponent(props: &RetryProps) -> Html {
    let on_retry = props.on_retry.clone();
    let on_click = Callback::from(move |_| {
        on_retry.emit(());
    });
    html! {
        <div>
            <h2>{ "Oh no!" }</h2>
            <p>{ "At least one of your answers was incorrect. You'll have to try again." }</p>
            <button onclick={on_click}>
                { "Retry" }
            </button>
        </div>
    }
}
