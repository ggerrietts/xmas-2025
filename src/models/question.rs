#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub child: String,
    pub question: String,
    pub correct_answer: usize,
    pub answers: Vec<String>,
}

impl Question {
    /// Create a new `Question` using string literals for fields and a Vec<&str> for answers.
    pub fn new(child: &str, question: &str, correct_answer: usize, answers: Vec<&str>) -> Self {
        Self {
            child: child.to_string(),
            question: question.to_string(),
            correct_answer,
            answers: answers.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn is_correct_answer(&self, answer_index: usize) -> bool {
        self.correct_answer == answer_index
    }
}
