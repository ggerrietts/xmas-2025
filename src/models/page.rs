use super::question::Question;

#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub url: String,
    pub location: String,
    pub questions: Vec<Question>,
    pub is_final: bool,
}

impl Page {
    pub fn new(url: &str, location: &str, questions: Vec<Question>, is_final: bool) -> Self {
        Page {
            url: url.to_string(),
            location: location.to_string(),
            questions,
            is_final,
        }
    }
}
