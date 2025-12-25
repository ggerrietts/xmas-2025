use crate::{models::get_question, state::Answer};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ReviewComponentProps {
    pub answers: Vec<Answer>,
    pub on_submit: Callback<()>,
    pub page_url: String,
}

#[component]
pub fn ReviewComponent(props: &ReviewComponentProps) -> Html {
    let ReviewComponentProps {
        answers,
        on_submit,
        page_url,
    } = props;
    let on_submit = on_submit.clone();

    let on_click = Callback::from(move |_| {
        on_submit.emit(());
    });

    html! {
        <div>
            {
                answers.iter().enumerate().map(|(_, answer)| {
                    if let Some(question) = get_question(page_url, answer.question_index) {
                        html! {
                            <div key={answer.question_index}>
                                <p><strong>{ &question.child }</strong></p>
                                <p>{ &question.question }</p>
                                <p>{ "Answer: " }{ &question.answers[answer.answer_index] }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }).collect::<Html>()
            }
            <button onclick={on_click}>
                { "Submit" }
            </button>
        </div>
    }
}
