use crate::{
    get_endpoint,
    response::{Item, ItemFile},
};
use serde_json::Value;

pub async fn get_items() {
    let item_file: ItemFile = get_endpoint("/data/en_US/item.json").await.unwrap();
    let items_map: serde_json::Map<String, Value> = item_file.data;
    let mut items: Vec<Item> = vec![];
    items_map
        .iter()
        .for_each(|x| items.push(serde_json::from_value(x.1.clone()).unwrap()));
    println!("{:?}", items);
}
