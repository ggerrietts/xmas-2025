use crate::models::{get_page, get_question, Page, Question};
use std::{borrow::Cow, fmt::Display, rc::Rc, str::FromStr};
use yew::prelude::*;
use yew_router::query::{FromQuery, ToQuery};

#[derive(Clone, Debug, Properties, PartialEq, Eq)]
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
    Advance(String),
    Retry,
    Review,
    SelectAnswer(QuestionIndex, AnswerIndex),
}

impl QuizState {
    fn has_errors(&self) -> bool {
        self.answers.iter().any(|a| !a.is_correct)
    }

    pub fn get_page(self: &Rc<Self>) -> Option<Page> {
        get_page(&self.page_url)
    }

    pub fn get_question(self: &Rc<Self>, question_index: usize) -> Option<Question> {
        get_question(&self.page_url, question_index)
    }

    pub fn get_current_question(self: &Rc<Self>) -> Option<Question> {
        get_question(&self.page_url, self.question_index)
    }

    pub fn dispatch(self: Rc<Self>, action: QuizAction) -> Rc<Self> {
        match action {
            QuizAction::Advance(new_url) => QuizState {
                answers: Vec::new(),
                page_url: new_url.clone(),
                question_index: 0,
                quiz_status: QuizPhase::PresentingQuestions,
            }
            .into(),
            QuizAction::Retry => QuizState {
                answers: Vec::new(),
                page_url: self.page_url.clone(),
                question_index: 0,
                quiz_status: QuizPhase::PresentingQuestions,
            }
            .into(),
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

impl ToQuery for QuizState {
    type Error = anyhow::Error;

    fn to_query(&self) -> anyhow::Result<Cow<'_, str>> {
        let answers = self
            .answers
            .iter()
            .map(|a| a.answer_index.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let s = format!(
            "p={}&q={}&s={}&a={}",
            self.page_url,
            self.question_index,
            self.quiz_status.clone() as u8,
            answers
        );
        Ok(Cow::Owned(s))
    }
}

impl FromQuery for QuizState {
    type Target = QuizState;
    type Error = anyhow::Error;

    fn from_query(query: &str) -> anyhow::Result<QuizState> {
        let params: Vec<&str> = query.split('&').collect();
        let mut page_url = String::new();
        let mut question_index = 0;
        let mut quiz_status = QuizPhase::PresentingQuestions;
        let mut answers: Vec<Answer> = Vec::new();

        for param in params {
            let mut key_value = param.split('=');
            let key = key_value
                .next()
                .ok_or(anyhow::anyhow!("query parameter missing key"))?;
            let value = key_value
                .next()
                .ok_or(anyhow::anyhow!("query parameter missing value"))?;

            match key {
                "p" => {
                    page_url = if let Some(_) = get_page(value) {
                        value.to_string()
                    } else {
                        anyhow::bail!("Invalid page URL: {}", value);
                    }
                }
                "q" => question_index = value.parse()?,
                "s" => {
                    let status_num: u8 = value.parse()?;
                    quiz_status = match status_num {
                        0 => QuizPhase::PresentingQuestions,
                        1 => QuizPhase::ReviewingAnswers,
                        2 => QuizPhase::MovingOn,
                        3 => QuizPhase::Retrying,
                        4 => QuizPhase::Finishing,
                        _ => {
                            anyhow::bail!("Invalid quiz status value: {}", status_num);
                        }
                    }
                }
                "a" => {
                    let answer_indices: Vec<&str> = value.split(',').collect();
                    for (i, ans_str) in answer_indices.iter().enumerate() {
                        let ans_index: usize = ans_str.parse()?;
                        answers.push(Answer {
                            question_index: i,
                            answer_index: ans_index,
                            is_correct: false, // correctness needs to be determined later
                        });
                    }
                }
                _ => {
                    anyhow::bail!("Unknown query parameter: {}", key);
                }
            }
        }

        Ok(QuizState {
            answers,
            page_url,
            question_index,
            quiz_status,
        })
    }
}

impl FromStr for QuizState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        QuizState::from_query(s)
    }
}

impl Display for QuizState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
