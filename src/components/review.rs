use crate::{models::get_question, state::Answer};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ReviewComponentProps {
    pub answers: Vec<Answer>,
    pub on_submit: Callback<()>,
    pub on_retry: Callback<()>,
    pub page_url: String,
}

#[component]
pub fn ReviewComponent(props: &ReviewComponentProps) -> Html {
    let ReviewComponentProps {
        answers,
        on_submit,
        on_retry,
        page_url,
    } = props;
    let on_submit = on_submit.clone();
    let on_retry = on_retry.clone();

    let on_retry_click = Callback::from(move |_| {
        on_retry.emit(());
    });

    let on_submit_click = Callback::from(move |_| {
        on_submit.emit(());
    });

    html! {
        <div class="review-component page-xmas">
            <div class="page-content">
                <h2>{ "Review Your Answers" }</h2>
                <ol>
                {
                    answers.iter().enumerate().map(|(_, answer)| {
                        if let Some(question) = get_question(page_url, answer.question_index) {
                            html! {
                                <li><strong>{ &question.question }</strong><br/>
                                    <em>{ format!("{}'s answer: ", &question.child) }</em>{ &question.answers[answer.answer_index] }
                                </li>
                            }
                        } else {
                            html! {}
                        }
                    }).collect::<Html>()
                }
                </ol>
                <div class="grid">
                    <div><button class="btn-xmas" onclick={on_submit_click}>{ "Submit" }</button></div>
                    <div><button class="btn-xmas-secondary" onclick={on_retry_click}>{ "Retry" }</button></div>
                </div>
            </div>
        </div>
    }
}
