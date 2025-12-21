use yew::prelude::*;
use crate::models::Question;

#[derive(Properties, PartialEq, Clone)]
pub struct ReviewComponentProps {
    pub questions: Vec<Question>,
    pub answers: Vec<Option<String>>,
    pub on_submit: Callback<()>,
}

pub struct ReviewComponent;

impl Component for ReviewComponent {
    type Message = ();
    type Properties = ReviewComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let questions = ctx.props().questions.clone();
        let answers = ctx.props().answers.clone();
        let on_submit = ctx.props().on_submit.clone();
        
        html! {
            <div class="review-component">
                <h2>{ "Review Your Answers" }</h2>
                <div class="review-list">
                    { for questions.iter().enumerate().map(move |(i, q)| {
                        let answer = answers.get(i)
                            .and_then(|a| a.as_ref())
                            .map(|a| a.as_str())
                            .unwrap_or("Not answered");
                        
                        html! {
                            <div class="review-item">
                                <h3>{ q.child.name.clone() }</h3>
                                <p class="review-question">{ q.question.clone() }</p>
                                <p class="review-answer">{ format!("Your answer: {}", answer) }</p>
                            </div>
                        }
                    })}
                </div>
                <button
                    class="submit-button"
                    onclick={move |_| on_submit.emit(())}
                >
                    { "Submit" }
                </button>
            </div>
        }
    }
}
