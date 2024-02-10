use crate::response::{Champion, Item, Rune, Stats};

pub fn calc_resisted_dmg(damage: f32, resistance: f32) -> f32 {
    if resistance >= 0. {
        return damage * 100. / (100. + resistance);
    }
    damage * (2. - 100. / (100. - resistance))
}

pub fn calc_total_auto_dmg(stats: Stats) -> f32 {
    let magic_dmg = calc_resisted_dmg(0., 0.);
    let physical_dmg = calc_resisted_dmg(stats.ad, 0.);
    let true_dmg = 0.;
    magic_dmg + physical_dmg + true_dmg
}

pub fn calc_stats(level: f32, champion: &Champion, rune: Vec<&Rune>, items: Vec<&Item>) -> Stats {
    let stats = Stats {
        movement_speed: champion.stats.movespeed,
        level,
        hp: champion.stats.hp + champion.stats.hpperlevel * (level - 1.),
        hp_regen: champion.stats.hpregen + champion.stats.hpregenperlevel * (level - 1.),
        mana: champion.stats.mp + champion.stats.mpperlevel * (level - 1.),
        mana_regen: champion.stats.mpregen + champion.stats.mpregenperlevel * level,
        heal_power: 0.,
        shield_power: 0.,
        ad: champion.stats.attackdamage + champion.stats.attackdamageperlevel * (level - 1.),
        armor_pen: 0.,
        lethality: 0.,
        armor: champion.stats.armor + champion.stats.armorperlevel * (level - 1.),
        ap: 0.,
        magic_pen: 0.,
        percent_magic_pen: 0.,
        mr: champion.stats.spellblock + champion.stats.spellblockperlevel * (level - 1.),
        crit: 0.,
        crit_dmg: 0.,
        attack_speed: champion.stats.attackspeed + champion.stats.attackspeedperlevel * (level - 1.),
        haste: 0.,
        tenacity: 0.,
        slow_resist: 0.,
        life_steal: 0.,
        physical_vamp: 0.,
        omnivamp: 0.,
    };
    stats
}
