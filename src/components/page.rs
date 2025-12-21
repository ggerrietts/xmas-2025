use yew::prelude::*;
use crate::models::Page;
use super::question::QuestionComponent;
use super::review::ReviewComponent;
use super::result::ResultComponent;

pub enum PageView {
    Question { index: usize },
    Review,
    Result { success: bool },
}

pub enum Msg {
    AnswerSelected { question_index: usize, answer: String },
    NextQuestion,
    SubmitAnswers,
    TryAgain,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PageComponentProps {
    pub page: Page,
}

pub struct PageComponent {
    view: PageView,
    answers: Vec<Option<String>>,
    page: Page,
}

impl Component for PageComponent {
    type Message = Msg;
    type Properties = PageComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let page = ctx.props().page.clone();
        let num_questions = page.questions.len();
        
        Self {
            view: PageView::Question { index: 0 },
            answers: vec![None; num_questions],
            page,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AnswerSelected { question_index, answer } => {
                if question_index < self.answers.len() {
                    self.answers[question_index] = Some(answer);
                }
                true
            }
            Msg::NextQuestion => {
                match &self.view {
                    PageView::Question { index } => {
                        let next_index = *index + 1;
                        if next_index < self.page.questions.len() {
                            self.view = PageView::Question { index: next_index };
                        } else {
                            self.view = PageView::Review;
                        }
                    }
                    _ => {}
                }
                true
            }
            Msg::SubmitAnswers => {
                let all_correct = self.validate_answers();
                self.view = PageView::Result { success: all_correct };
                true
            }
            Msg::TryAgain => {
                // Reset all answers and go back to first question
                self.answers = vec![None; self.page.questions.len()];
                self.view = PageView::Question { index: 0 };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.view {
            PageView::Question { index } => {
                let question_index = *index;
                let question = &self.page.questions[question_index];
                let selected_answer = self.answers[question_index].clone();
                let can_proceed = selected_answer.is_some();
                
                html! {
                    <div class="question-view">
                        <QuestionComponent
                            question={question.clone()}
                            selected_answer={selected_answer}
                            on_answer_selected={ctx.link().callback(move |answer: String| {
                                Msg::AnswerSelected { question_index, answer }
                            })}
                            on_next={ctx.link().callback(|_| Msg::NextQuestion)}
                            can_proceed={can_proceed}
                        />
                    </div>
                }
            }
            PageView::Review => {
                html! {
                    <ReviewComponent
                        questions={self.page.questions.clone()}
                        answers={self.answers.clone()}
                        on_submit={ctx.link().callback(|_| Msg::SubmitAnswers)}
                    />
                }
            }
            PageView::Result { success } => {
                html! {
                    <ResultComponent
                        success={*success}
                        next_destination={self.page.next_destination.clone()}
                        on_try_again={ctx.link().callback(|_| Msg::TryAgain)}
                    />
                }
            }
        }
    }
}

impl PageComponent {
    fn validate_answers(&self) -> bool {
        self.page.questions.iter().enumerate().all(|(i, q)| {
            if let Some(ref answer) = self.answers[i] {
                answer == &q.correct_answer
            } else {
                false
            }
        })
    }
}
