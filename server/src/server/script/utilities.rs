use chrono::{DateTime, Datelike, Local, TimeZone, Timelike};
use models::enums::effect_id::EffectId;
use models::enums::{EnumWithNumberValue, EnumWithStringValue};
use script_sdk::{Reply, Value};

use crate::repository::model::item_model::ItemModel;
use crate::server::service::global_config_service::GlobalConfigService;

const DATE_FIELDS: [&str; 9] = [
    "DT_SECOND", "DT_MINUTE", "DT_HOUR", "DT_DAYOFWEEK", "DT_DAYOFMONTH", "DT_MONTH", "DT_YEAR", "DT_DAYOFYEAR", "DT_YYYYMMDD",
];
const MONTHS: [&str; 12] = [
    "JANUARY", "FEBRUARY", "MARCH", "APRIL", "MAY", "JUNE", "JULY", "AUGUST", "SEPTEMBER", "OCTOBER", "NOVEMBER", "DECEMBER",
];
const WEEKDAYS: [&str; 7] = ["SUNDAY", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"];
const ITEM_FIELDS: [&str; 21] = [
    "ITEMINFO_BUY", "ITEMINFO_SELL", "ITEMINFO_TYPE", "ITEMINFO_MAXCHANCE", "ITEMINFO_GENDER", "ITEMINFO_LOCATIONS", "ITEMINFO_WEIGHT",
    "ITEMINFO_ATTACK", "ITEMINFO_DEFENSE", "ITEMINFO_RANGE", "ITEMINFO_SLOT", "ITEMINFO_VIEW", "ITEMINFO_EQUIPLEVELMIN",
    "ITEMINFO_WEAPONLEVEL", "ITEMINFO_ALIASNAME", "ITEMINFO_EQUIPLEVELMAX", "ITEMINFO_MAGICATTACK", "ITEMINFO_ID", "ITEMINFO_AEGISNAME",
    "ITEMINFO_ARMORLEVEL", "ITEMINFO_SUBTYPE",
];

pub fn constant(name: &str) -> Option<Value> {
    if let Some(index) = DATE_FIELDS.iter().position(|value| *value == name) {
        return Some(((index + 1) as i32).into());
    }
    if let Some(index) = MONTHS.iter().position(|value| *value == name) {
        return Some(((index + 1) as i32).into());
    }
    if let Some(index) = WEEKDAYS.iter().position(|value| *value == name) {
        return Some((index as i32).into());
    }
    if let Some(index) = ITEM_FIELDS.iter().position(|value| *value == name) {
        return Some((index as i32).into());
    }
    if let Ok(effect) = EffectId::try_from_string(name) {
        return Some((effect.value() as i32).into());
    }
    match name {
        "VIP_STATUS_ACTIVE" | "PET_CATCH_UNIVERSAL" | "PET_CATCH_UNIVERSAL_NO_BOSS" => Some(1.into()),
        "VIP_STATUS_EXPIRE" | "PET_CATCH_ALL" => Some(2.into()),
        "VIP_STATUS_REMAINING" => Some(3.into()),
        "PET_CATCH_NORMAL" => Some(0.into()),
        "SEX_FEMALE" => Some(0.into()),
        "SEX_MALE" => Some(1.into()),
        "SEX_BOTH" => Some(2.into()),
        _ => None,
    }
}

pub fn get_time(field: &Value) -> Value {
    time_value(Local::now(), field).into()
}

fn time_value<Tz: TimeZone>(time: DateTime<Tz>, field: &Value) -> i32 {
    let field = field.number_value().ok().or_else(|| {
        DATE_FIELDS.iter().position(|name| *name == field.text()).map(|index| index as i32 + 1)
    });
    match field {
        Some(1) => time.second() as i32,
        Some(2) => time.minute() as i32,
        Some(3) => time.hour() as i32,
        Some(4) => time.weekday().num_days_from_sunday() as i32,
        Some(5) => time.day() as i32,
        Some(6) => time.month() as i32,
        Some(7) => time.year(),
        Some(8) => time.ordinal0() as i32,
        Some(9) => time.year() * 10_000 + time.month() as i32 * 100 + time.day() as i32,
        _ => -1,
    }
}

pub fn find_item<'a>(configuration: &'a GlobalConfigService, id_or_name: &Value) -> Option<&'a ItemModel> {
    if let Ok(id) = id_or_name.number_value() {
        configuration.find_item(id)
    } else {
        let name = id_or_name.text();
        configuration.find_item_by_name(&name).or_else(|| super::game_data::data().item_aliases.get(&name).and_then(|id| configuration.find_item(*id)))
    }
}

