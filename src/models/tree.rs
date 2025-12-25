use super::{Page, Question};
use std::sync::OnceLock;

pub fn get_page(url: &str) -> Option<Page> {
    for page in pages().iter() {
        if page.url == url {
            return Some(page.clone());
        }
    }
    None
}

pub fn get_question(url: &str, index: usize) -> Option<Question> {
    for page in pages().iter() {
        if page.url == url {
            return page.questions.get(index).cloned();
        }
    }
    None
}

static PAGES: OnceLock<Vec<Page>> = OnceLock::new();

fn pages() -> Vec<Page> {
    PAGES
        .get_or_init(|| {
            vec![
                Page::new(
                    "/happyholly",
                    "Under the tree",
                    vec![
                        Question::new(
                            "Gwen",
                            "Which members of Twice performed on the K-Pop Demon Hunters soundtrack?",
                            2,
                            vec!["Nayeon", "Jihyo", "Mina", "Dahyun"],
                        ),
                        Question::new(
                            "Evan",
                            "Which of the following is NOT an NF song?",
                            0,
                            vec!["Desire", "Lie", "Let You Down", "Clouds"],
                        ),
                    ],
                    false,
                ),
                Page::new(
                    "/elftastic",
                    "Inside the dryer",
                    vec![
                        Question::new(
                            "Gwen",
                            "In 'My Lie in April', what instrument does Kousei Arima play?",
                            2,
                            vec!["Violin", "Cello", "Piano", "Flute"],
                        ),
                        Question::new(
                            "Evan",
                            "Who directed the movie 'Inglorious Basterds'?",
                            1,
                            vec![
                                "Steven Spielberg",
                                "Quentin Tarantino",
                                "Christopher Nolan",
                                "Martin Scorsese",
                            ],
                        ),
                    ],
                    false,
                ),
                Page::new(
                    "/candycanary",
                    "In the dishwasher",
                    vec![
                        Question::new(
                            "Gwen",
                            "Which band is the battle-of-the-bands archrival of the Rainbooms?",
                            1,
                            vec![
                                "Trixie and the Illusions",
                                "The Dazzlings",
                                "Grand Funk Railroad",
                                "Huntr/x",
                            ],
                        ),
                        Question::new(
                            "Evan",
                            "Which gang fought Pennywise in 'It'?",
                            0,
                            vec![
                                "The Losers' Club",
                                "The Little Rascals",
                                "The Warriors",
                                "The Hell Fire Club",
                            ],
                        ),
                    ],
                    false,
                ),
            ]
        })
        .to_vec()
}
