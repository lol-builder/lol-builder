use dominator::{html, Dom};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

use lol_stats::{
    calc::{calc_stats, calc_total_auto_dmg},
    model::{get_champions, get_items},
    response::{Champion, Item, Stats},
};

struct App {
    items: Vec<Item>,
    champions: Vec<Champion>,
    // runes: Vec<Rune>,
    // cur_items: Vec<Item>,
}

impl App {
    fn new(items: Vec<Item>, champions: Vec<Champion>) -> Arc<Self> {
        Arc::new(Self {
            items,
            champions,
            // runes,
            // cur_items: vec![],
        })
    }

    fn render(app: Arc<Self>) -> Dom {
        let cur_champion = app.champions.get(30).unwrap();
        let items = vec![app.items.first().unwrap()];
        // let runes = vec![];
        let stats = calc_stats(3., cur_champion, items.clone());
        html!("div", {
            .children(
                &mut [
                    html!("div", {
                        .text(format!("{:?}", items).as_str())
                    }),
                    html!("div", {
                        .text(format!("{}", cur_champion.name).as_str())
                    }),
                    html!("div", {
                        .text(format!("{:?}", stats).as_str())
                    }),
                    html!("div", {
                        .text(format!("{:?}", calc_total_auto_dmg(stats)).as_str())
                    }),
                ]
            )
        })
    }
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let items = get_items();
    let champions = get_champions();

    let app = App::new(items.await, champions.await);

    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
