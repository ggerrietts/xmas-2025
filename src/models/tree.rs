use super::{Page, PageCard, Question};
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

pub fn get_page_cards() -> Vec<PageCard> {
    let pages = pages();
    PageCard::vec_from_pages(&pages)
}

static PAGES: OnceLock<Vec<Page>> = OnceLock::new();

fn pages() -> Vec<Page> {
    PAGES
        .get_or_init(|| {
            vec![
                Page::new(
                    "start",
                    "behind the tree",
                    vec![
                        Question::new(
                            "Bridget",
                            "In 'Law & Order: SVU', who is NOT one of the main detectives?",
                            2,
                            vec!["Olivia Benson", "Elliot Stabler", "Alexandra Cabot", "John Munch"],
                        ),
                        Question::new(
                            "Evan",
                            "Which of the following is NOT an NF song?",
                            0,
                            vec!["Desire", "Lie", "Let You Down", "Clouds"],
                        ),
                        Question::new(
                            "Gwen",
                            "Which members of Twice performed on the K-Pop Demon Hunters soundtrack?",
                            2,
                            vec![
                                "Jeongyeon, Sana, and Chaeyoung",
                                "Chaeyoung, Dahyun, and Chaeyoung",
                                "Jeongyeon, Jihyo, and Chaeyoung",
                                "Jeongyeon, Nayeon, and Chaeyoung",
                            ],
                        ),
                    ],
                    false,
                ),
                Page::new(
                    "elftastic",
                    "inside the dryer",
                    vec![
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
                        Question::new(
                            "Gwen",
                            "In 'My Lie in April', what instrument does Kousei Arima play?",
                            2,
                            vec!["Violin", "Cello", "Piano", "Flute"],
                        ),
                        Question::new(
                            "Bridget",
                            "In 'Law & Order: SVU', which longtime colleague does Olivia Benson ultimately replace when she is promoted to Captain of the unit?",
                            2,
                            vec![
                                "John Munch",
                                "Elliot Stabler",
                                "Donald Cragen",
                                "Amanda Rollins",
                            ],
                        ),
                    ],
                    false,
                ),
                Page::new(
                    "candybowl",
                    "by the coffee",
                    vec![
                        Question::new(
                            "Gwen",
                            "Which band is the battle-of-the-bands archrival of the Rainbooms?",
                            1,
                            vec![
                                "Trixie and the Illusions",
                                "The Dazzlings",
                                "Grand Funk Railroad",
                                "Huntrx",
                            ],
                        ),
                        Question::new(
                            "Bridget",
                            "Which Childish Gambino album features the song 'Redbone'?",
                            2,
                            vec![
                                "Because the Internet",
                                "Camp",
                                "Awaken, My Love!",
                                "3.15.20",
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
                Page::new(
                    "gingerbread",
                    "in bathroom cabinet",
                    vec![
                        Question::new(
                            "Bridget",
                            "Which character in 'Criminal Minds' is known for their love of magic tricks and sleight-of-hand?",
                            1,
                            vec![
                                "Spencer Reid",
                                "David Rossi",
                                "Aaron Hotchner",
                                "Derek Morgan",
                            ],
                        ),
                        Question::new(
                            "Evan",
                            "In 'Stranger Things', what lab ran the program that connects Eleven to the Upside Down?",
                            1,
                            vec![
                                "MIT",
                                "The Hawkins Lab",
                                "The Argonne Psychic Initiative",
                                "The Nevada Energy Research Facility",
                            ],
                        ),
                        Question::new(
                            "Gwen",
                            "At South Park Elementary, what subject does Mr. Garrison primarily teach during the early seasons of the show?",
                            2,
                            vec![
                                "Science",
                                "History",
                                "Third-grade classroom instruction",
                                "Music",
                            ],
                        ),
                    ],
                    false,
                ),
                Page::new(
                    "nutcracker",
                    "in dad's sock drawer",
                    vec![
                        Question::new(
                            "Evan",
                            "In the medical drama 'The Pitt', what unique narrative structure does the first season use for its episodes?",
                            1,
                            vec![
                                "Each episode focuses on a different hospital department",
                                "Each episode takes place during one hour of a single extended ER shift",
                                "Each episode retells events from multiple perspectives",
                                "Each episode is presented as recovered ER security footage",
                            ],
                        ),
                        Question::new(
                            "Gwen",
                            "In 'My Little Pony: Friendship Is Magic', what key event leads to Twilight Sparkle becoming the Princess of Friendship?",
                            1,
                            vec![
                                "Discovering the Crystal Empire",
                                "Solving Starswirl's unfinished spell and understanding true friendship",
                                "Passing Celestia's final magical exam",
                                "Restoring harmony after Discord's return",
                            ],
                        ),
                        Question::new(
                            "Bridget",
                            "What is the name of the group of working-class teens at the center of 'Outer Banks'?",
                            3,
                            vec![
                                "The Shoals",
                                "The Breakers",
                                "The Riptides",
                                "The Pogues",
                            ],
                        ),
                    ],
                    false,
                ),

                Page::new(
                    "snowflake",
                    "in the veggie drawer",
                    vec![
                        Question::new(
                            "Bridget",
                            "Which Harry Styles album includes the song 'As It Was'?",
                            2,
                            vec![
                                "Fine Line",
                                "Harry Styles",
                                "Harry's House",
                                "Lights Up",
                            ],
                        ),
                        Question::new(
                            "Evan",
                            "In 'South Park', Cartman desperately wants to be invited to Kyle's birthday party at Casa Bonita. What attraction at the restaurant is his favorite?",
                            1,
                            vec![
                                "The Fountain Plaza",
                                "Black Bart's Cave",
                                "The Cliff Divers' Pool",
                                "The Pirate's Grotto",
                            ],
                        ),
                    ],
                    false,
                ),

                Page::new(
                    "holly",
                    "behind the measuring cups",
                    vec![
                        Question::new(
                            "Gwen",
                            "In 'She-Ra and the Princesses of Power', what personal realization about the Horde drives Adora to take up the Sword of Protection?",
                            1,
                            vec![
                                "That the Horde plans to abandon Etheria",
                                "That the Horde has been lying to her about being a hero",
                                "That Catra is secretly working against her",
                                "That the First Ones created the Horde",
                            ],
                        ),
                        Question::new(
                            "Bridget",
                            "What is the title of Coldplay's debut full-length studio album?",
                            2,
                            vec![
                                "Rush of Blood to the Head",
                                "X&Y",
                                "Parachutes",
                                "Viva la Vida",
                            ],
                        ),
                        Question::new(
                            "Evan",
                            "What is the name of the town where 'It' takes place?",
                            2,
                            vec![
                                "Salem",
                                "Castle Rock",
                                "Derry",
                                "Bangor",
                            ],
                        ),
                    ],
                    false,
                ),

                Page::new(
                    "tinsel",
                    "under the couch cushion",
                    vec![
                        Question::new(
                            "Bridget",
                            "On SZA's album 'Ctrl', several songs include spoken-word commentary from her mother and grandmother. What theme do these interludes primarily reflect?",
                            1,
                            vec![
                                "Fame and money",
                                "Confidence and self-worth",
                                "Nostalgia for childhood",
                                "Spiritual faith",
                            ],
                        ),
                        Question::new(
                            "Evan",
                            "The events of 'Black Hawk Down' are based on a U.S. military operation in which country?",
                            1,
                            vec![
                                "Afghanistan",
                                "Somalia",
                                "Kuwait",
                                "Iraq",
                            ],
                        ),
                        Question::new(
                            "Gwen",
                            "Which Mitski album includes the song 'Nobody'?",
                            2,
                            vec![
                                "Puberty 2",
                                "Laurel Hell",
                                "Be the Cowboy",
                                "Bury Me at Makeout Creek",
                            ],
                        ),
                    ],
                    false,
                ),

                Page::new(
                    "bauble",
                    "under dad's keyboard",
                    vec![
                        Question::new(
                            "Bridget",
                            "Westfield State's main campus includes a central student gathering space that houses dining, student organizations, and activity areas. What is the name of this building?",
                            1,
                            vec![
                                "Parenzo Union",
                                "Ely Campus Center",
                                "Bates Hall",
                                "Horace Mann Commons",
                            ],
                        ),
                        Question::new(
                            "Evan",
                            "Which NF album features the hit song 'Let You Down'?",
                            2,
                            vec![
                                "Mansion",
                                "The Search",
                                "Perception",
                                "Hope",
                            ],
                        ),
                        Question::new(
                            "Gwen",
                            "At many Twenty One Pilots concerts, the show closes with the crowd chanting together during 'Trees' while confetti falls and drums are played among the audience. Among fans, what does this moment most commonly represent?",
                            0,
                            vec![
                                "A celebration of the connection between band and fans",
                                "A coded reference to early independent-era songs",
                                "A signal that an encore is about to begin",
                                "A symbolic retelling of the Blurryface storyline",
                            ],
                        ),
                    ],
                    true,
                ),
            ]
        })
        .to_vec()
}