pub fn item_information(item: Option<&ItemModel>, field: &Value, configuration: &GlobalConfigService) -> Reply {
    let field = field.number_value().ok().or_else(|| {
        ITEM_FIELDS.iter().position(|name| *name == field.text()).map(|index| index as i32)
    });
    let Some(item) = item else {
        return Ok(if field == Some(18) { Value::String(String::new()) } else { (-1).into() });
    };
    let subtype = item.weapon_type.map(|weapon| weapon.value()).or_else(|| item.ammo_type.map(|ammo| ammo.value())).unwrap_or(0) as i32;
    let value = match field {
        Some(0) => item.price_buy.unwrap_or(0),
        Some(1) => item.price_sell.unwrap_or_else(|| item.price_buy.unwrap_or(0) / 2),
        Some(2) => item.item_type.value() as i32,
        Some(3) => configuration.item_max_drop_chance(item.id),
        Some(4) => match item.gender.as_deref() { Some("Female") | Some("F") => 0, Some("Male") | Some("M") => 1, _ => 2 },
        Some(5) => item.location as i32,
        Some(6) => item.weight,
        Some(7) => i32::from(item.attack.unwrap_or(0)),
        Some(8) => i32::from(item.defense.unwrap_or(0)),
        Some(9) => i32::from(item.range.unwrap_or(0)),
        Some(10) => i32::from(item.slots.unwrap_or(0)),
        Some(11) => if item.weapon_type.is_some() || item.ammo_type.is_some() { subtype } else { item.view.unwrap_or(0) },
        Some(12) => i32::from(item.equip_level_min.unwrap_or(0)),
        Some(13) => i32::from(item.weapon_level.unwrap_or(0)),
        Some(14) => item.alias_name.as_ref().and_then(|name| configuration.find_item_by_name(name)).map_or(0, |alias| alias.id),
        Some(15) => i32::from(item.equip_level_max.unwrap_or(0)),
        Some(16) => 0,
        Some(17) => item.id,
        Some(18) => return Ok(item.name_aegis.clone().into()),
        Some(19) => i32::from(item.armor_level.unwrap_or(0)),
        Some(20) => subtype,
        _ => -1,
    };
    Ok(value.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_queries_match_local_calendar_and_zero_based_day_of_year() {
        let date = chrono::Utc.with_ymd_and_hms(2009, 9, 13, 14, 15, 16).unwrap();
        assert_eq!(time_value(date, &1.into()), 16);
        assert_eq!(time_value(date, &2.into()), 15);
        assert_eq!(time_value(date, &3.into()), 14);
        assert_eq!(time_value(date, &4.into()), 0);
        assert_eq!(time_value(date, &5.into()), 13);
        assert_eq!(time_value(date, &6.into()), 9);
        assert_eq!(time_value(date, &7.into()), 2009);
        assert_eq!(time_value(date, &8.into()), 255);
        assert_eq!(time_value(date, &9.into()), 20_090_913);
        assert_eq!(time_value(date, &0.into()), -1);
        assert_eq!(time_value(date, &"DT_MONTH".into()), 9);
    }

    #[test]
    fn named_date_and_visual_constants_resolve_to_protocol_values() {
        assert_eq!(constant("SEPTEMBER"), Some(9.into()));
        assert_eq!(constant("SUNDAY"), Some(0.into()));
        assert_eq!(constant("ITEMINFO_AEGISNAME"), Some(18.into()));
        assert!(constant("EF_COIN").is_some_and(|value| value.is_number()));
    }
}
