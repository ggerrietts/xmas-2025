use crate::models::get_question;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct QuestionComponentProps {
    pub page_url: String,
    pub question_index: usize,
    pub on_answer_selected: Callback<usize>,
}

#[component]
pub fn QuestionComponent(props: &QuestionComponentProps) -> Html {
    let QuestionComponentProps {
        page_url,
        question_index,
        on_answer_selected,
    } = props;

    let question = get_question(page_url, *question_index).unwrap();

    html! {
        <div class="question-component page-xmas">
            <div class="page-content">
                <h2>{ format!("{}'s Question", question.child) }</h2>
                <p class="question-text">{ question.question.clone() }</p>
                <ol>
                    { for question.answers.iter().enumerate().map(move |(index, answer)| {
                        let callback = on_answer_selected.clone();
                        html! {
                            <li><a href="#" onclick={move |e: MouseEvent| {
                                e.prevent_default();
                                callback.emit(index.clone());
                            }}>
                                { answer.clone() }
                            </a></li>
                        }
                    })}
                </ol>
            </div>
        </div>
    }
}
