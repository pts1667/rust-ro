use models::enums::bonus::BonusType;
use models::enums::item::ItemGroup;
use models::enums::EnumWithNumberValue;
use models::status::{KnownSkill, StatusSnapshot};
use models::status_change::StatusChangeKind;

pub fn scale_item_healing(snapshot: &StatusSnapshot, skills: &[KnownSkill], item_id: u32,
    hp: i64, sp: i64, ranked_potion: bool, rogue_spirit: bool) -> (i64, i64) {
    let skill = |name: &str| skills.iter().find(|skill| skill.value.to_name() == name).map_or(0, |skill| skill.level as i64);
    let mut hp_rate = 100 + 2 * snapshot.vit() as i64 + 10 * skill("SM_RECOVERY") + 5 * skill("AM_LEARNINGPOTION");
    let mut sp_rate = 100 + 2 * snapshot.int() as i64 + 10 * skill("MG_SRECOVERY") + 5 * skill("AM_LEARNINGPOTION");
    if ranked_potion {
        hp_rate += hp_rate / 2;
        sp_rate += sp_rate / 2;
        if rogue_spirit { hp_rate *= 2; }
    }
    let mut hp_group = 0_i64;
    let mut sp_group = 0_i64;
    let mut hp_item = 0_i64;
    let mut sp_item = 0_i64;
    for bonus in snapshot.bonuses() {
        match *bonus.bonus() {
            BonusType::HpRegenFromItemPercentage(value) => hp_rate += value as i64,
            BonusType::SpRegenFromItemPercentage(value) => sp_rate += value as i64,
            BonusType::HpRegenFromItemIDPercentage(id, value) if id == item_id => hp_item += value as i64,
            BonusType::SpRegenFromItemIDPercentage(id, value) if id == item_id => sp_item += value as i64,
            BonusType::HpRegenFromItemGroupPercentage(group, value) if group_contains(group, item_id) => hp_group += value as i64,
            BonusType::SpRegenFromItemGroupPercentage(group, value) if group_contains(group, item_id) => sp_group += value as i64,
            BonusType::HpRegenFromHerbPercentage(value) if group_contains(ItemGroup::Herb.value() as i32, item_id) => hp_group += value as i64,
            BonusType::HpRegenFromFruitPercentage(value) if group_contains(ItemGroup::Fruit.value() as i32, item_id) => hp_group += value as i64,
            BonusType::HpRegenFromMeatPercentage(value) if group_contains(ItemGroup::Meat.value() as i32, item_id) => hp_group += value as i64,
            BonusType::HpRegenFromCandyPercentage(value) if group_contains(ItemGroup::Candy.value() as i32, item_id) => hp_group += value as i64,
            BonusType::HpRegenFromJuicePercentage(value) if group_contains(ItemGroup::Juice.value() as i32, item_id) => hp_group += value as i64,
            BonusType::HpRegenFromFishPercentage(value) if group_contains(ItemGroup::Fish.value() as i32, item_id) => hp_group += value as i64,
            BonusType::HpRegenFromFoodPercentage(value) if group_contains(ItemGroup::Food.value() as i32, item_id) => hp_group += value as i64,
            BonusType::HpRegenFromPotionPercentage(value) if group_contains(ItemGroup::Potion.value() as i32, item_id) => hp_group += value as i64,
            _ => {}
        }
    }
    hp_rate += hp_rate * hp_group / 100;
    hp_rate += hp_rate * hp_item / 100;
    sp_rate += sp_rate * sp_group / 100;
    sp_rate += sp_rate * sp_item / 100;
    let enhanced = |amount: i64, rate: i64| if amount <= 0 { amount } else {
        amount.max((amount as i128 * rate as i128 / 100).clamp(0, i64::MAX as i128) as i64)
    };
    let hp = enhanced(hp, hp_rate);
    let sp = enhanced(sp, sp_rate);
    let penalty = if snapshot.has_status_change(StatusChangeKind::NoRecovery) { 100 } else {
        snapshot.status_change(StatusChangeKind::CriticalWound).map_or(0, |change| {
            if change.values[1] != 0 { change.values[1] } else { change.values[0] * 20 }
        }).clamp(0, 100)
    };
    let penalized = |amount: i64| if amount <= 0 { amount } else { amount - amount * penalty as i64 / 100 };
    (penalized(hp), penalized(sp))
}

fn group_contains(group: i32, item_id: u32) -> bool {
    crate::server::script::game_data::group(&group.into()).is_some_and(|group| {
        group.subgroups.iter().any(|subgroup| subgroup.entries.iter().any(|entry| entry.item_id as u32 == item_id))
    })
}

#[cfg(test)]
mod tests {
    use models::enums::skill_enums::SkillEnum;
    use models::status::{Status, StatusSnapshot};
    use models::status_bonus::StatusBonus;
    use models::status_change::StatusChange;

    use super::*;

    #[test]
    fn vitality_general_group_and_item_rates_follow_classic_order() {
        let mut snapshot = StatusSnapshot::_from(&Status { vit: 10, int: 5, ..Default::default() });
        snapshot.set_bonuses(vec![StatusBonus::new(BonusType::HpRegenFromItemPercentage(50)),
            StatusBonus::new(BonusType::HpRegenFromPotionPercentage(100)),
            StatusBonus::new(BonusType::HpRegenFromItemIDPercentage(501, 50)),
            StatusBonus::new(BonusType::SpRegenFromItemPercentage(50)),
            StatusBonus::new(BonusType::SpRegenFromItemIDPercentage(501, 50))]);
        let skills = [KnownSkill { value: SkillEnum::SmRecovery, level: 1 }];
        assert_eq!(scale_item_healing(&snapshot, &skills, 501, 100, 100, false, false), (540, 240));
        assert_eq!(scale_item_healing(&snapshot, &skills, 502, 100, 100, false, false), (360, 160));
    }

    #[test]
    fn ranked_potions_and_rogue_spirit_scale_before_item_bonuses() {
        let snapshot = StatusSnapshot::_from(&Status::default());
        assert_eq!(scale_item_healing(&snapshot, &[], 501, 100, 100, true, true), (300, 150));
        assert_eq!(scale_item_healing(&snapshot, &[], 501, 100, 100, false, true), (100, 100));
    }

    #[test]
    fn critical_wound_reduces_both_heals_after_bonuses_and_no_recovery_preserves_damage() {
        let mut snapshot = StatusSnapshot::_from(&Status::default());
        snapshot.set_bonuses(vec![StatusBonus::new(BonusType::HpRegenFromItemPercentage(100))]);
        let mut wound = StatusChange { kind: StatusChangeKind::CriticalWound, values: [2,40,0,0],
            started_at: 0, expires_at: None, next_periodic_at: 0, flags: 0, inherited_from: None };
        snapshot.set_active_statuses(vec![wound.clone()]);
        assert_eq!(scale_item_healing(&snapshot, &[], 501, 100, 100, false, false), (120,60));
        wound.kind = StatusChangeKind::NoRecovery;
        snapshot.set_active_statuses(vec![wound]);
        assert_eq!(scale_item_healing(&snapshot, &[], 501, 100, -10, false, false), (0,-10));
    }
}
