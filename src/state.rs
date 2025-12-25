use crate::models::{get_page, get_question};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuizState {
    pub answers: Vec<Answer>,
    pub page_url: String,
    pub question_index: usize,
    pub quiz_status: QuizPhase,
}

type QuestionIndex = usize;
type AnswerIndex = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answer {
    pub question_index: QuestionIndex,
    pub answer_index: AnswerIndex,
    pub is_correct: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuizPhase {
    PresentingQuestions,
    ReviewingAnswers,
    MovingOn,
    Retrying,
    Finishing,
}

pub enum QuizAction {
    Review,
    SelectAnswer(QuestionIndex, AnswerIndex),
}

impl QuizState {
    pub fn new(url: &str) -> Self {
        QuizState {
            answers: Vec::new(),
            page_url: url.to_string(),
            question_index: 0,
            quiz_status: QuizPhase::PresentingQuestions,
        }
    }

    fn has_errors(&self) -> bool {
        self.answers.iter().any(|a| !a.is_correct)
    }
}

impl Reducible for QuizState {
    type Action = QuizAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            QuizAction::SelectAnswer(question_index, answer_index) => {
                let mut answers = self.answers.clone();
                let page = get_page(&self.page_url).unwrap();
                let question = get_question(&self.page_url, question_index).unwrap();
                answers.push(Answer {
                    question_index: question_index,
                    answer_index: answer_index,
                    is_correct: question.is_correct_answer(answer_index),
                });
                let index = question_index + 1;
                let status = if index < page.questions.len() {
                    QuizPhase::PresentingQuestions
                } else {
                    QuizPhase::ReviewingAnswers
                };
                QuizState {
                    answers,
                    page_url: self.page_url.clone(),
                    question_index: index,
                    quiz_status: status,
                }
                .into()
            }
            QuizAction::Review => {
                // check answers, route to correct landing
                let page = get_page(&self.page_url).unwrap();
                let next_phase = if self.has_errors() {
                    QuizPhase::Retrying
                } else if page.is_final {
                    QuizPhase::Finishing
                } else {
                    QuizPhase::MovingOn
                };
                QuizState {
                    answers: self.answers.clone(),
                    page_url: self.page_url.clone(),
                    question_index: self.question_index,
                    quiz_status: next_phase,
                }
                .into()
            } // Handle other actions as needed
        }
    }
}
