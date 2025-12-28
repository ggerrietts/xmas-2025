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

pub struct PageCard {
    pub next_url: String,
    pub location: String,
}

impl PageCard {
    pub fn vec_from_pages(pages: &Vec<Page>) -> Vec<PageCard> {
        let mut page_cards = Vec::new();
        let mut first = true;
        let mut location = String::new();
        for page in pages {
            if first {
                first = false;
            } else {
                let card = PageCard {
                    next_url: page.url.clone(),
                    location: location.clone(),
                };
                page_cards.push(card);
            }
            location = page.location.clone();
        }
        page_cards
    }
}
