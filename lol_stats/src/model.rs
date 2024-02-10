use crate::{
    get_endpoint,
    response::{Champion, Item},
};
use serde_json::Value;

pub async fn get_items() -> Vec<Item> {
    let item_file: serde_json::Map<String, Value> =
        get_endpoint("/data/en_US/item.json").await.unwrap();
    let items_map: serde_json::Map<String, Value> =
        item_file.get("data").unwrap().as_object().unwrap().clone();
    let mut items: Vec<Item> = vec![];
    items_map
        .iter()
        .for_each(|x| items.push(serde_json::from_value(x.1.clone()).unwrap()));
    items
}

pub async fn get_champions() -> Vec<Champion> {
    let champions_file: serde_json::Map<String, Value> =
        get_endpoint("/data/en_US/champion.json").await.unwrap();
    let champions_map: serde_json::Map<String, Value> = champions_file
        .get("data")
        .unwrap()
        .as_object()
        .unwrap()
        .clone();
    let mut champions_id: Vec<String> = vec![];
    champions_map.iter().for_each(|x| {
        champions_id.push(serde_json::from_value(x.1.clone().get("id").unwrap().clone()).unwrap())
    });
    let mut champions: Vec<Champion> = vec![];
    for id in champions_id {
        let champion_file: serde_json::Map<String, Value> =
            get_endpoint(&format!("/data/en_US/champion/{}.json", id))
                .await
                .unwrap();
        let champion: Champion =
            serde_json::from_value(champion_file.get("data").unwrap().get(id).unwrap().clone())
                .unwrap();
        champions.push(champion);
    }
    champions
}

// pub async fn get_runes() -> Vec<MainRune> {
//     let rune_file: serde_json::Map<String, Value> =
//         get_endpoint("/data/en_US/runesReforged.json").await.unwrap();
//     let runes_map: serde_json::Map<String, Value> =
//     rune_file.get("data").unwrap().as_object().unwrap().clone();
//     let mut runes: Vec<MainRune> = vec![];
//     runes_map
//         .iter()
//         .for_each(|x| runes.push(serde_json::from_value(x.1.clone()).unwrap()));
//     runes
// }
