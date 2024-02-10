use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Stats {
    pub movement_speed: f32,
    pub level: f32,
    pub hp: f32,
    pub hp_regen: f32, // over 5 seconds
    pub mana: f32,
    pub mana_regen: f32, // over 5 seconds
    pub heal_power: f32,
    pub shield_power: f32,
    pub ad: f32,
    pub armor_pen: f32,
    pub lethality: f32,
    pub armor: f32,
    pub ap: f32,
    pub magic_pen: f32,
    pub percent_magic_pen: f32,
    pub mr: f32,
    pub crit: f32,
    pub crit_dmg: f32,
    pub attack_speed: f32,
    pub haste: f32,
    pub tenacity: f32,
    pub slow_resist: f32,
    pub life_steal: f32,
    pub physical_vamp: f32,
    pub omnivamp: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub gold: Gold,
    pub stats: ItemStats,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Gold {
    pub base: usize,
    pub purchasable: bool,
    pub total: usize,
    pub sell: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ItemStats {
    #[serde(rename = "FlatMovementSpeedMod")]
    #[serde(default)]
    flat_movement_speed: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Champion {
    pub name: String,
    pub stats: ChampionStats,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChampionStats {
    pub hp: f32,
    pub hpperlevel: f32,
    pub mp: f32,
    pub mpperlevel: f32,
    pub movespeed: f32,
    pub armor: f32,
    pub armorperlevel: f32,
    pub spellblock: f32,
    pub spellblockperlevel: f32,
    pub attackrange: f32,
    pub hpregen: f32,
    pub hpregenperlevel: f32,
    pub mpregen: f32,
    pub mpregenperlevel: f32,
    pub crit: f32,
    pub critperlevel: f32,
    pub attackdamage: f32,
    pub attackdamageperlevel: f32,
    pub attackspeedperlevel: f32,
    pub attackspeed: f32,
}
