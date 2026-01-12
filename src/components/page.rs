use super::{
    fragments::{congrats_page, four_oh_four},
    question::QuestionComponent,
    retry::RetryComponent,
    review::ReviewComponent,
    success::SuccessComponent,
};
use crate::router::Route;
use crate::state::{QuizAction, QuizPhase, QuizState};
// use crate::router::Route;

use std::rc::Rc;

use yew::prelude::*;
use yew_router::{
    prelude::*,
    query::{FromQuery, ToQuery},
};

/// Creates a callback that dispatches an action and navigates to the route with the new state
fn make_nav_callback<F>(
    navigator: Navigator,
    quiz_state: Rc<QuizState>,
    action_fn: F,
) -> Callback<()>
where
    F: Fn(Rc<QuizState>) -> QuizAction + 'static,
{
    Callback::from(move |_| {
        let quiz_state = quiz_state.clone();
        let quiz_action = action_fn(quiz_state.clone());
        let new_state = quiz_state.dispatch(quiz_action);
        let _ = navigator
            .push_with_query(&Route::Page, new_state.to_query().unwrap().into_owned())
            .unwrap();
    })
}

/// Creates a callback that dispatches an action with a parameter and navigates
fn make_nav_callback_with_param<T, F>(
    navigator: Navigator,
    quiz_state: Rc<QuizState>,
    action_fn: F,
) -> Callback<T>
where
    T: 'static,
    F: Fn(Rc<QuizState>, T) -> QuizAction + 'static,
{
    Callback::from(move |param| {
        let quiz_state = quiz_state.clone();
        let quiz_action = action_fn(quiz_state.clone(), param);
        let new_state = quiz_state.dispatch(quiz_action);
        let _ = navigator
            .push_with_query(&Route::Page, new_state.to_query().unwrap().into_owned())
            .unwrap();
    })
}

#[component]
pub fn PageComponent() -> Html {
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let quiz_state: Rc<QuizState> = {
        let query = location.query_str();
        Rc::new(QuizState::from_query(query).unwrap())
    };

    let page = if let Some(page) = quiz_state.get_page() {
        page
    } else {
        return four_oh_four();
    };

    match quiz_state.quiz_status {
        QuizPhase::PresentingQuestions => {
            html! {
                <QuestionComponent
                    quiz_state={quiz_state.clone()}
                    on_answer_selected={
                        make_nav_callback_with_param(navigator.clone(), quiz_state.clone(), |state, answer| {
                            QuizAction::SelectAnswer(state.question_index, answer)
                        })
                    }
                />
            }
        }
        QuizPhase::ReviewingAnswers => {
            html! {
                <ReviewComponent
                    quiz_state={quiz_state.clone()}
                    on_retry={
                        make_nav_callback(navigator.clone(), quiz_state.clone(), |_| QuizAction::Retry)
                    }
                    on_submit={
                        make_nav_callback(navigator.clone(), quiz_state.clone(), |_| QuizAction::Review)
                    }
                />
            }
        }
        QuizPhase::MovingOn => {
            html! {
                <SuccessComponent
                    location = { page.location.clone() }
                    on_success = {
                        make_nav_callback_with_param(navigator.clone(), quiz_state.clone(), |_, next_url| {
                            QuizAction::Advance(next_url)
                        })
                    }
                />
            }
        }
        QuizPhase::Retrying => {
            html! {
                <RetryComponent
                    on_retry={
                        make_nav_callback(navigator.clone(), quiz_state.clone(), |_| QuizAction::Retry)
                    }
                />
            }
        }
        QuizPhase::Finishing => {
            let page = page.clone();
            congrats_page(&page.location)
        }
    }
}
