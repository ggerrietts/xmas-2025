use super::{
    fragments::{congrats_page, four_oh_four},
    question::QuestionComponent,
    retry::RetryComponent,
    review::ReviewComponent,
    success::SuccessComponent,
};
use crate::models::get_page;
use crate::router::Route;
use crate::state::{QuizAction, QuizPhase, QuizState};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PageComponentProps {
    pub page_url: String,
}

#[component]
pub fn PageComponent(props: &PageComponentProps) -> Html {
    let page_url = props.page_url.clone();
    let page = get_page(&page_url);
    if let None = page {
        return four_oh_four();
    }
    let page = page.unwrap();

    let page_state = use_reducer(|| QuizState::new(&page_url));
    let navigator = use_navigator().unwrap();

    match page_state.quiz_status {
        QuizPhase::PresentingQuestions => {
            html! {
                <QuestionComponent
                    page_url={page_url.clone()}
                    question_index={page_state.question_index}
                    on_answer_selected={
                        let page_state = page_state.clone();
                        Callback::from(move |selected_answer| {
                            page_state.dispatch(QuizAction::SelectAnswer(
                                page_state.question_index,
                                selected_answer,
                            ));
                        })
                    }
                />
            }
        }
        QuizPhase::ReviewingAnswers => {
            html! {
                <ReviewComponent
                    page_url={page_url.clone()}
                    answers={page_state.answers.clone()}
                    on_submit={
                        let page_state = page_state.clone();
                        Callback::from(move |_| {
                            page_state.dispatch(QuizAction::Review);
                        })
                    }
                />
            }
        }
        QuizPhase::MovingOn => {
            let page = page.clone();
            let navigator = navigator.clone();

            let onsuccess = Callback::from(move |next_url: String| {
                let path = format!("/{}", next_url);
                navigator.push(&Route::Page { page: path });
            });

            html! {
                <SuccessComponent
                    location = { page.location.clone() }
                    on_success = { onsuccess }
                />
            }
        }
        QuizPhase::Retrying => {
            let navigator = navigator.clone();
            let on_retry = Callback::from(move |_| {
                navigator.push(&Route::Page {
                    page: page_url.clone(),
                });
            });
            html! {
                <RetryComponent
                    on_retry={on_retry}
                />
            }
        }
        QuizPhase::Finishing => {
            let page = page.clone();
            // Assume final page always has next_url to the
            congrats_page(&page.location)
        }
    }
}
