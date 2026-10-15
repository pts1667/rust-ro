//! How the server is set up (rates, limits, rathena battle options), for a program that plans around them. Read-only, needs no character.
use configuration::battle_config::BattleConfig;
use configuration::configuration::{Config, MapCombatDamageRates};
use serde_json::{Value, json};

use crate::server::game_loop::GAME_TICK_RATE;
use crate::server::service::global_config_service::GlobalConfigService;

const DROP_KINDS: [&str; 5] = ["heal", "use", "equip", "card", "common"];

/// A rate of `config.json` times the percentage of the matching battle option: what the server applies.
fn effective_rate(rate: f32, battle_percent: i64) -> f64 {
    f64::from(rate) * battle_percent as f64 / 100.0
}

fn rate(rate: f32, battle: &BattleConfig, option: &str) -> Value {
    let percent = battle.get(option);
    json!({ "config": rate, "battle_percent": percent, "effective": effective_rate(rate, percent) })
}

fn damage_rates(rates: &MapCombatDamageRates) -> Value {
    json!({
        "weapon_skill": rates.weapon_skill,
        "magic_skill": rates.magic_skill,
        "misc_skill": rates.misc_skill,
        "normal_short": rates.normal_short,
        "normal_long": rates.normal_long,
    })
}

fn drops(config: &Config) -> Value {
    let battle = &config.battle;
    let by_kind: serde_json::Map<String, Value> = DROP_KINDS
        .iter()
        .map(|kind| {
            let percent = |suffix: &str| battle.get(&format!("item_rate_{kind}{suffix}"));
            ((*kind).to_string(), json!({ "normal": percent(""), "boss": percent("_boss"), "mvp": percent("_mvp") }))
        })
        .collect();
    json!({
        "rate": config.game.drop_rate,
        "card_rate": config.game.drop_rate_card,
        "item_rate_percent": by_kind,
        "treasure_percent": battle.get("item_rate_treasure"),
        "zero_chance_allowed": battle.flag("drop_rate0item"),
    })
}

pub fn server_settings() -> Value {
    let config = GlobalConfigService::instance().config();
    let (game, battle) = (&config.game, &config.battle);
    json!({
        "packetver": config.server.packetver,
        "tick_ms": GAME_TICK_RATE,
        "experience": {
            "base": rate(game.base_exp_rate, battle, "base_exp_rate"),
            "job": rate(game.job_exp_rate, battle, "job_exp_rate"),
            "pvp_exp": game.pvp_exp,
            "party_even_share_bonus": game.party_even_share_bonus,
            "guild_exp_rate_percent": config.char_server.guild_exp_rate,
            "death_penalty": {
                "kind": game.death_penalty.kind,
                "base": game.death_penalty.base,
                "job": game.death_penalty.job,
                "zeny": game.death_penalty.zeny,
                "max_level_loses_base": game.death_penalty.max_level_loses_base,
                "max_level_loses_job": game.death_penalty.max_level_loses_job,
            },
        },
        "drops": drops(config),
        "limits": {
            "max_base_level": game.max_base_level,
            "max_stat_level": game.max_stat_level,
            "max_inventory": game.max_inventory,
            "default_char_speed": game.default_char_speed,
            "item_use_interval_ms": game.item_use_interval,
            "cashfood_use_interval_ms": game.cashfood_use_interval,
        },
        "vending": {
            "max_value": game.vending_max_value,
            "over_max": game.vending_over_max,
            "tax": game.vending_tax,
            "tax_min": game.vending_tax_min,
            "buyer_name": game.buyer_name,
        },
        "guild": { "max_alliances": game.guild_max_alliances, "max_castles": game.guild_max_castles },
        "pet_support": {
            "attack_exp_to_master": game.pet_support.attack_exp_to_master,
            "attack_exp_rate": game.pet_support.attack_exp_rate,
            "minimum_intimacy": game.pet_support.minimum_intimacy,
        },
        "damage_rates_percent": {
            "gvg": damage_rates(&game.gvg_damage_rates),
            "battleground": damage_rates(&game.battleground_damage_rates),
        },
        "mobs": { "density": game.mob_density },
        "bots": { "max_bots": config.bots.max_bots, "max_registered_bots": config.bots.max_registered_bots, "idle_logout_secs": config.bots.idle_logout_secs, "command_group": config.bots.command_group },
    })
}

/// The battle options whose name contains `filter` (all of them without one), with their value, default and range.
pub fn battle_options(filter: Option<&str>) -> Value {
    let filter = filter.map(str::to_ascii_lowercase);
    let options: Vec<Value> = GlobalConfigService::instance()
        .config()
        .battle
        .options()
        .filter(|(name, ..)| filter.as_deref().is_none_or(|filter| name.contains(filter)))
        .map(|(name, value, default, min, max)| json!({ "name": name, "value": value, "default": default, "min": min, "max": max }))
        .collect();
    json!({ "options": options })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rate_is_the_config_value_scaled_by_the_battle_percentage() {
        assert_eq!(effective_rate(2.0, 150), 3.0);
        assert_eq!(effective_rate(1.0, 100), 1.0);
        let battle = BattleConfig::default();
        let shown = rate(5.0, &battle, "base_exp_rate");
        assert_eq!((shown["battle_percent"].as_i64(), shown["effective"].as_f64()), (Some(100), Some(5.0)));
    }

    #[test]
    fn the_settings_and_the_battle_options_are_listed() {
        crate::tests::common::before_all();
        let settings = server_settings();
        assert!(settings["experience"]["base"]["effective"].is_number());
        assert!(settings["drops"]["item_rate_percent"]["card"]["mvp"].is_number());
        assert!(settings["limits"]["max_base_level"].as_u64().unwrap() >= 1);

        let all = battle_options(None)["options"].as_array().unwrap().len();
        let exp = battle_options(Some("EXP_RATE"));
        let names: Vec<_> = exp["options"].as_array().unwrap().iter().map(|option| option["name"].as_str().unwrap()).collect();
        assert!(names.contains(&"base_exp_rate") && names.contains(&"job_exp_rate"), "{names:?}");
        assert!(names.len() < all);
        assert!(battle_options(Some("no such option"))["options"].as_array().unwrap().is_empty());
    }
}
