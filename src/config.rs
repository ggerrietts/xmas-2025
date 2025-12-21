use crate::models::Pages;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub async fn load_config() -> Result<Pages, String> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let url = "/data.json";
    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|_| "Failed to create request".to_string())?;

    let window = web_sys::window().ok_or("No window object")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch data.json".to_string())?;

    let resp: Response = wasm_bindgen::JsCast::dyn_into(resp_value)
        .map_err(|_| "Response is not a Response object".to_string())?;

    let json = JsFuture::from(
        resp.json()
            .map_err(|_| "Failed to get JSON from response".to_string())?,
    )
    .await
    .map_err(|_| "Failed to parse JSON".to_string())?;

    let pages: Pages = serde_wasm_bindgen::from_value(json)
        .map_err(|e| format!("Failed to deserialize JSON: {:?}", e))?;

    Ok(pages)
}

pub fn get_page_by_url<'a>(pages: &'a Pages, url: &str) -> Option<&'a crate::models::Page> {
    pages.pages.iter().find(|p| p.url == url)
}
