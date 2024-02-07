use dioxus::prelude::*;
use lol_stats::model::get_items;
use utils::async_block;

mod utils;

fn main() {
    async_block(get_items());
    // dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render!("nothing yet")
}
