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
    flat_ms: f32,
    #[serde(rename = "FlatHPPoolMod")]
    #[serde(default)]
    flat_hp: f32,
    #[serde(rename = "FlatMPPoolMod")]
    #[serde(default)]
    flat_mana: f32,
    #[serde(rename = "PercentHPPoolMod")]
    #[serde(default)]
    percent_hp: f32,
    #[serde(rename = "PercentMPPoolMod")]
    #[serde(default)]
    percent_mana: f32,
    #[serde(rename = "FlatHPRegenMod")]
    #[serde(default)]
    flat_hp_regen: f32,
    #[serde(rename = "PercentHPRegenMod")]
    #[serde(default)]
    percent_hp_regen: f32,
    #[serde(rename = "FlatMPRegenMod")]
    #[serde(default)]
    flat_mana_regen: f32,
    #[serde(rename = "PercentMPRegenMod")]
    #[serde(default)]
    percent_mana_regen: f32,
    #[serde(rename = "FlatArmorMod")]
    #[serde(default)]
    flat_armor: f32,
    #[serde(rename = "PercentArmorMod")]
    #[serde(default)]
    percent_armor: f32,
    #[serde(rename = "FlatPhysicalDamageMod")]
    #[serde(default)]
    flat_ad: f32,
    #[serde(rename = "FlatMagicDamageMod")]
    #[serde(default)]
    flat_ap: f32,
    #[serde(rename = "PercentPhysicalDamageMod")]
    #[serde(default)]
    percent_ad: f32,
    #[serde(rename = "PercentMagicDamageMod")]
    #[serde(default)]
    percent_ap: f32,
    #[serde(rename = "PercentMovementSpeedMod")]
    #[serde(default)]
    percent_ms: f32,
    #[serde(rename = "FlatAttackSpeedMod")]
    #[serde(default)]
    flat_as: f32,
    #[serde(rename = "PercentAttackSpeedMod")]
    #[serde(default)]
    percent_as: f32,
    #[serde(rename = "FlatCritChanceMod")]
    #[serde(default)]
    flat_crit: f32,
    #[serde(rename = "FlatEnergyRegenMod")]
    #[serde(default)]
    flat_energy_regen: f32,
    #[serde(rename = "FlatEnergyPoolMod")]
    #[serde(default)]
    flat_energy: f32,
    #[serde(rename = "PercentLifeStealMod")]
    #[serde(default)]
    percent_lifesteal: f32,
    #[serde(rename = "PercentSpellVampMod")]
    #[serde(default)]
    percent_spellvamp: f32,
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
