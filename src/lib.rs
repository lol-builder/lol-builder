use dominator::{html, Dom};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

use lol_stats::{model::get_items, response::Item};

struct App {
    items: Vec<Item>,
}

impl App {
    fn new(items: Vec<Item>) -> Arc<Self> {
        Arc::new(Self { items })
    }

    fn render(app: Arc<Self>) -> Dom {
        html!("div", {
            .text(format!("{:#?}", app.items.first().unwrap()).as_str())
        })
    }
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let app = App::new(get_items().await);

    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
