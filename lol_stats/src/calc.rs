use crate::response::{Champion, Item, Stats};

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

pub fn calc_stats(level: f32, champion: &Champion, items: Vec<&Item>) -> Stats {
    let hp_bonus = 0.;
    let hp_regen_bonus = 0.;
    let mp_bonus = 0.;
    let mp_regen_bonus = 0.;
    let ad_bonus = 0.;
    let armor_bonus = 0.;
    let mr_bonus = 0.;
    let attack_speed_bonus = 0.;
    let stats = Stats {
        movement_speed: champion.stats.movespeed,
        level,
        hp: stats_formula(champion.stats.hp, champion.stats.hpperlevel, level, hp_bonus),
        hp_regen: stats_formula(champion.stats.hpregen, champion.stats.hpregenperlevel, level, hp_regen_bonus),
        mana: stats_formula(champion.stats.mp, champion.stats.mpperlevel, level, mp_bonus),
        mana_regen: stats_formula(champion.stats.mpregen, champion.stats.mpregenperlevel, level, mp_regen_bonus),
        heal_power: 0.,
        shield_power: 0.,
        ad: stats_formula(champion.stats.attackdamage, champion.stats.attackdamageperlevel, level, ad_bonus),
        armor_pen: 0.,
        lethality: 0.,
        armor: stats_formula(champion.stats.armor, champion.stats.armorperlevel, level, armor_bonus),
        ap: 0.,
        magic_pen: 0.,
        percent_magic_pen: 0.,
        mr: stats_formula(champion.stats.spellblock, champion.stats.spellblockperlevel, level, mr_bonus),
        crit: 0.,
        crit_dmg: 0.,
        // 0.648
        // 3
        attack_speed: champion.stats.attackspeed + (champion.stats.attackspeed * stats_formula(0., champion.stats.attackspeedperlevel, level, attack_speed_bonus)),
        haste: 0.,
        tenacity: 0.,
        slow_resist: 0.,
        life_steal: 0.,
        physical_vamp: 0.,
        omnivamp: 0.,
    };
    stats
}

fn stats_formula(base: f32, growth: f32, level: f32, bonus: f32) -> f32 {
    base + bonus + growth * (level - 1.) * (0.7025 + 0.0175 * (level - 1.))
}