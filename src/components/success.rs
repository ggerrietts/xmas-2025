use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SuccessProps {
    pub location: String,
    pub on_success: Callback<String>,
}

#[component]
pub fn SuccessComponent(props: &SuccessProps) -> Html {
    let location = props.location.clone();
    let destination = use_state(String::new);
    let on_success = props.on_success.clone();

    let on_input = {
        let destination = destination.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            destination.set(target.value());
        })
    };

    let on_submit = {
        let destination = destination.clone();
        Callback::from(move |_| {
            on_success.emit((*destination).clone());
        })
    };

    html! {
        <div class="page-xmas">
            <div class="page-content">
                <h2>{ "Success!" }</h2>
                <p>{ "You got all questions correct!" }</p>
                <p>{ format!("Find your next clue {}.", location) }</p>
                <p>{ "Type the code word on that clue here:" }</p>
                <form>
                    <fieldset role="group">
                        <input
                            type="text"
                            placeholder="code word"
                            oninput={on_input}
                            value={(*destination).clone()}
                        />
                        <button onclick={on_submit}>
                            { "Go!" }
                        </button>
                    </fieldset>
                </form>
            </div>
        </div>
    }
}
