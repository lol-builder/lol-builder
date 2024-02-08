use dioxus::prelude::*;
use dioxus_web::Config;
use lol_stats::{model::get_items, response::Item};

#[derive(Props, PartialEq)]
struct Data {
    pub items: Vec<Item>,
}

fn main() {
    let data: Data = Data {
        items: vec![],
    };
    dioxus_web::launch_with_props(app, data, Config::new());
}

fn app<'a>(cx: &'a Scoped<'a, Data>) -> Element<'a> {
    let f = use_future(cx, (), |_| get_items());
    match f.value() {
        Some(items) => render!(
            format!("{:?}", items.first().unwrap())
        ),
        None => render!(
            div {
                format!("loading")
            }
        ),
    }
}
