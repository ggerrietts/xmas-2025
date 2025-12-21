use yew::prelude::*;
use crate::models::Question;

#[derive(Properties, PartialEq, Clone)]
pub struct QuestionComponentProps {
    pub question: Question,
    pub selected_answer: Option<String>,
    pub on_answer_selected: Callback<String>,
    pub on_next: Callback<()>,
    pub can_proceed: bool,
}

pub struct QuestionComponent;

impl Component for QuestionComponent {
    type Message = ();
    type Properties = QuestionComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let question = ctx.props().question.clone();
        let selected = ctx.props().selected_answer.clone();
        let on_answer_selected = ctx.props().on_answer_selected.clone();
        let on_next = ctx.props().on_next.clone();
        let can_proceed = ctx.props().can_proceed;
        
        html! {
            <div class="question-component">
                <h2>{ format!("Question for {}", question.child.name) }</h2>
                <p class="question-text">{ question.question.clone() }</p>
                <div class="answers">
                    { for question.answers.iter().map(move |answer| {
                        let is_selected = selected.as_ref().map_or(false, |s| s == answer);
                        let answer_clone = answer.clone();
                        let callback = on_answer_selected.clone();
                        
                        html! {
                            <button
                                class={if is_selected { "answer-button selected" } else { "answer-button" }}
                                onclick={move |_| callback.emit(answer_clone.clone())}
                            >
                                { answer.clone() }
                            </button>
                        }
                    })}
                </div>
                if can_proceed {
                    <button
                        class="next-button"
                        onclick={move |_| on_next.emit(())}
                    >
                        { "Next" }
                    </button>
                }
            </div>
        }
    }
}
