use yew::prelude::*;
use crate::config;
use crate::components::page::PageComponent;

pub enum Msg {
    ConfigLoaded(Result<crate::models::Pages, String>),
}

pub struct Router {
    pages: Option<crate::models::Pages>,
    error: Option<String>,
}

impl Component for Router {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            Msg::ConfigLoaded(config::load_config().await)
        });

        Self {
            pages: None,
            error: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ConfigLoaded(Ok(pages)) => {
                self.pages = Some(pages);
                self.error = None;
                true
            }
            Msg::ConfigLoaded(Err(e)) => {
                self.error = Some(e);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(ref error) = self.error {
            return html! {
                <div>
                    <h1>{ "Error loading configuration" }</h1>
                    <p>{ error }</p>
                </div>
            };
        }

        let pages = match &self.pages {
            Some(p) => p,
            None => {
                return html! {
                    <div>
                        <h1>{ "Loading..." }</h1>
                    </div>
                };
            }
        };

        let current_url = get_hash_url();
        let page = config::get_page_by_url(pages, &current_url);

        match page {
            Some(p) => html! {
                <PageComponent page={p.clone()} />
            },
            None => html! {
                <div>
                    <h1>{ "Page not found" }</h1>
                    <p>{ format!("No page found for URL: {}", current_url) }</p>
                </div>
            },
        }
    }
}

fn get_hash_url() -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let hash = location.hash().unwrap_or_default();
    
    // Remove the # prefix if present
    if hash.starts_with('#') {
        hash[1..].to_string()
    } else if hash.is_empty() {
        // Default to first page or empty
        String::new()
    } else {
        hash
    }
}
