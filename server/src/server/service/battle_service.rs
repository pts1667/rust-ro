use std::borrow::Cow;
use std::mem;
use std::sync::Once;
use std::sync::mpsc::SyncSender;

use models::enums::action::ActionType;
use models::enums::bonus::BonusType;
use models::enums::class::{JobFamily, JobName};
use models::enums::element::Element;
use models::enums::mob::{MobClass, MobDamageMode, MobGroup, MobMode, MobRace};
use models::enums::size::Size;
use models::enums::skill::SkillState;
use models::enums::skill_enums::SkillEnum;
use models::enums::weapon::WeaponType;
use models::enums::{EnumStackable, EnumWithMaskValueU32, EnumWithMaskValueU64, EnumWithNumberValue, EnumWithStringValue};
use models::status::StatusSnapshot;
use models::status_bonus::{BattleFlag, StatusBonus};
use packets::packets::PacketZcNotifyAct;
use skills::OffensiveSkill;

use crate::packets::packets::Packet;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::map_item::{MapItemSnapshot, MapItemType};
use crate::server::service::combat_trigger_service::MagicReflectionKind;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_combat_service::{GrandCrossAttackContext, MagicAttackContext};
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;

static mut SERVICE_INSTANCE: Option<BattleService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct BattleService {
    client_notification_sender: SyncSender<Notification>,
    status_service: &'static StatusService,
    configuration_service: &'static GlobalConfigService,
    battle_result_mode: BattleResultMode,
}

pub enum BattleResultMode {
    TestMin,
    TestMax,
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalAttackRoll {
    Hit,
    Miss,
    LuckyDodge,
    Critical,
    DoubleAttack,
}

#[derive(Default, Clone, Copy)]
struct SkillDamageRules {
    ignore_defense: bool,
    ignore_attack_cards: bool,
    ignore_defense_cards: bool,
    ignore_element: bool,
}

#[derive(Default, Clone, Copy)]
struct PhysicalAttackOverrides {
    soft_defense_roll: Option<u64>,
    base_attack: Option<i32>,
    raw_attack: Option<u32>,
    secondary_hand: bool,
    sphere_hits: u16,
    skill_level: u8,
}

impl SkillDamageRules {
    fn from_skill(skill_id: u32) -> Self {
        let metadata = crate::server::script::skill::metadata::SkillMetadata::find(skill_id);
        let enabled = |name| metadata.is_some_and(|skill| skill.damage_flags.get(name).copied().unwrap_or(false));
        Self {
            ignore_defense: enabled("IgnoreDefense"),
            ignore_attack_cards: enabled("IgnoreAtkCard"),
            ignore_defense_cards: enabled("IgnoreDefCard"),
            ignore_element: enabled("IgnoreElement"),
        }
    }
}

#[cfg(test)]
mod equipment_bonus_tests {
    use models::status_bonus::StatusBonus;

    use super::*;

    fn context() -> BattleService {
        crate::tests::common::before_all();
        StatusService::init(GlobalConfigService::instance(), crate::tests::common::test_script_vm());
        let (sender, _) = std::sync::mpsc::sync_channel(64);
        BattleService::new(
            sender,
            StatusService::instance(),
            GlobalConfigService::instance(),
            BattleResultMode::TestMax,
        )
    }

    fn status() -> StatusSnapshot {
        StatusSnapshot::new_for_mob(
            1002,
            200,
            20,
            200,
            20,
            1,
            1,
            0,
            1,
            1,
            1,
            100,
            100,
            100,
            100,
            150,
            0,
            0,
            Size::Medium,
            Element::Neutral,
            MobRace::Plant,
            1,
        )
    }

    fn weapon_status(weapons: &[(WeaponType, models::enums::item::EquipmentLocation)]) -> StatusSnapshot {
        let mut raw = models::status::Status::default();
        raw.str = 0;
        raw.dex = 0;
        raw.luk = 0;
        for (index, (weapon_type, location)) in weapons.iter().enumerate() {
            raw.weapons.push(models::item::WearWeapon {
                item_id: 1201,
                attack: 100,
                level: 1,
                weapon_type: *weapon_type,
                location: location.as_flag(),
                refine: 0,
                element: Element::Neutral,
                card0: 0,
                card1: 0,
                card2: 0,
                card3: 0,
                ranked_forged: false,
                inventory_index: index,
                range: 1,
            });
        }
        StatusSnapshot::_from(&raw)
    }

    #[test]
    fn conditional_weapon_attack_adds_base_attack_only_for_the_actual_combined_weapon() {
        use models::enums::item::EquipmentLocation::{HandLeft, HandRight};
        let service = context();
        let mut target = status();
        target.set_def(50);
        target.set_base_vit(0);
        target.set_size(Size::Medium);
        let mut source = weapon_status(&[(WeaponType::Sword1H, HandRight)]);
        BonusType::ConditionalWeaponAtk(WeaponType::Dagger, 100).add_bonus_to_status(&mut source);
        assert_eq!(source.bonus_atk(), 0);
        BonusType::ConditionalWeaponAtk(WeaponType::Sword1H, 100).add_bonus_to_status(&mut source);
        assert_eq!(
            service.physical_damage(&source, &target, 1.0, false, &Element::Neutral, true, 0),
            200
        );
        assert_eq!(
            service.physical_damage(&source, &target, 1.0, false, &Element::Neutral, false, 0),
            99
        );
        let mut left_only = weapon_status(&[(WeaponType::Dagger, HandLeft)]);
        BonusType::ConditionalWeaponAtk(WeaponType::Dagger, 100).add_bonus_to_status(&mut left_only);
        assert_eq!(left_only.bonus_atk(), 100);
        let mut dual = weapon_status(&[(WeaponType::Dagger, HandRight), (WeaponType::Sword1H, HandLeft)]);
        assert_eq!(dual.combined_weapon_type(), WeaponType::DoubleDs);
        BonusType::ConditionalWeaponAtk(WeaponType::Dagger, 100).add_bonus_to_status(&mut dual);
        BonusType::ConditionalWeaponAtk(WeaponType::DoubleDs, 100).add_bonus_to_status(&mut dual);
        assert_eq!(dual.bonus_atk(), 0);
    }

    #[test]
    fn conditional_weapon_damage_rates_apply_per_hand_before_defense_and_splash_split() {
        use models::enums::item::EquipmentLocation::{HandLeft, HandRight};
        let service = context();
        let mut source = weapon_status(&[(WeaponType::Sword1H, HandRight), (WeaponType::Dagger, HandLeft)]);
        source.set_job(JobName::Assassin.value() as u32);
        source.set_bonus_atk(100);
        source.set_known_skills(vec![
            models::status::KnownSkill {
                value: SkillEnum::AsRight,
                level: 5,
            },
            models::status::KnownSkill {
                value: SkillEnum::AsLeft,
                level: 5,
            },
        ]);
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::EnableIgnoreSizeModifier),
            StatusBonus::new(BonusType::ConditionalWeaponDamagePercentage(WeaponType::Sword1H, 50)),
            StatusBonus::new(BonusType::ConditionalWeaponDamagePercentage(WeaponType::Dagger, 100)),
        ]);
        let mut target = status();
        target.set_def(0);
        target.set_base_vit(0);
        target.set_size(Size::Medium);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Critical),
            (300, 320)
        );
        target.set_def(50);
        assert_eq!(
            service.player_physical_splash_damage(&source, &target, 1.0, 1, false, &Element::Neutral, SkillEnum::SmBash.id(), 2),
            74
        );
    }

    #[test]
    fn skill_resistance_precedes_card_defenses_and_magic_resistance_precedes_mdef() {
        let service = context();
        let mut source = status();
        let mut target = status();
        let physical_id = SkillEnum::SmBash.id();
        let magic_id = SkillEnum::MgFirebolt.id();
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::SkillIdDamagePercentage(physical_id, 50)),
            StatusBonus::new(BonusType::SkillIdDamagePercentage(magic_id, 50)),
            StatusBonus::new(BonusType::PhysicalDamageAgainstRacePercentage(MobRace::All, 20)),
        ]);
        target.set_def(50);
        target.set_base_vit(10);
        target.set_mdef(50);
        target.set_base_int(10);
        target.set_bonuses(vec![
            StatusBonus::new(BonusType::ResistanceSkillIdPercentage(physical_id, 50)),
            StatusBonus::new(BonusType::ResistanceSkillIdPercentage(magic_id, 50)),
            StatusBonus::new(BonusType::ResistanceDamageFromRacePercentage(MobRace::All, 20)),
        ]);
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag();
        assert_eq!(
            service.actor_physical_skill_damage(100, &source, &target, false, 1.0, 1, &Element::Neutral, flags, physical_id),
            28
        );
        assert_eq!(
            service.magic_damage_from_context(
                &source,
                &target,
                MagicAttackContext::new(100, 1.0, Element::Neutral, 1, magic_id)
            ),
            21
        );
        target
            .bonuses_mut()
            .retain(|bonus| !matches!(bonus.bonus(), BonusType::ResistanceSkillIdPercentage(..)));
        assert_eq!(
            service.magic_damage_from_context(
                &source,
                &target,
                MagicAttackContext::new(100, 1.0, Element::Neutral, 1, magic_id)
            ),
            52
        );
    }

    #[test]
    fn misc_defense_compounds_before_skill_resistance_and_pressure_ignores_card_defense() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        let mut target = status();
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::SkillIdDamagePercentage(SkillEnum::TfThrowstone.id(), 20)),
            StatusBonus::new(BonusType::SkillIdDamagePercentage(SkillEnum::PaPressure.id(), 20)),
        ]);
        target.set_bonuses(vec![
            StatusBonus::new(BonusType::ResistanceMiscAttackPercentage(25)),
            StatusBonus::new(BonusType::ResistanceRangeAttackPercentage(20)),
            StatusBonus::new(BonusType::ResistanceSkillIdPercentage(SkillEnum::TfThrowstone.id(), 50)),
            StatusBonus::new(BonusType::ResistanceSkillIdPercentage(SkillEnum::PaPressure.id(), 50)),
        ]);
        let stone = skills::skill_enums::to_object(SkillEnum::TfThrowstone, 1).unwrap();
        let pressure = skills::skill_enums::to_object(SkillEnum::PaPressure, 5).unwrap();
        assert_eq!(service.calculate_damage(&source, &target, stone.as_offensive_skill()), 18);
        assert_eq!(service.calculate_damage(&source, &target, pressure.as_offensive_skill()), 1200);
        let normal = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag();
        assert_eq!(
            service.apply_damage_reduction(100.0, &source, &target, &Element::Neutral, normal),
            100.0
        );
        target
            .bonuses_mut()
            .push(StatusBonus::new(BonusType::ResistanceMiscAttackPercentage(100)));
        assert_eq!(service.calculate_damage(&source, &target, stone.as_offensive_skill()), 0);
    }

    #[test]
    fn actual_guardian_battlefield_and_event_classes_drive_attack_and_defense_filters() {
        let service = context();
        let mut source = status();
        let mut target = status();
        target.set_def(50);
        target.set_base_vit(0);
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::PhysicalDamageAgainstClassPercentage(MobClass::Guardian, 50)),
            StatusBonus::new(BonusType::IgnoreDefClass(MobClass::Guardian)),
        ]);
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag();
        target.set_mob_class(MobClass::Guardian);
        assert_eq!(
            service.actor_physical_damage(100, &source, &target, false, false, &Element::Neutral, flags),
            150
        );
        target.set_mob_class(MobClass::Battlefield);
        assert_eq!(
            service.actor_physical_damage(100, &source, &target, false, false, &Element::Neutral, flags),
            50
        );
        source.set_mob_class(MobClass::Battlefield);
        target.set_bonuses(vec![StatusBonus::new(BonusType::ResistanceDamageFromClassPercentage(
            MobClass::Battlefield,
            40,
        ))]);
        assert_eq!(
            service.actor_physical_damage(100, &source, &target, false, false, &Element::Neutral, flags),
            30
        );
        source.set_mob_class(MobClass::Event);
        assert_eq!(
            service.actor_physical_damage(100, &source, &target, false, false, &Element::Neutral, flags),
            50
        );
    }

    #[test]
    fn flee_rate_follows_flat_flee_and_precedes_learned_dodge_and_blind() {
        let service = context();
        let mut character = crate::tests::common::character_helper::create_character();
        character.status.base_level = 50;
        character.status.agi = 20;
        character.status.known_skills.push(models::status::KnownSkill {
            value: SkillEnum::TfMiss,
            level: 5,
        });
        character
            .status
            .temporary_bonuses
            .add(models::status_bonus::TemporaryStatusBonus::with_duration(
                BonusType::Flee(10),
                0,
                0,
                1000,
                1,
            ));
        character
            .status
            .temporary_bonuses
            .add(models::status_bonus::TemporaryStatusBonus::with_duration(
                BonusType::FleePercentage(50),
                0,
                0,
                1000,
                1,
            ));
        assert_eq!(service.status_service.to_snapshot(&character.status).flee(), 135);
        character.status.active_statuses.push(models::status_change::StatusChange {
            kind: models::status_change::StatusChangeKind::Blind,
            values: [1, 0, 0, 0],
            started_at: 0,
            expires_at: None,
            next_periodic_at: 0,
            inherited_from: None,
            flags: 0,
        });
        assert_eq!(service.status_service.to_snapshot(&character.status).flee(), 101);
        let mut snapshot = status();
        snapshot.set_flee(100);
        BonusType::FleePercentage(-150).add_percentage_bonus_to_status(&mut snapshot);
        assert_eq!(snapshot.flee(), 0);
    }

    #[test]
    fn race_size_element_and_class_bonuses_stack_as_separate_multipliers() {
        let service = context();
        let mut source = status();
        let target = status();
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::PhysicalDamageAgainstElementPercentage(Element::AllElement, 20)),
            StatusBonus::new(BonusType::PhysicalDamageAgainstRacePercentage(MobRace::All, 50)),
            StatusBonus::new(BonusType::PhysicalDamageAgainstSizePercentage(Size::All, 10)),
            StatusBonus::new(BonusType::PhysicalDamageAgainstClassPercentage(MobClass::All, 20)),
        ]);
        assert_eq!(service.apply_damage_bonus_modifier(100.0, &source, &target), 237.0);
    }

    #[test]
    fn element_resistance_with_battle_flags_only_reduces_matching_attacks() {
        let service = context();
        let source = status();
        let mut target = status();
        target.set_bonuses(vec![StatusBonus::new(BonusType::ResistanceDamageFromElementWithFlags(
            (Element::Neutral, BattleFlag::normalize(BattleFlag::Magic.as_flag(), false)),
            25,
        ))]);
        assert_eq!(
            service.apply_damage_reduction(
                100.0,
                &source,
                &target,
                &Element::Neutral,
                BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag()
            ),
            75.0
        );
        assert_eq!(
            service.apply_damage_reduction(
                100.0,
                &source,
                &target,
                &Element::Neutral,
                BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag()
            ),
            100.0
        );
    }

    #[test]
    fn critical_damage_bonus_does_not_increase_noncritical_damage() {
        let service = context();
        let target = status();
        let mut source = status();
        let normal = service.physical_damage(&source, &target, 1.0, false, &Element::Neutral, false, 0);
        source.set_bonuses(vec![StatusBonus::new(BonusType::CriticalDamagePercentage(50))]);
        assert_eq!(
            service.physical_damage(&source, &target, 1.0, false, &Element::Neutral, false, 0),
            normal
        );
        assert_eq!(
            service.physical_damage(&source, &target, 1.0, false, &Element::Neutral, true, 0),
            normal * 150 / 100
        );
    }

    #[test]
    fn critical_damage_rate_scales_raw_attack_before_refine_mastery_and_forged_stars() {
        let service = context();
        let mut raw = models::status::Status::default();
        raw.str = 0;
        raw.dex = 0;
        raw.luk = 0;
        raw.weapons.push(models::item::WearWeapon {
            item_id: 1101,
            attack: 100,
            level: 1,
            weapon_type: WeaponType::Sword1H,
            location: models::enums::item::EquipmentLocation::HandRight.as_flag(),
            refine: 5,
            element: Element::Neutral,
            card0: 255,
            card1: 5 * 256,
            card2: 0,
            card3: 0,
            ranked_forged: false,
            inventory_index: 0,
            range: 1,
        });
        raw.known_skills.push(models::status::KnownSkill {
            value: SkillEnum::SmSword,
            level: 10,
        });
        raw.known_skills.push(models::status::KnownSkill {
            value: SkillEnum::BsWeaponresearch,
            level: 10,
        });
        let mut source = StatusSnapshot::_from(&raw);
        let mut target = status();
        target.set_base_vit(0);
        target.set_def(90);
        assert_eq!(
            service.physical_damage(&source, &target, 1.0, false, &Element::Neutral, true, 0),
            175
        );
        source.set_bonuses(vec![StatusBonus::new(BonusType::CriticalDamagePercentage(50))]);
        assert_eq!(
            service.physical_damage(&source, &target, 1.0, false, &Element::Neutral, true, 0),
            225
        );
        target.set_def(0);
        assert_eq!(
            service.physical_damage(&source, &target, 1.0, false, &Element::Neutral, false, 0),
            174
        );
    }

    #[test]
    fn percentage_defense_ignore_reduces_integer_vitality_before_sampling() {
        let service = context();
        let mut source = status();
        source.set_bonuses(vec![StatusBonus::new(BonusType::IgnoreDefRacePercentage(MobRace::All, 50))]);
        let mut target = status();
        target.set_job(0);
        target.set_def(99);
        target.set_base_vit(99);
        assert_eq!(service.player_vitdef(&target), 78);
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag();
        assert_eq!(
            service.actor_physical_damage(1000, &source, &target, true, false, &Element::Neutral, flags),
            460
        );
        let (sender, _) = std::sync::mpsc::sync_channel(64);
        let minimum = BattleService::new(
            sender,
            StatusService::instance(),
            GlobalConfigService::instance(),
            BattleResultMode::TestMin,
        );
        assert_eq!(minimum.player_vitdef(&target), 113);
        assert_eq!(
            minimum.actor_physical_damage(1000, &source, &target, true, false, &Element::Neutral, flags),
            460
        );
        target.set_job(1002);
        target.set_base_vit(80);
        assert_eq!(
            service.actor_physical_damage(1000, &source, &target, false, false, &Element::Neutral, flags),
            460
        );
        assert_eq!(
            minimum.actor_physical_damage(1000, &source, &target, false, false, &Element::Neutral, flags),
            457
        );
    }

    #[test]
    fn classic_pressure_and_stone_fling_use_fixed_misc_damage_and_distinct_card_rules() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::AtkPercentage(100)),
            StatusBonus::new(BonusType::PhysicalDamageAgainstRacePercentage(MobRace::All, 100)),
            StatusBonus::new(BonusType::SkillIdDamagePercentage(SkillEnum::PaPressure.id(), 20)),
            StatusBonus::new(BonusType::SkillIdDamagePercentage(SkillEnum::TfThrowstone.id(), 20)),
        ]);
        let mut target = status();
        target.set_def(99);
        target.set_base_vit(99);
        target.set_mdef(99);
        target.set_bonuses(vec![
            StatusBonus::new(BonusType::ResistanceDamageFromRacePercentage(MobRace::All, 50)),
            StatusBonus::new(BonusType::ResistanceRangeAttackPercentage(20)),
        ]);
        let pressure = skills::skill_enums::to_object(SkillEnum::PaPressure, 5).unwrap();
        let stone = skills::skill_enums::to_object(SkillEnum::TfThrowstone, 1).unwrap();
        assert_eq!(service.calculate_damage(&source, &target, pressure.as_offensive_skill()), 2400);
        assert_eq!(service.calculate_damage(&source, &target, stone.as_offensive_skill()), 24);
        source.set_job(1002);
        assert_eq!(service.calculate_damage(&source, &target, stone.as_offensive_skill()), 14);
        target
            .bonuses_mut()
            .push(StatusBonus::new(BonusType::ResistancePhysicalAttackFromMobIdPercentage(
                1002, 100,
            )));
        assert_eq!(service.calculate_damage(&source, &target, stone.as_offensive_skill()), 14);
        target.set_element(Element::Ghost);
        target.set_element_level(4);
        assert_eq!(service.calculate_damage(&source, &target, stone.as_offensive_skill()), 0);
        assert_eq!(service.calculate_damage(&source, &target, pressure.as_offensive_skill()), 2400);
        target.set_job(1080);
        target.set_element(Element::Neutral);
        assert_eq!(service.calculate_damage(&source, &target, stone.as_offensive_skill()), 1);
        assert_eq!(service.calculate_damage(&source, &target, pressure.as_offensive_skill()), 1);
        assert_eq!(
            crate::server::service::script_combat_service::skill_flags(SkillEnum::TfThrowstone.id()),
            BattleFlag::Misc.as_flag() | BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag()
        );
        assert_eq!(
            crate::server::service::script_combat_service::skill_flags(SkillEnum::PaPressure.id()),
            BattleFlag::Misc.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag()
        );
    }

    #[test]
    fn ignoring_defense_applies_to_both_hard_and_vitality_defense() {
        let service = context();
        let mut target = status();
        target.set_def(90);
        target.set_base_vit(70);
        let mut source = status();
        let reduced = service.calculate_damage(&source, &target, None);
        source.set_bonuses(vec![StatusBonus::new(BonusType::IgnoreDefClass(MobClass::All))]);
        let ignored = service.calculate_damage(&source, &target, None);
        assert!(ignored > reduced);
        assert_eq!(ignored, 101);
    }

    #[test]
    fn equipment_reflection_reuses_spell_power_against_caster_defenses() {
        let service = context();
        let mut caster = status();
        caster.set_matk_min(500);
        caster.set_matk_max(500);
        caster.set_mdef(0);
        caster.set_bonuses(vec![StatusBonus::new(BonusType::ResistanceMagicAttackPercentage(50))]);
        let mut victim = status();
        victim.set_mdef(90);
        let context = MagicAttackContext {
            matk: 100,
            modifier: 1.0,
            element: Element::Neutral,
            hits: 3,
            skill_id: 0,
            grand_cross: None,
        };
        let original = service.magic_damage_from_context(&caster, &victim, context).max(0) as u32;
        let reflected = service
            .reflected_magic_damage(MagicReflectionKind::Equipment, original, &caster, Some(context))
            .unwrap();
        assert_eq!(
            reflected,
            service.magic_damage_from_context(&caster, &caster, context).max(0) as u32
        );
        assert!(reflected > original);
        assert!(reflected < 150);
        assert_eq!(
            service
                .reflected_magic_damage(MagicReflectionKind::Mirror, original, &caster, Some(context))
                .unwrap(),
            original
        );
        assert!(
            service
                .reflected_magic_damage(MagicReflectionKind::Equipment, original, &caster, None)
                .is_err()
        );
    }

    #[test]
    fn classic_mdef_rate_ignore_keeps_soft_defense_and_rounds_the_hard_reduction_first() {
        let service = context();
        let mut source = status();
        let mut target = status();
        target.set_mdef(51);
        target.set_base_int(10);
        target.set_base_vit(11);
        let context = MagicAttackContext::new(100, 1.0, Element::Neutral, 1, SkillEnum::MgFirebolt.id());
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 39);
        source.set_bonuses(vec![StatusBonus::new(BonusType::IgnoreMDefRacePercentage(MobRace::All, 50))]);
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 64);
        target.set_job(0);
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 59);
        source
            .bonuses_mut()
            .push(StatusBonus::new(BonusType::IgnoreMDefClassPercentage(MobClass::All, 50)));
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 85);
        let context = MagicAttackContext::new(100, 1.0, Element::Neutral, 1, SkillEnum::NpcEarthquake.id());
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 100);
    }

    #[test]
    fn signed_flat_attack_reduces_damage_and_berserk_adds_to_the_weapon_ratio_before_defense() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_base_str(100);
        source.set_base_dex(0);
        source.set_base_luk(0);
        BonusType::Atk(-30).add_bonus_to_status(&mut source);
        BonusType::Atk(10).add_bonus_to_status(&mut source);
        assert_eq!(source.bonus_atk(), -20);
        let mut target = status();
        target.set_base_vit(10);
        target.set_def(50);
        assert_eq!(service.calculate_damage(&source, &target, None), 80);
        source.set_active_statuses(vec![models::status_change::StatusChange {
            kind: models::status_change::StatusChangeKind::Berserk,
            values: [1, 0, 0, 0],
            started_at: 0,
            expires_at: None,
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        }]);
        assert_eq!(service.calculate_damage(&source, &target, None), 170);
        assert_eq!(
            service.player_physical_skill_damage(&source, &target, 3.0, 1, false, &Element::Neutral, SkillEnum::AsSplasher.id()),
            350
        );
        assert_eq!(
            BattleService::player_weapon_skill_ratio(&source, 3.0, SkillEnum::PaSacrifice.id()),
            3.0
        );
    }

    #[test]
    fn forged_star_damage_survives_a_miss_and_excluded_skills_do_not_use_it() {
        let service = context();
        let mut raw = models::status::Status::default();
        raw.weapons.push(models::item::WearWeapon {
            item_id: 1201,
            attack: 40,
            level: 1,
            weapon_type: WeaponType::Dagger,
            location: models::enums::item::EquipmentLocation::HandRight.as_flag(),
            refine: 0,
            element: Element::Fire,
            card0: 255,
            card1: 15 * 256 + 3,
            card2: 100,
            card3: 0,
            ranked_forged: false,
            inventory_index: 0,
            range: 1,
        });
        let source = StatusSnapshot::_from(&raw);
        let mut target = status();
        target.set_def(100);
        assert_eq!(BattleService::forged_star_damage(&source, 0), 40);
        assert_eq!(BattleService::forged_star_damage(&source, SkillEnum::MoExtremityfist.id()), 0);
        assert_eq!(
            service.normal_miss_damage(
                &source,
                &target,
                &Element::Fire,
                BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag()
            ),
            40
        );
    }

    #[test]
    fn weapon_skill_accuracy_uses_classic_final_rate_modifiers_and_caps() {
        let mut source = status();
        let mut target = status();
        source.set_hit(80);
        target.set_flee(80);
        assert_eq!(
            BattleService::skill_hit_rate(&source, &target, SkillEnum::AcDouble.id(), 10),
            80
        );
        assert_eq!(BattleService::skill_hit_rate(&source, &target, SkillEnum::SmBash.id(), 1), 84);
        assert_eq!(
            BattleService::skill_hit_rate(&source, &target, SkillEnum::KnPierce.id(), 10),
            95
        );
        assert_eq!(BattleService::skill_hit_rate(&source, &target, SkillEnum::SmMagnum.id(), 1), 88);
        source.set_known_skills(vec![models::status::KnownSkill {
            value: SkillEnum::AsSonicaccel,
            level: 1,
        }]);
        assert_eq!(
            BattleService::skill_hit_rate(&source, &target, SkillEnum::AsSonicblow.id(), 1),
            95
        );
        target.set_flee(1000);
        assert_eq!(BattleService::skill_hit_rate(&source, &target, SkillEnum::AcDouble.id(), 1), 5);
        assert_eq!(
            BattleService::skill_hit_rate(&source, &target, SkillEnum::PaShieldchain.id(), 1),
            25
        );
    }

    #[test]
    fn weapon_skill_admission_honors_guaranteed_hits_without_critical_or_lucky_rolls() {
        let mut source = status();
        let mut target = status();
        source.set_hit(0);
        target.set_flee(1000);
        target.set_perfect_dodge(100.0);
        let mut rng = fastrand::Rng::with_seed(12);
        assert!((0..100).all(|_| BattleService::skill_hits_with_rng(&source, &target, SkillEnum::PaSacrifice.id(), 1, &mut rng)));
        source.set_bonuses(vec![StatusBonus::new(BonusType::PerfectHitPercentage(100))]);
        assert!((0..100).all(|_| BattleService::skill_hits_with_rng(&source, &target, SkillEnum::SmBash.id(), 1, &mut rng)));
        source.set_bonuses(vec![]);
        let outcomes = (0..100)
            .map(|_| BattleService::skill_hits_with_rng(&source, &target, SkillEnum::SmBash.id(), 1, &mut rng))
            .collect::<Vec<_>>();
        assert!(outcomes.contains(&true) && outcomes.contains(&false));
        assert!(context().skill_hits(&source, &target, SkillEnum::SmBash.id(), 1));
    }

    #[test]
    fn companion_skill_damage_obeys_metadata_defense_and_card_exclusions() {
        let service = context();
        let mut source = status();
        let mut target = status();
        target.set_base_vit(0);
        target.set_def(80);
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        assert_eq!(
            service.actor_physical_skill_damage(
                100,
                &source,
                &target,
                false,
                1.0,
                5,
                &Element::Neutral,
                flags,
                SkillEnum::MlSpiralpierce.id()
            ),
            100
        );
        target.set_def(0);
        source.set_bonuses(vec![StatusBonus::new(BonusType::PhysicalDamageAgainstRacePercentage(
            MobRace::All,
            100,
        ))]);
        assert_eq!(
            service.actor_physical_skill_damage(
                100,
                &source,
                &target,
                false,
                1.0,
                1,
                &Element::Water,
                flags,
                SkillEnum::MaFreezingtrap.id()
            ),
            100
        );
        target.set_bonuses(vec![StatusBonus::new(BonusType::ResistanceDamageFromRacePercentage(
            MobRace::All,
            50,
        ))]);
        assert_eq!(
            service.actor_physical_skill_damage(
                100,
                &source,
                &target,
                false,
                1.0,
                1,
                &Element::Neutral,
                flags,
                SkillEnum::MaLandmine.id()
            ),
            200
        );
    }

    #[test]
    fn elemental_attack_buffs_and_magic_only_element_bonuses_have_distinct_paths() {
        let service = context();
        let mut source = status();
        let target = status();
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::DamageUsingElementPercentage(Element::Water, 20)),
            StatusBonus::new(BonusType::MagicalDamageUsingElementPercentage(Element::Water, 50)),
        ]);
        assert_eq!(
            service.apply_element_attack_bonus(100.0, &source, &Element::Water, false),
            120.0
        );
        assert_eq!(service.apply_element_attack_bonus(100.0, &source, &Element::Water, true), 180.0);
        assert_eq!(service.apply_element_attack_bonus(100.0, &source, &Element::Fire, true), 100.0);
        assert_eq!(
            service.apply_damage_reduction_with_rules(
                100.0,
                &source,
                &target,
                &Element::Neutral,
                BattleFlag::Magic.as_flag(),
                SkillDamageRules {
                    ignore_defense_cards: true,
                    ..Default::default()
                }
            ),
            100.0
        );
    }

    #[test]
    fn mercenary_land_mine_uses_misc_damage_without_weapon_or_defense_modifiers() {
        let service = context();
        let mut source = status();
        let mut target = status();
        target.set_def(99);
        target.set_base_vit(99);
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::PhysicalDamageAgainstRacePercentage(MobRace::All, 100)),
            StatusBonus::new(BonusType::SkillIdDamagePercentage(SkillEnum::MaLandmine.id(), 20)),
        ]);
        target.set_bonuses(vec![StatusBonus::new(BonusType::ResistanceDamageFromRacePercentage(
            MobRace::All,
            90,
        ))]);
        let flags = BattleFlag::Misc.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag();
        assert_eq!(
            service.actor_misc_skill_damage(100, &source, &target, &Element::Earth, flags, SkillEnum::MaLandmine.id()),
            120
        );
    }

    #[test]
    fn custom_player_skill_ratios_obey_splasher_card_exclusions_defense_and_skill_bonuses() {
        let service = context();
        let mut source = status();
        let mut target = status();
        target.set_base_vit(0);
        target.set_def(0);
        let skill = SkillEnum::AsSplasher.id();
        let plain = service.player_physical_skill_damage(&source, &target, 10.0, 1, false, &Element::Neutral, skill);
        assert!(plain > service.player_physical_skill_damage(&source, &target, 1.0, 1, false, &Element::Neutral, skill));
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::PhysicalDamageAgainstRacePercentage(MobRace::All, 100)),
            StatusBonus::new(BonusType::SkillIdDamagePercentage(skill, 50)),
        ]);
        assert_eq!(
            service.player_physical_skill_damage(&source, &target, 10.0, 1, false, &Element::Neutral, skill),
            plain * 150 / 100
        );
        target.set_def(90);
        assert!(service.player_physical_skill_damage(&source, &target, 10.0, 1, false, &Element::Neutral, skill) < plain);
    }

    #[test]
    fn splasher_splits_raw_attack_before_rates_and_defense_but_keeps_safe_refine() {
        let service = context();
        let mut raw = models::status::Status::default();
        raw.weapons.push(models::item::WearWeapon {
            item_id: 1201,
            attack: 0,
            level: 1,
            weapon_type: WeaponType::Dagger,
            location: models::enums::item::EquipmentLocation::HandRight.as_flag(),
            refine: 3,
            element: Element::Neutral,
            card0: 0,
            card1: 0,
            card2: 0,
            card3: 0,
            ranked_forged: false,
            inventory_index: 0,
            range: 1,
        });
        let mut source = StatusSnapshot::_from(&raw);
        source.set_base_str(100);
        source.set_base_dex(0);
        source.set_base_luk(0);
        source.set_bonuses(vec![StatusBonus::new(BonusType::AtkPercentage(50))]);
        let mut target = status();
        target.set_def(50);
        target.set_base_vit(10);
        let skill = SkillEnum::AsSplasher.id();
        let full = service.player_physical_skill_damage(&source, &target, 1.0, 1, false, &Element::Neutral, skill);
        let split = service.player_physical_splash_damage(&source, &target, 1.0, 1, false, &Element::Neutral, skill, 2);
        assert_eq!(full, 146);
        assert_eq!(split, 71);
        assert_ne!(split, full / 2);
        assert_eq!(
            service.player_physical_splash_damage(&source, &target, 1.0, 1, false, &Element::Neutral, skill, 3),
            45
        );
        assert_eq!(
            service.player_physical_splash_damage(&source, &target, 1.0, 1, false, &Element::Neutral, skill, 0),
            full
        );
    }

    #[test]
    fn dual_wield_uses_each_weapon_element_and_mastery_with_classic_card_unity() {
        let service = context();
        let mut raw = models::status::Status::default();
        raw.job = JobName::Assassin.value() as u32;
        for (index, location, attack, element) in [
            (0, models::enums::item::EquipmentLocation::HandRight, 100, Element::Neutral),
            (1, models::enums::item::EquipmentLocation::HandLeft, 50, Element::Fire),
        ] {
            raw.weapons.push(models::item::WearWeapon {
                item_id: 1201,
                attack,
                level: 1,
                weapon_type: WeaponType::Dagger,
                location: location.as_flag(),
                refine: 0,
                element,
                card0: 0,
                card1: 0,
                card2: 0,
                card3: 0,
                ranked_forged: false,
                inventory_index: index,
                range: 1,
            });
        }
        let mut source = StatusSnapshot::_from(&raw);
        source.set_base_str(100);
        source.set_base_dex(100);
        source.set_base_luk(0);
        source.set_bonuses(vec![StatusBonus::new(BonusType::PhysicalDamageAgainstRacePercentage(
            MobRace::All,
            50,
        ))]);
        source.set_left_hand_bonuses(vec![
            BonusType::PhysicalDamageAgainstRacePercentage(MobRace::All, 30),
            BonusType::ElementWeapon(Element::Fire),
        ]);
        let mut target = status();
        target.set_base_vit(0);
        target.set_def(0);
        target.set_size(Size::Small);
        target.set_element(Element::Earth);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (240, 121)
        );
        source.set_known_skills(vec![
            models::status::KnownSkill {
                value: SkillEnum::AsRight,
                level: 5,
            },
            models::status::KnownSkill {
                value: SkillEnum::AsLeft,
                level: 5,
            },
        ]);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (480, 324)
        );
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::DoubleAttack),
            (960, 324)
        );
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::LuckyDodge),
            (0, 0)
        );
        assert_eq!(
            service.player_physical_skill_damage(&source, &target, 1.0, 1, false, &Element::Neutral, SkillEnum::SmBash.id()),
            480
        );
        source.set_bonuses(vec![StatusBonus::new(BonusType::PhysicalDamageAgainstMobIdPercentage(
            target.job(),
            50,
        ))]);
        source.set_left_hand_bonuses(vec![BonusType::PhysicalDamageAgainstMobIdPercentage(target.job(), 50)]);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (320, 485)
        );
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::WeaponAtk(40)),
            StatusBonus::new(BonusType::WeaponRefineAtk(10)),
        ]);
        source.set_left_hand_bonuses(vec![
            BonusType::WeaponAtk(100),
            BonusType::WeaponRefineAtk(20),
            BonusType::ElementWeapon(Element::Fire),
        ]);
        assert_eq!((source.weapon_atk(), source.left_weapon_atk()), (140, 150));
        assert_eq!((source.weapon_upgrade_damage(), source.left_weapon_upgrade_damage()), (10, 20));
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (369, 466)
        );
        source.set_right_hand_weapon(None);
        assert_eq!(service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit).0, 0);
    }

    #[test]
    fn left_hand_defense_effects_transfer_to_the_right_hand_and_refine_follows_piercing() {
        let service = context();
        let mut raw = models::status::Status::default();
        for (index, location) in [
            models::enums::item::EquipmentLocation::HandRight,
            models::enums::item::EquipmentLocation::HandLeft,
        ]
        .into_iter()
        .enumerate()
        {
            raw.weapons.push(models::item::WearWeapon {
                item_id: 1201,
                attack: 100,
                level: 1,
                weapon_type: WeaponType::Dagger,
                location: location.as_flag(),
                refine: 0,
                element: Element::Neutral,
                card0: 0,
                card1: 0,
                card2: 0,
                card3: 0,
                ranked_forged: false,
                inventory_index: index,
                range: 1,
            });
        }
        raw.str = 100;
        raw.dex = 0;
        raw.luk = 0;
        let mut source = StatusSnapshot::_from(&raw);
        let mut target = status();
        target.set_def(50);
        target.set_base_vit(10);
        target.set_size(Size::Small);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (139, 139)
        );
        source.set_bonuses(vec![StatusBonus::new(BonusType::IgnoreDefClass(MobClass::Normal))]);
        source.set_left_hand_bonuses(vec![BonusType::IgnoreDefClass(MobClass::Normal)]);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (299, 139)
        );
        source.set_right_hand_weapon(None);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (0, 299)
        );
        raw.weapons[0].refine = 1;
        let mut source = StatusSnapshot::_from(&raw);
        source.set_bonuses(vec![StatusBonus::new(BonusType::IncreaseDamageAgainstClassBaseOnDef(
            MobClass::Normal,
        ))]);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (181, 139)
        );
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Critical),
            (302, 300)
        );
    }

    #[test]
    fn overthrust_and_berserk_add_to_the_skill_ratio_for_player_and_actor_damage() {
        use models::status_change::{StatusChange, StatusChangeKind};
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_base_str(100);
        source.set_base_dex(0);
        source.set_base_luk(0);
        source.set_active_statuses(vec![StatusChange {
            kind: StatusChangeKind::Overthrust,
            values: [5, 1, 25, 0],
            started_at: 0,
            expires_at: Some(5000),
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        }]);
        assert!(source.active_statuses()[0].bonuses().is_empty());
        let mut target = status();
        target.set_def(50);
        target.set_base_vit(10);
        assert_eq!(
            service.player_physical_skill_damage(&source, &target, 3.0, 1, false, &Element::Neutral, SkillEnum::SmBash.id()),
            315
        );
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag();
        assert_eq!(
            service.actor_physical_skill_damage(
                200,
                &source,
                &target,
                false,
                3.0,
                1,
                &Element::Neutral,
                flags,
                SkillEnum::MsBash.id()
            ),
            315
        );
        let mut statuses = source.active_statuses().clone();
        statuses.push(StatusChange {
            kind: StatusChangeKind::Berserk,
            values: [1, 0, 0, 0],
            started_at: 0,
            expires_at: Some(5000),
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        });
        source.set_active_statuses(statuses);
        assert_eq!(BattleService::weapon_skill_ratio(&source, 3.0, SkillEnum::SmBash.id()), 4.25);
        assert_eq!(
            service.player_physical_skill_damage(&source, &target, 3.0, 1, false, &Element::Neutral, SkillEnum::SmBash.id()),
            415
        );
        assert_eq!(
            BattleService::weapon_skill_ratio(&source, 3.0, SkillEnum::PaSacrifice.id()),
            3.0
        );
    }

    #[test]
    fn learned_masteries_apply_after_defense_before_element_and_weapon_research_remains_non_elemental() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_base_str(100);
        source.set_base_dex(0);
        source.set_base_luk(0);
        source.set_right_hand_weapon_type(WeaponType::Dagger);
        source.set_known_skills(vec![models::status::KnownSkill {
            value: SkillEnum::SmSword,
            level: 10,
        }]);
        source.set_bonuses(vec![StatusBonus::new(BonusType::ElementWeapon(Element::Fire))]);
        let mut target = status();
        target.set_def(90);
        target.set_base_vit(10);
        target.set_element(Element::Water);
        assert_eq!(service.calculate_damage(&source, &target, None), 25);
        let mut skills = source.known_skills().clone();
        skills.extend([
            models::status::KnownSkill {
                value: SkillEnum::BsWeaponresearch,
                level: 10,
            },
            models::status::KnownSkill {
                value: SkillEnum::BsHiltbinding,
                level: 1,
            },
        ]);
        source.set_known_skills(skills);
        assert_eq!(service.calculate_damage(&source, &target, None), 49);
        assert_eq!(
            service.normal_miss_damage(&source, &target, &Element::Fire, BattleFlag::Weapon.as_flag()),
            24
        );
        source
            .bonuses_mut()
            .push(StatusBonus::new(BonusType::PhysicalDamageAgainstRacePercentage(
                MobRace::All,
                100,
            )));
        assert_eq!(service.calculate_damage(&source, &target, None), 98);
        source.set_known_skills(vec![models::status::KnownSkill {
            value: SkillEnum::AlDemonbane,
            level: 10,
        }]);
        source.set_base_level(99);
        target.set_race(MobRace::Demon);
        assert_eq!(service.learned_weapon_mastery_damage(&source, &target), 79);
        target.set_job(0);
        assert_eq!(service.learned_weapon_mastery_damage(&source, &target), 0);
        source.set_known_skills(vec![models::status::KnownSkill {
            value: SkillEnum::HtBeastbane,
            level: 10,
        }]);
        target.set_race(MobRace::Brute);
        assert_eq!(service.learned_weapon_mastery_damage(&source, &target), 40);
    }

    #[test]
    fn every_classic_weapon_mastery_uses_its_actual_weapon_and_riding_changes_spear_mastery() {
        let service = context();
        let mut source = status();
        source.set_known_skills(
            [
                SkillEnum::SmSword,
                SkillEnum::SmTwohand,
                SkillEnum::KnSpearmastery,
                SkillEnum::AmAxemastery,
                SkillEnum::PrMacemastery,
                SkillEnum::TkRun,
                SkillEnum::MoIronhand,
                SkillEnum::BaMusicallesson,
                SkillEnum::DcDancinglesson,
                SkillEnum::SaAdvancedbook,
                SkillEnum::AsKatar,
            ]
            .into_iter()
            .map(|value| models::status::KnownSkill { value, level: 2 })
            .collect(),
        );
        let target = status();
        for (weapon, damage) in [
            (WeaponType::Dagger, 8),
            (WeaponType::Sword1H, 8),
            (WeaponType::Sword2H, 8),
            (WeaponType::Spear1H, 8),
            (WeaponType::Spear2H, 8),
            (WeaponType::Axe1H, 6),
            (WeaponType::Axe2H, 6),
            (WeaponType::Mace, 6),
            (WeaponType::Mace2H, 6),
            (WeaponType::Fist, 26),
            (WeaponType::Knuckle, 6),
            (WeaponType::Musical, 6),
            (WeaponType::Whip, 6),
            (WeaponType::Book, 6),
            (WeaponType::Katar, 6),
            (WeaponType::Bow, 0),
        ] {
            source.set_right_hand_weapon_type(weapon);
            assert_eq!(service.learned_weapon_mastery_damage(&source, &target), damage, "{:?}", weapon);
        }
        source.set_right_hand_weapon_type(WeaponType::Spear1H);
        source.set_state(SkillState::Riding.as_flag());
        assert_eq!(service.learned_weapon_mastery_damage(&source, &target), 10);
    }

    #[test]
    fn double_attack_uses_the_highest_rate_and_respects_weapon_and_temporary_skill_permissions() {
        let mut raw = models::status::Status::default();
        raw.known_skills.push(models::status::KnownSkill {
            value: SkillEnum::TfDouble,
            level: 10,
        });
        let mut source = StatusSnapshot::_from(&raw);
        source.set_right_hand_weapon_type(WeaponType::Dagger);
        source.set_bonuses(vec![StatusBonus::new(BonusType::PerfectHitPercentage(100))]);
        let mut target = status();
        target.set_perfect_dodge(0.0);
        assert_eq!(BattleService::double_attack_chance(&source), 50);
        for seed in 0..128 {
            let expected = if fastrand::Rng::with_seed(seed).f32() * 100.0 < 50.0 {
                NormalAttackRoll::DoubleAttack
            } else {
                NormalAttackRoll::Hit
            };
            assert_eq!(
                BattleService::normal_attack_roll(&source, &target, false, &mut fastrand::Rng::with_seed(seed)),
                expected
            );
        }
        source.bonuses_mut().extend([
            StatusBonus::new(BonusType::DoubleAttackChancePercentage(60)),
            StatusBonus::new(BonusType::DoubleAttackChancePercentage(70)),
            StatusBonus::new(BonusType::DoubleAttackAdditionalChancePercentage(5)),
        ]);
        assert_eq!(BattleService::double_attack_chance(&source), 75);
        source.set_bonuses(vec![]);
        source.set_right_hand_weapon_type(WeaponType::Katar);
        assert_eq!(BattleService::double_attack_chance(&source), 0);
        raw.script_skill_grants
            .insert(SkillEnum::TfDouble.id(), models::skill_grant::ScriptSkillGrant {
                learned_level: 0,
                current_level: 10,
            });
        let mut granted = StatusSnapshot::_from(&raw);
        granted.set_right_hand_weapon_type(WeaponType::Katar);
        assert_eq!(BattleService::double_attack_chance(&granted), 50);
        granted.set_right_hand_weapon_type(WeaponType::Fist);
        assert_eq!(BattleService::double_attack_chance(&granted), 0);
    }

    #[test]
    fn envenom_applies_its_mastery_addition_between_two_poison_attribute_checks() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_base_str(100);
        source.set_base_dex(0);
        source.set_base_luk(0);
        let mut target = status();
        target.set_def(50);
        target.set_base_vit(10);
        target.set_element(Element::Water);
        target.set_element_level(4);
        let skill = skills::skill_enums::to_object(SkillEnum::TfPoison, 5).unwrap();
        assert_eq!(service.calculate_damage(&source, &target, skill.as_offensive_skill()), 24);
    }

    #[test]
    fn classic_back_stab_bow_penalty_snatch_and_final_strike_use_source_stats_and_hp() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_base_str(100);
        source.set_base_dex(10);
        source.set_base_luk(0);
        source.set_hp(1000);
        let mut target = status();
        target.set_def(50);
        target.set_base_vit(10);
        let back_stab = skills::skill_enums::to_object(SkillEnum::RgBackstap, 10).unwrap();
        assert_eq!(service.calculate_damage(&source, &target, back_stab.as_offensive_skill()), 697);
        let snatch = skills::skill_enums::to_object(SkillEnum::RgIntimidate, 5).unwrap();
        assert_eq!(service.calculate_damage(&source, &target, snatch.as_offensive_skill()), 242);
        let final_strike = skills::skill_enums::to_object(SkillEnum::NjIssen, 10).unwrap();
        assert_eq!(
            service.calculate_damage(&source, &target, final_strike.as_offensive_skill()),
            2390
        );
        source.set_hp(1);
        assert_eq!(
            service.calculate_damage(&source, &target, final_strike.as_offensive_skill()),
            1990
        );
        source.set_right_hand_weapon_type(WeaponType::Bow);
        source.set_base_str(10);
        source.set_base_dex(100);
        assert_eq!(service.calculate_damage(&source, &target, back_stab.as_offensive_skill()), 394);
        assert!(!BattleService::attack_uses_ammo(&source, SkillEnum::RgBackstap.id()));
        assert!(BattleService::attack_uses_ammo(&source, SkillEnum::AcDouble.id()));
        assert!(BattleService::attack_uses_ammo(&source, 0));
    }

    #[test]
    fn weapon_attack_bonuses_use_weapon_variance_and_add_refine_after_defense_even_when_unarmed() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_base_str(100);
        source.set_base_dex(0);
        source.set_base_luk(0);
        let mut target = status();
        target.set_base_vit(0);
        target.set_def(50);
        assert_eq!(service.calculate_damage(&source, &target, None), 100);
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::WeaponAtk(100)),
            StatusBonus::new(BonusType::WeaponRefineAtk(10)),
        ]);
        assert_eq!(source.weapon_atk(), 100);
        assert_eq!(source.weapon_upgrade_damage(), 10);
        assert_eq!(service.calculate_damage(&source, &target, None), 159);
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::WeaponAtk(-100)),
            StatusBonus::new(BonusType::WeaponRefineAtk(-10)),
        ]);
        assert_eq!(source.weapon_atk(), 0);
        assert_eq!(source.weapon_upgrade_damage(), 0);
        assert_eq!(service.calculate_damage(&source, &target, None), 100);
        source.set_bonuses(vec![
            StatusBonus::new(BonusType::WeaponAtk(i32::MAX)),
            StatusBonus::new(BonusType::WeaponRefineAtk(i32::MAX)),
        ]);
        assert_eq!((source.weapon_atk(), source.weapon_upgrade_damage()), (u16::MAX, u16::MAX));
        source.set_bonuses(vec![StatusBonus::new(BonusType::WeaponAtk(3))]);
        source.set_base_dex(1);
        target.set_def(0);
        let minimum_service = BattleService {
            battle_result_mode: BattleResultMode::TestMin,
            ..context()
        };
        assert_eq!(minimum_service.calculate_damage(&source, &target, None), 201);
    }

    #[test]
    fn elemental_absorption_preserves_signed_weapon_magic_and_katar_totals() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_base_str(100);
        source.set_base_dex(0);
        source.set_base_luk(0);
        source.set_bonuses(vec![StatusBonus::new(BonusType::ElementWeapon(Element::Fire))]);
        let mut target = status();
        target.set_base_vit(0);
        target.set_base_int(0);
        target.set_def(0);
        target.set_mdef(0);
        target.set_element(Element::Fire);
        target.set_element_level(4);
        assert_eq!(service.calculate_damage(&source, &target, None), -100);
        assert_eq!(
            service.player_physical_skill_damage_signed(&source, &target, 2.0, 1, false, &Element::Fire, SkillEnum::SmBash.id()),
            -200
        );
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag();
        assert_eq!(
            service.actor_physical_damage_signed(200, &source, &target, false, false, &Element::Fire, flags),
            -100
        );
        let context = MagicAttackContext::new(100, 1.0, Element::Fire, 2, SkillEnum::MgFirebolt.id());
        assert_eq!(service.magic_damage_from_context(&source, &target, context), -100);
        assert_eq!(
            service
                .reflected_magic_damage_signed(MagicReflectionKind::Equipment, 100, &target, Some(context))
                .unwrap(),
            -100
        );
        source.set_right_hand_weapon_type(WeaponType::Katar);
        source.set_known_skills(vec![models::status::KnownSkill {
            value: SkillEnum::TfDouble,
            level: 10,
        }]);
        assert_eq!(
            service.normal_weapon_damage_signed_parts(&source, &target, NormalAttackRoll::Hit),
            (-100, -21)
        );
        assert_eq!(service.calculate_damage(&source, &target, None), -121);
        assert_eq!(
            service.normal_weapon_damage_signed_parts(&source, &target, NormalAttackRoll::LuckyDodge),
            (0, 0)
        );
    }

    #[test]
    fn katar_backhand_is_derived_after_double_attack_and_does_not_hit_plants() {
        let service = context();
        let mut raw = models::status::Status::default();
        raw.weapons.push(models::item::WearWeapon {
            item_id: 1250,
            attack: 100,
            level: 1,
            weapon_type: WeaponType::Katar,
            location: models::enums::item::EquipmentLocation::HandRight.as_flag(),
            refine: 0,
            element: Element::Neutral,
            card0: 0,
            card1: 0,
            card2: 0,
            card3: 0,
            ranked_forged: false,
            inventory_index: 0,
            range: 1,
        });
        let mut source = StatusSnapshot::_from(&raw);
        source.set_base_str(100);
        source.set_base_dex(100);
        source.set_base_luk(0);
        source.set_known_skills(vec![models::status::KnownSkill {
            value: SkillEnum::TfDouble,
            level: 10,
        }]);
        let mut target = status();
        target.set_base_vit(0);
        target.set_def(0);
        target.set_size(Size::Small);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (295, 61)
        );
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::DoubleAttack),
            (590, 123)
        );
        target.set_job(GlobalConfigService::instance().get_mob_by_name("GREEN_PLANT").id as u32);
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Hit),
            (1, 0)
        );
    }

    #[test]
    fn magnum_follow_up_uses_defended_normal_damage_and_spheres_remain_non_elemental() {
        let service = context();
        let mut source = StatusSnapshot::_from(&models::status::Status::default());
        source.set_base_str(100);
        source.set_base_dex(0);
        source.set_base_luk(0);
        source.set_spirit_sphere_count(5);
        let mut target = status();
        target.set_base_vit(10);
        target.set_def(50);
        target.set_element(Element::Earth);
        assert_eq!(service.calculate_damage(&source, &target, None), 105);
        source.set_active_statuses(vec![models::status_change::StatusChange {
            kind: models::status_change::StatusChangeKind::WeaponAttackElement,
            values: [Element::Fire.value() as i32, 20, 0, 0],
            started_at: 0,
            expires_at: Some(10000),
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        }]);
        assert_eq!(service.calculate_damage(&source, &target, None), 135);
        assert_eq!(
            service.player_physical_skill_damage(&source, &target, 3.0, 1, false, &Element::Neutral, SkillEnum::SmBash.id()),
            335
        );
        assert_eq!(
            service.player_physical_skill_damage(&source, &target, 3.0, 1, false, &Element::Neutral, SkillEnum::AsSplasher.id()),
            305
        );
        assert_eq!(
            service.normal_weapon_damage_parts(&source, &target, NormalAttackRoll::Miss),
            (15, 0)
        );
        source.set_spirit_sphere_count(2);
        assert_eq!(
            BattleService::spirit_sphere_damage(&source, SkillEnum::MoFingeroffensive.id(), 3),
            15
        );
        assert_eq!(
            BattleService::spirit_sphere_damage(&source, SkillEnum::LkSpiralpierce.id(), 1),
            0
        );
        assert_eq!(
            BattleService::spirit_sphere_damage(&source, SkillEnum::MoExtremityfist.id(), 1),
            0
        );
    }

    #[test]
    fn maximize_power_forces_the_weapon_roll_but_keeps_classic_over_refine_bounds() {
        let mut service = context();
        service.battle_result_mode = BattleResultMode::TestMin;
        let mut raw = models::status::Status::default();
        raw.weapons.push(models::item::WearWeapon {
            item_id: 1201,
            attack: 100,
            level: 1,
            weapon_type: WeaponType::Dagger,
            location: models::enums::item::EquipmentLocation::HandRight.as_flag(),
            refine: 8,
            element: Element::Neutral,
            card0: 0,
            card1: 0,
            card2: 0,
            card3: 0,
            ranked_forged: false,
            inventory_index: 0,
            range: 1,
        });
        let mut source = StatusSnapshot::_from(&raw);
        source.set_base_dex(1);
        let mut target = status();
        target.set_size(Size::Small);
        assert_eq!(service.weapon_atk_with_critical(&source, &target, false), 2);
        assert_eq!(service.weapon_atk_with_critical(&source, &target, true), 101);
        source.set_active_statuses(vec![models::status_change::StatusChange {
            kind: models::status_change::StatusChangeKind::MaximizePower,
            values: [1, 0, 0, 0],
            started_at: 0,
            expires_at: None,
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        }]);
        assert_eq!(service.weapon_atk_with_critical(&source, &target, false), 101);
        service.battle_result_mode = BattleResultMode::TestMax;
        assert_eq!(service.weapon_atk_with_critical(&source, &target, false), 103);
        assert_eq!(
            (
                BattleService::weapon_over_refine_limit(1, 7),
                BattleService::weapon_over_refine_limit(1, 8),
                BattleService::weapon_over_refine_limit(2, 7),
                BattleService::weapon_over_refine_limit(3, 6),
                BattleService::weapon_over_refine_limit(4, 5),
                BattleService::weapon_over_refine_limit(4, 10)
            ),
            (0, 3, 5, 8, 13, 78)
        );
    }

    #[test]
    fn weapon_endows_arrows_and_enchant_arms_follow_their_separate_element_precedence() {
        let service = context();
        let mut raw = models::status::Status::default();
        raw.weapons.push(models::item::WearWeapon {
            item_id: 1701,
            attack: 100,
            level: 1,
            weapon_type: WeaponType::Bow,
            location: models::enums::item::EquipmentLocation::HandRight.as_flag(),
            refine: 0,
            element: Element::Neutral,
            card0: 0,
            card1: 0,
            card2: 0,
            card3: 0,
            ranked_forged: false,
            inventory_index: 0,
            range: 1,
        });
        raw.ammo = Some(models::item::WearAmmo {
            item_id: 1752,
            inventory_index: 1,
            ammo_type: models::enums::weapon::AmmoType::Arrow,
            element: Element::Fire,
            attack: 3,
        });
        let mut source = StatusSnapshot::_from(&raw);
        source.set_bonuses(vec![StatusBonus::new(BonusType::ElementWeapon(Element::Holy))]);
        assert_eq!(service.attack_element(&source, None), Element::Fire);
        raw.ammo.as_mut().unwrap().element = Element::Neutral;
        source.set_ammo(raw.ammo.as_ref().map(|ammo| ammo.to_snapshot()));
        assert_eq!(service.attack_element(&source, None), Element::Holy);
        source.set_active_statuses(vec![models::status_change::StatusChange {
            kind: models::status_change::StatusChangeKind::EnchantArms,
            values: [Element::Wind.value() as i32, 0, 0, 0],
            started_at: 0,
            expires_at: None,
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        }]);
        assert_eq!(service.attack_element(&source, None), Element::Wind);
        let bash = skills::skill_enums::to_offensive_skill(SkillEnum::SmBash, 1).unwrap();
        assert_eq!(service.attack_element(&source, Some(bash.as_ref())), Element::Wind);
        let fire = skills::skill_enums::to_offensive_skill(SkillEnum::MgFirebolt, 1).unwrap();
        assert_eq!(service.attack_element(&source, Some(fire.as_ref())), Element::Fire);
        source.set_active_statuses(vec![]);
        raw.ammo.as_mut().unwrap().element = Element::Fire;
        source.set_ammo(raw.ammo.as_ref().map(|ammo| ammo.to_snapshot()));
        let back_stab = skills::skill_enums::to_offensive_skill(SkillEnum::RgBackstap, 1).unwrap();
        assert_eq!(service.attack_element(&source, Some(back_stab.as_ref())), Element::Holy);
        let musical_strike = skills::skill_enums::to_offensive_skill(SkillEnum::BaMusicalstrike, 1).unwrap();
        assert_eq!(service.attack_element(&source, Some(musical_strike.as_ref())), Element::Fire);
        let demon_shock = crate::server::script::skill::metadata::SkillMetadata::find(SkillEnum::NpcMagicalattack.id()).unwrap();
        assert_eq!(service.skill_attack_element(&source, demon_shock, 1), Element::Holy);
        let estun = crate::server::script::skill::metadata::SkillMetadata::find(SkillEnum::SlStun.id()).unwrap();
        assert_eq!(service.skill_attack_element(&source, estun, 1), Element::Neutral);
        source.set_active_statuses(vec![models::status_change::StatusChange {
            kind: models::status_change::StatusChangeKind::WaterWeapon,
            values: [1, 0, 0, 0],
            started_at: 0,
            expires_at: None,
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        }]);
        assert_eq!(service.skill_attack_element(&source, estun, 1), Element::Water);
        assert_eq!(service.skill_attack_element(&source, demon_shock, 1), Element::Water);
        assert_eq!(service.attack_element(&source, Some(musical_strike.as_ref())), Element::Fire);
    }

    #[test]
    fn grand_cross_combines_separately_defended_rolls_and_applies_target_element_twice() {
        let service = context();
        let mut source = status();
        source.set_job(0);
        let mut target = status();
        target.set_base_vit(0);
        target.set_base_int(10);
        target.set_def(50);
        target.set_mdef(40);
        target.set_element(Element::Undead);
        target.set_element_level(1);
        let mut context = MagicAttackContext::new(100, 5.0, Element::Holy, 1, SkillEnum::CrGrandcross.id());
        context.grand_cross = Some(GrandCrossAttackContext {
            raw_atk: 100,
            refine_bonus: 10,
            self_target: false,
        });
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 1237);
        source.set_matk_min(999);
        source.set_matk_max(999);
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 1237);
        target.set_element(Element::Neutral);
        context.grand_cross.as_mut().unwrap().self_target = true;
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 275);
        target.set_def(0);
        target.set_mdef(0);
        target.set_base_int(100);
        context.matk = 1;
        context.grand_cross = Some(GrandCrossAttackContext {
            raw_atk: 20,
            refine_bonus: 0,
            self_target: false,
        });
        assert_eq!(service.magic_damage_from_context(&source, &target, context), 5);
    }
}

fn sum_bonus(status: &StatusSnapshot, mut value: impl FnMut(&BonusType) -> Option<i32>) -> i32 {
    status
        .bonuses()
        .iter()
        .filter_map(|bonus| value(bonus.bonus()))
        .fold(0_i32, i32::saturating_add)
}

fn scale_damage(damage: f32, percent: i32) -> f32 {
    (damage * (100_i32.saturating_add(percent).max(0)) as f32 / 100.0).floor()
}

fn roll_percent(chance: f32, rng: &mut fastrand::Rng) -> bool {
    chance > 0.0 && (chance >= 100.0 || rng.f32() * 100.0 < chance)
}

fn known_skill_level(status: &StatusSnapshot, value: SkillEnum) -> i32 {
    status
        .known_skills()
        .iter()
        .filter(|skill| skill.value == value)
        .map(|skill| i32::from(skill.level))
        .max()
        .unwrap_or(0)
}

impl BattleService {
    pub fn skill_hit_rate(source: &StatusSnapshot, target: &StatusSnapshot, skill_id: u32, level: u8) -> u8 {
        let mut rate = 80 + i32::from(source.hit()) - i32::from(target.flee());
        let known = |value: SkillEnum| {
            source
                .known_skills()
                .iter()
                .find(|skill| skill.value == value)
                .map_or(0, |skill| i32::from(skill.level))
        };
        match SkillEnum::try_from_value(skill_id).ok() {
            Some(SkillEnum::SmBash | SkillEnum::KnPierce) => rate += rate * 5 * i32::from(level) / 100,
            Some(SkillEnum::SmMagnum) => rate += rate * 10 * i32::from(level) / 100,
            Some(SkillEnum::AsSonicblow) if known(SkillEnum::AsSonicaccel) > 0 => rate += rate * 50 / 100,
            _ => {}
        }
        rate += rate * 2 * known(SkillEnum::BsWeaponresearch) / 100;
        rate = rate.clamp(5, 95);
        if skill_id == SkillEnum::PaShieldchain.id() {
            rate += 20;
        }
        rate.clamp(0, 100) as u8
    }

    pub fn skill_hits(&self, source: &StatusSnapshot, target: &StatusSnapshot, skill_id: u32, level: u8) -> bool {
        if !matches!(self.battle_result_mode, BattleResultMode::Normal) {
            return true;
        }
        Self::skill_hits_with_rng(source, target, skill_id, level, &mut fastrand::Rng::new())
    }

    pub fn skill_hits_with_rng(
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        skill_id: u32,
        level: u8,
        rng: &mut fastrand::Rng,
    ) -> bool {
        use models::status_change::StatusChangeKind;
        if crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
            .is_some_and(|skill| skill.damage_flags.get("IgnoreFlee").copied().unwrap_or(false))
            || target.active_statuses().iter().any(|status| {
                matches!(
                    status.kind,
                    StatusChangeKind::Stone | StatusChangeKind::Freeze | StatusChangeKind::Stun | StatusChangeKind::Sleep
                )
            })
        {
            return true;
        }
        if skill_id == SkillEnum::CrShieldboomerang.id()
            && source
                .status_change(StatusChangeKind::Spirit)
                .is_some_and(|status| status.values[1] == SkillEnum::SlCrusader.id() as i32)
        {
            return true;
        }
        let perfect = sum_bonus(source, |bonus| match bonus {
            BonusType::PerfectHitPercentage(percent) => Some(i32::from(*percent)),
            _ => None,
        });
        roll_percent(perfect as f32, rng) || rng.u8(0..100) < Self::skill_hit_rate(source, target, skill_id, level)
    }

    pub fn normal_attack_roll(source: &StatusSnapshot, target: &StatusSnapshot, ranged: bool, rng: &mut fastrand::Rng) -> NormalAttackRoll {
        if roll_percent(target.perfect_dodge(), rng) {
            return NormalAttackRoll::LuckyDodge;
        }
        let double_attack = roll_percent(Self::double_attack_chance(source) as f32, rng)
            || *source.right_hand_weapon_type() == WeaponType::Revolver
                && roll_percent((5 * known_skill_level(source, SkillEnum::GsChainaction)) as f32, rng);
        let race_crit = sum_bonus(source, |bonus| match bonus {
            BonusType::CriticalAgainstRacePercentage(race, percent) if *race == MobRace::All || race == target.race() => {
                Some(*percent as i32)
            }
            _ => None,
        });
        let ranged_crit = if ranged {
            sum_bonus(source, |bonus| match bonus {
                BonusType::LongRangeCriticalChance(percent) => Some(*percent as i32),
                _ => None,
            })
        } else {
            0
        };
        let critical_chance = (source.crit() + race_crit as f32 + ranged_crit as f32)
            * if *source.right_hand_weapon_type() == WeaponType::Katar {
                2.0
            } else {
                1.0
            }
            - target.luk() as f32 / 5.0;
        if !double_attack && roll_percent(critical_chance, rng) {
            return NormalAttackRoll::Critical;
        }
        let perfect_hit = sum_bonus(source, |bonus| match bonus {
            BonusType::PerfectHitPercentage(chance) => Some(*chance as i32),
            _ => None,
        });
        let mut hit_rate = 80 + i32::from(source.hit()) - i32::from(target.flee())
            + if double_attack {
                known_skill_level(source, SkillEnum::TfDouble)
            } else {
                0
            };
        hit_rate += hit_rate * 2 * known_skill_level(source, SkillEnum::BsWeaponresearch) / 100;
        let immobilized = target.active_statuses().iter().any(|status| {
            matches!(
                status.kind,
                models::status_change::StatusChangeKind::Stone
                    | models::status_change::StatusChangeKind::Freeze
                    | models::status_change::StatusChangeKind::Stun
                    | models::status_change::StatusChangeKind::Sleep
            )
        });
        if !immobilized && !roll_percent(perfect_hit as f32, rng) && !roll_percent(hit_rate.clamp(5, 95) as f32, rng) {
            return NormalAttackRoll::Miss;
        }
        if double_attack {
            NormalAttackRoll::DoubleAttack
        } else {
            NormalAttackRoll::Hit
        }
    }

    pub fn double_attack_chance(source: &StatusSnapshot) -> i32 {
        if *source.right_hand_weapon_type() == WeaponType::Fist {
            return 0;
        }
        let equipment = source
            .bonuses()
            .iter()
            .filter_map(|bonus| match bonus.bonus() {
                BonusType::DoubleAttackChancePercentage(value) => Some(i32::from(*value)),
                _ => None,
            })
            .max()
            .unwrap_or(0)
            + sum_bonus(source, |bonus| match bonus {
                BonusType::DoubleAttackAdditionalChancePercentage(value) => Some(i32::from(*value)),
                _ => None,
            });
        let skill = if *source.right_hand_weapon_type() == WeaponType::Dagger
            || source.temporary_skill_ids().contains(&SkillEnum::TfDouble.id())
            || source
                .bonuses()
                .iter()
                .any(|bonus| matches!(bonus.bonus(), BonusType::EnableSkillId(id, _) if *id == SkillEnum::TfDouble.id()))
        {
            5 * known_skill_level(source, SkillEnum::TfDouble)
        } else {
            0
        };
        equipment.max(skill).clamp(0, 100)
    }

    pub fn actor_physical_damage(
        &self,
        raw: u32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        player_target: bool,
        critical: bool,
        element: &Element,
        flags: u32,
    ) -> u32 {
        self.actor_physical_damage_signed(raw, source, target, player_target, critical, element, flags)
            .max(0) as u32
    }

    pub fn actor_physical_damage_signed(
        &self,
        raw: u32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        player_target: bool,
        critical: bool,
        element: &Element,
        flags: u32,
    ) -> i32 {
        self.actor_physical_damage_with_rules(
            raw,
            source,
            target,
            player_target,
            critical,
            element,
            flags,
            SkillDamageRules::default(),
            0,
        )
    }

    fn actor_physical_damage_with_rules(
        &self,
        raw: u32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        player_target: bool,
        critical: bool,
        element: &Element,
        flags: u32,
        rules: SkillDamageRules,
        skill_id: u32,
    ) -> i32 {
        if self.is_infinite_defense(target, flags) {
            return 1;
        }
        let class = self.target_class(target);
        let ignores_def = critical
            || rules.ignore_defense
            || source.bonuses().iter().any(|bonus| match bonus.bonus() {
                BonusType::IgnoreDefClass(filter) => *filter == MobClass::All || *filter == class,
                BonusType::IgnoreDefRace(filter) => *filter == MobRace::All || filter == target.race(),
                _ => false,
            });
        let ignored_percentage = sum_bonus(source, |bonus| match bonus {
            BonusType::IgnoreDefRacePercentage(filter, percent) if *filter == MobRace::All || filter == target.race() => {
                Some(*percent as i32)
            }
            _ => None,
        })
        .clamp(0, 100);
        let soft_def = if ignores_def {
            0.0
        } else {
            self.sample_soft_defense(target, player_target, ignored_percentage, None)
        };
        let hard_def = if ignores_def {
            0.0
        } else {
            Self::reduced_hard_defense(target, ignored_percentage)
        };
        let damage = ((raw as f32 * (1.0 - hard_def)).floor() - soft_def).max(1.0)
            * if rules.ignore_element {
                1.0
            } else {
                Self::element_modifier(element, target)
            };
        let damage = self.apply_skill_damage_modifiers(damage, source, target, skill_id);
        let damage = if rules.ignore_attack_cards {
            damage
        } else {
            let damage = if rules.ignore_element {
                damage
            } else {
                self.apply_element_attack_bonus(damage, source, element, false)
            };
            self.apply_damage_bonus_modifier_with_rules(damage, source, target, rules)
        };
        self.apply_damage_reduction_with_rules(damage, source, target, element, flags, rules)
            .clamp(i32::MIN as f32, i32::MAX as f32)
            .floor() as i32
    }

    pub fn actor_physical_skill_damage(
        &self,
        raw_atk: u32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        player_target: bool,
        ratio: f32,
        hits: i16,
        element: &Element,
        flags: u32,
        skill_id: u32,
    ) -> u32 {
        self.actor_physical_skill_damage_signed(raw_atk, source, target, player_target, ratio, hits, element, flags, skill_id)
            .max(0) as u32
    }

    pub fn actor_physical_skill_damage_signed(
        &self,
        raw_atk: u32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        player_target: bool,
        ratio: f32,
        hits: i16,
        element: &Element,
        flags: u32,
        skill_id: u32,
    ) -> i32 {
        let hits = if hits == 0 { 1 } else { hits };
        let count = u32::from(hits.unsigned_abs());
        let ratio = Self::weapon_skill_ratio(source, ratio, skill_id);
        let ratio = if hits > 1 { ratio / count as f32 } else { ratio };
        let raw = (raw_atk as f64 * f64::from(ratio)).clamp(0.0, u32::MAX as f64).floor() as u32;
        let damage = self.actor_physical_damage_with_rules(
            raw,
            source,
            target,
            player_target,
            false,
            element,
            flags,
            SkillDamageRules::from_skill(skill_id),
            skill_id,
        );
        let damage = if hits > 1 {
            damage.saturating_mul(count as i32)
        } else {
            damage / count as i32 * count as i32
        };
        damage
    }

    pub fn actor_misc_skill_damage(
        &self,
        raw: u32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        element: &Element,
        flags: u32,
        skill_id: u32,
    ) -> u32 {
        let rules = SkillDamageRules::from_skill(skill_id);
        let damage = self.apply_damage_reduction_with_options(raw as f32, source, target, element, flags, rules, false);
        let damage = self.apply_skill_damage_modifiers(damage, source, target, skill_id).floor()
            * if rules.ignore_element {
                1.0
            } else {
                Self::element_modifier(element, target)
            };
        if damage > 0.0 && self.is_infinite_defense(target, flags) {
            1
        } else {
            damage.clamp(0.0, u32::MAX as f32).floor() as u32
        }
    }

    pub fn player_physical_skill_damage(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        ratio: f32,
        hits: i16,
        ranged: bool,
        element: &Element,
        skill_id: u32,
    ) -> u32 {
        self.player_physical_skill_damage_signed(source, target, ratio, hits, ranged, element, skill_id)
            .max(0) as u32
    }

    pub fn player_physical_skill_damage_signed(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        ratio: f32,
        hits: i16,
        ranged: bool,
        element: &Element,
        skill_id: u32,
    ) -> i32 {
        self.player_physical_splash_damage_signed(source, target, ratio, hits, ranged, element, skill_id, 1)
    }

    pub fn player_physical_splash_damage(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        ratio: f32,
        hits: i16,
        ranged: bool,
        element: &Element,
        skill_id: u32,
        targets: u32,
    ) -> u32 {
        self.player_physical_splash_damage_signed(source, target, ratio, hits, ranged, element, skill_id, targets)
            .max(0) as u32
    }

    pub fn player_physical_splash_damage_signed(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        ratio: f32,
        hits: i16,
        ranged: bool,
        element: &Element,
        skill_id: u32,
        targets: u32,
    ) -> i32 {
        let source = Self::right_hand_source(source);
        let source = source.as_ref();
        let hits = if hits == 0 { 1 } else { hits };
        let count = u32::from(hits.unsigned_abs());
        let ratio = Self::player_weapon_skill_ratio(source, ratio, skill_id);
        let ratio = if hits > 1 { ratio / count as f32 } else { ratio };
        let per_hit = self.physical_damage_with_overrides(
            source,
            target,
            ratio,
            ranged,
            element,
            false,
            skill_id,
            targets,
            PhysicalAttackOverrides {
                sphere_hits: count.min(u32::from(u16::MAX)) as u16,
                ..Default::default()
            },
        );
        let flags = BattleFlag::Weapon.as_flag()
            | if ranged {
                BattleFlag::Long.as_flag()
            } else {
                BattleFlag::Short.as_flag()
            };
        let damage = if self.is_infinite_defense(target, flags) && per_hit > 0 {
            count as i32
        } else if hits > 1 {
            per_hit.saturating_mul(count as i32)
        } else {
            per_hit / count as i32 * count as i32
        };
        damage
    }

    pub fn normal_splash_damage(&self, source: &StatusSnapshot, target: &StatusSnapshot, ranged: bool) -> u32 {
        self.normal_splash_damage_signed(source, target, ranged).max(0) as u32
    }

    pub fn normal_splash_damage_signed(&self, source: &StatusSnapshot, target: &StatusSnapshot, ranged: bool) -> i32 {
        let mut source = source.clone();
        source.set_bonuses(
            source
                .bonuses()
                .iter()
                .filter(|bonus| {
                    !matches!(
                        bonus.bonus(),
                        BonusType::PhysicalDamageAgainstRacePercentage(..)
                            | BonusType::PhysicalDamageAgainstSizePercentage(..)
                            | BonusType::PhysicalDamageAgainstElementPercentage(..)
                            | BonusType::PhysicalDamageAgainstClassPercentage(..)
                            | BonusType::PhysicalDamageAgainstMobIdPercentage(..)
                            | BonusType::DamageAgainstMobGroupPercentage(..)
                            | BonusType::DamageRangedAtkPercentage(..)
                            | BonusType::NormalAttackPercentage(..)
                            | BonusType::PhysicalDamageAgainstElementWithFlags(..)
                    )
                })
                .cloned()
                .collect(),
        );
        self.physical_damage(&source, target, 1.0, ranged, &self.attack_element(&source, None), false, 0)
    }

    fn right_hand_source(source: &StatusSnapshot) -> Cow<'_, StatusSnapshot> {
        if !source
            .left_hand_bonuses()
            .iter()
            .any(|bonus| matches!(bonus, BonusType::PhysicalDamageAgainstMobIdPercentage(..)))
        {
            return Cow::Borrowed(source);
        }
        let mut right = source.clone();
        right.set_bonuses(
            source
                .bonuses()
                .iter()
                .map(|bonus| match bonus.bonus() {
                    BonusType::PhysicalDamageAgainstMobIdPercentage(id, percent) => {
                        let left = source
                            .left_hand_bonuses()
                            .iter()
                            .filter_map(|bonus| match bonus {
                                BonusType::PhysicalDamageAgainstMobIdPercentage(left_id, value) if left_id == id => Some(i32::from(*value)),
                                _ => None,
                            })
                            .sum::<i32>();
                        StatusBonus::new(BonusType::PhysicalDamageAgainstMobIdPercentage(
                            *id,
                            (i32::from(*percent) - left).clamp(i8::MIN as i32, i8::MAX as i32) as i8,
                        ))
                    }
                    _ => *bonus,
                })
                .collect(),
        );
        Cow::Owned(right)
    }

    fn left_hand_source(source: &StatusSnapshot) -> Option<StatusSnapshot> {
        let weapon = source.left_hand_weapon().as_ref().copied()?;
        let mut left = source.clone();
        left.set_right_hand_weapon(Some(weapon));
        left.set_right_hand_weapon_type(*weapon.weapon_type());
        left.set_left_hand_weapon(None);
        left.set_left_hand_bonuses(Vec::new());
        let mut bonuses = source
            .bonuses()
            .iter()
            .filter(|bonus| {
                !matches!(
                    bonus.bonus(),
                    BonusType::PhysicalDamageAgainstRacePercentage(..)
                        | BonusType::PhysicalDamageAgainstSizePercentage(..)
                        | BonusType::PhysicalDamageAgainstElementPercentage(..)
                        | BonusType::PhysicalDamageAgainstClassPercentage(..)
                        | BonusType::PhysicalDamageAgainstMobIdPercentage(..)
                        | BonusType::DamageAgainstMobGroupPercentage(..)
                        | BonusType::PhysicalDamageAgainstElementWithFlags(..)
                        | BonusType::IgnoreDefClass(..)
                        | BonusType::IgnoreDefRace(..)
                        | BonusType::IncreaseDamageAgainstClassBaseOnDef(..)
                        | BonusType::ElementWeapon(..)
                        | BonusType::WeaponAtk(..)
                        | BonusType::WeaponRefineAtk(..)
                )
            })
            .copied()
            .collect::<Vec<_>>();
        bonuses.extend(
            source
                .left_hand_bonuses()
                .iter()
                .filter(|bonus| {
                    matches!(
                        bonus,
                        BonusType::PhysicalDamageAgainstMobIdPercentage(..)
                            | BonusType::ElementWeapon(..)
                            | BonusType::WeaponAtk(..)
                            | BonusType::WeaponRefineAtk(..)
                    ) || source.right_hand_weapon().is_none()
                        && matches!(
                            bonus,
                            BonusType::IgnoreDefClass(..)
                                | BonusType::IgnoreDefRace(..)
                                | BonusType::IncreaseDamageAgainstClassBaseOnDef(..)
                        )
                })
                .copied()
                .map(StatusBonus::new),
        );
        bonuses.extend(
            source
                .active_statuses()
                .iter()
                .flat_map(|change| change.bonuses())
                .filter(|bonus| matches!(bonus, BonusType::ElementWeapon(..)))
                .map(StatusBonus::new),
        );
        left.set_bonuses(bonuses);
        Some(left)
    }

    pub fn normal_weapon_damage_parts(&self, source: &StatusSnapshot, target: &StatusSnapshot, outcome: NormalAttackRoll) -> (u32, u32) {
        let (right, left) = self.normal_weapon_damage_signed_parts(source, target, outcome);
        (right.max(0) as u32, left.max(0) as u32)
    }

    pub fn normal_weapon_damage_signed_parts(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        outcome: NormalAttackRoll,
    ) -> (i32, i32) {
        if outcome == NormalAttackRoll::LuckyDodge {
            return (0, 0);
        }
        let ranged = source.right_hand_weapon_type().is_ranged();
        let flags = BattleFlag::Weapon.as_flag()
            | BattleFlag::Normal.as_flag()
            | if ranged {
                BattleFlag::Long.as_flag()
            } else {
                BattleFlag::Short.as_flag()
            };
        let soft_defense_roll = fastrand::u64(..);
        let hit = outcome != NormalAttackRoll::Miss;
        let critical = outcome == NormalAttackRoll::Critical;
        let base_attack = i64::from(self.status_service.fist_atk(source, source.right_hand_weapon_type().is_ranged()))
            + i64::from(source.base_atk())
            + i64::from(source.bonus_atk());
        let damage = |hand: &StatusSnapshot, secondary_hand| {
            if hit {
                self.physical_damage_with_overrides(
                    hand,
                    target,
                    Self::player_weapon_skill_ratio(hand, 1.0, 0),
                    ranged,
                    &self.attack_element(hand, None),
                    critical,
                    0,
                    1,
                    PhysicalAttackOverrides {
                        soft_defense_roll: Some(soft_defense_roll),
                        base_attack: Some(base_attack.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32),
                        raw_attack: None,
                        secondary_hand,
                        sphere_hits: 0,
                        skill_level: 0,
                    },
                )
            } else {
                self.normal_miss_damage(hand, target, &self.attack_element(hand, None), flags)
                    .min(i32::MAX as u32) as i32
            }
        };
        let right_source = Self::right_hand_source(source);
        let mut right = damage(right_source.as_ref(), false).saturating_mul(if outcome == NormalAttackRoll::DoubleAttack { 2 } else { 1 });
        if *source.right_hand_weapon_type() == WeaponType::Katar {
            let level = source
                .known_skills()
                .iter()
                .find(|skill| skill.value == SkillEnum::TfDouble)
                .map_or(0, |skill| i32::from(skill.level));
            let left = if self.is_infinite_defense(target, flags) {
                0
            } else {
                right.saturating_mul(1 + 2 * level) / 100
            };
            return (right, left);
        }
        let Some(left_source) = Self::left_hand_source(source) else {
            return (right, 0);
        };
        let mut left = damage(&left_source, true);
        let thief_lineage = JobName::try_from_value(source.job() as usize)
            .is_ok_and(|job| matches!(job.job_family(), JobFamily::Thief | JobFamily::Assassin));
        let level = |id| {
            source
                .known_skills()
                .iter()
                .find(|skill| skill.value == id)
                .map_or(0, |skill| i32::from(skill.level))
        };
        if right != 0 {
            if thief_lineage {
                right = right.saturating_mul(50 + 10 * level(SkillEnum::AsRight)) / 100;
            }
            right = right.max(1);
        }
        if left != 0 {
            if thief_lineage {
                left = left.saturating_mul(30 + 10 * level(SkillEnum::AsLeft)) / 100;
            }
            left = left.max(1);
        }
        if source.right_hand_weapon().is_none() {
            right = 0;
        }
        if self.is_infinite_defense(target, flags) && right > 0 && left > 0 {
            right = 1;
            left = 1;
        }
        (right, left)
    }

    pub(crate) fn new(
        client_notification_sender: SyncSender<Notification>,
        status_service: &'static StatusService,
        configuration_service: &'static GlobalConfigService,
        battle_result_mode: BattleResultMode,
    ) -> Self {
        BattleService {
            client_notification_sender,
            status_service,
            configuration_service,
            battle_result_mode,
        }
    }

    pub fn init(
        client_notification_sender: SyncSender<Notification>,
        status_service: &'static StatusService,
        configuration_service: &'static GlobalConfigService,
    ) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(BattleService::new(
                client_notification_sender,
                status_service,
                configuration_service,
                BattleResultMode::Normal,
            ));
        });
    }

    pub fn calculate_damage(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        skill: Option<&dyn OffensiveSkill>,
    ) -> i32 {
        self.calculate_damage_with_context(source_status, target_status, skill).0
    }

    pub fn calculate_damage_with_context(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        skill: Option<&dyn OffensiveSkill>,
    ) -> (i32, Option<MagicAttackContext>) {
        let mut damage = 0;
        let mut magic_context = None;
        if let Some(skill) = skill {
            if matches!(skill.id(), id if id == SkillEnum::PaPressure.id() || id == SkillEnum::TfThrowstone.id()) {
                let raw = if skill.id() == SkillEnum::PaPressure.id() {
                    500 + 300 * u32::from(skill.level())
                } else if JobName::try_from_value(source_status.job() as usize).is_ok() {
                    50
                } else {
                    30
                };
                let flags = BattleFlag::Misc.as_flag()
                    | BattleFlag::Long.as_flag()
                    | BattleFlag::Skill.as_flag()
                    | if skill.id() == SkillEnum::TfThrowstone.id() {
                        BattleFlag::Weapon.as_flag()
                    } else {
                        0
                    };
                return (
                    self.actor_misc_skill_damage(raw, source_status, target_status, &Element::Neutral, flags, skill.id())
                        .min(i32::MAX as u32) as i32,
                    None,
                );
            } else if matches!(skill.id(),id if id==SkillEnum::CrGrandcross.id() || id==SkillEnum::NpcGranddarkness.id()) {
                let (value, context) =
                    self.grand_cross_skill_damage_with_context(source_status, target_status, skill.level(), false, skill.id());
                damage = value;
                magic_context = Some(context);
            } else if Self::is_weapon_skill(skill) {
                let source_status = Self::right_hand_source(source_status);
                let source_status = source_status.as_ref();
                let ratio = if skill.id() == SkillEnum::RgBackstap.id() {
                    1.0 + (2.0 + 0.4 * f32::from(skill.level()))
                        * if *source_status.right_hand_weapon_type() == WeaponType::Bow {
                            0.5
                        } else {
                            1.0
                        }
                } else if skill.id() == SkillEnum::RgIntimidate.id() {
                    1.0 + 0.3 * f32::from(skill.level())
                } else if skill.id() == SkillEnum::NjIssen.id() {
                    1.0
                } else if skill.id() == SkillEnum::TfPoison.id() {
                    1.0
                } else {
                    skill.dmg_atk().unwrap_or(1.0)
                };
                let mut skill_modifier = Self::player_weapon_skill_ratio(source_status, ratio, skill.id());
                if skill.hit_count() > 1 {
                    skill_modifier /= skill.hit_count() as f32;
                }
                damage = self.physical_damage_with_overrides(
                    source_status,
                    target_status,
                    skill_modifier,
                    skill.is_ranged(),
                    &self.attack_element(source_status, Some(skill)),
                    false,
                    skill.id(),
                    1,
                    PhysicalAttackOverrides {
                        raw_attack: (skill.id() == SkillEnum::NjIssen.id()).then(|| {
                            (u64::from(source_status.str()) * 40 + u64::from(source_status.hp()) * 8 * u64::from(skill.level()) / 100)
                                .min(u64::from(u32::MAX)) as u32
                        }),
                        sphere_hits: u16::from(skill.hit_count().unsigned_abs()),
                        skill_level: skill.level(),
                        ..Default::default()
                    },
                );
                let flags = BattleFlag::Weapon.as_flag()
                    | if skill.is_ranged() {
                        BattleFlag::Long.as_flag()
                    } else {
                        BattleFlag::Short.as_flag()
                    };
                if self.is_infinite_defense(target_status, flags) && damage > 0 {
                    damage = i32::from(skill.hit_count().unsigned_abs().max(1));
                } else if skill.hit_count() > 1 {
                    damage *= skill.hit_count() as i32;
                } else {
                    damage = ((damage as f32 / skill.hit_count().abs() as f32).floor() * skill.hit_count().abs() as f32) as i32;
                }
            } else if skill.is_magic() {
                let mut skill_modifier = skill.dmg_matk().unwrap_or(1.0);
                if skill.hit_count() > 1 {
                    skill_modifier /= skill.hit_count() as f32;
                }
                let mut context = self.magic_attack_context(source_status, skill_modifier, self.attack_element(source_status, Some(skill)));
                context.hits = u16::from(skill.hit_count().unsigned_abs().max(1));
                context.skill_id = skill.id();
                damage = self.magic_damage_from_context(source_status, target_status, context);
                magic_context = Some(context);
            }
        } else {
            let (right, left) = self.normal_weapon_damage_signed_parts(source_status, target_status, NormalAttackRoll::Hit);
            damage = right.saturating_add(left);
        }

        (damage, magic_context)
    }

    pub fn is_weapon_skill(skill: &dyn OffensiveSkill) -> bool {
        skill.id() == SkillEnum::NjIssen.id() || skill.is_physical()
    }

    /// (([((({(base_atk +
    /// + rnd(min(DEX,ATK), ATK)*SizeModifier) * SkillModifiers * (1 - DEF/100)
    ///   - VitDEF + BaneSkill + UpgradeDamage}
    /// + MasterySkill + WeaponryResearchSkill + EnvenomSkill) *
    ///   ElementalModifier) + Enhancements) * DamageBonusModifiers *
    ///   DamageReductionModifiers] * NumberOfMultiHits) - KyrieEleisonEffect) /
    ///   NumberOfMultiHits
    fn physical_damage_character_attack_monster(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        skill_modifier: f32,
        is_ranged: bool,
        element: &Element,
        skill_id: u32,
    ) -> i32 {
        self.physical_damage(
            source_status,
            target_status,
            skill_modifier,
            is_ranged,
            element,
            false,
            skill_id,
        )
    }

    fn physical_damage(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        skill_modifier: f32,
        is_ranged: bool,
        element: &Element,
        critical: bool,
        skill_id: u32,
    ) -> i32 {
        let skill_modifier = if skill_id == 0 {
            Self::player_weapon_skill_ratio(source_status, skill_modifier, skill_id)
        } else {
            skill_modifier
        };
        self.physical_damage_with_split(
            source_status,
            target_status,
            skill_modifier,
            is_ranged,
            element,
            critical,
            skill_id,
            1,
        )
    }

    fn player_weapon_skill_ratio(source: &StatusSnapshot, ratio: f32, skill_id: u32) -> f32 {
        Self::weapon_skill_ratio(source, ratio, skill_id)
    }

    pub fn weapon_skill_ratio(source: &StatusSnapshot, ratio: f32, skill_id: u32) -> f32 {
        if skill_id == SkillEnum::PaSacrifice.id() {
            return ratio;
        }
        ratio
            + if source.has_status_change(models::status_change::StatusChangeKind::Berserk) {
                1.0
            } else {
                0.0
            }
            + source
                .status_change(models::status_change::StatusChangeKind::Overthrust)
                .map_or(0.0, |change| change.values[2] as f32 / 100.0)
    }

    pub fn attack_uses_ammo(source: &StatusSnapshot, skill_id: u32) -> bool {
        if skill_id == 0 {
            return source.right_hand_weapon_type().is_ranged();
        }
        crate::server::script::skill::metadata::SkillMetadata::find(skill_id)
            .and_then(|metadata| metadata.requires.as_ref()?.get("Ammo")?.as_object())
            .is_some_and(|ammo| ammo.values().any(|enabled| enabled.as_bool() == Some(true)))
    }

    fn skill_stacks_mastery(skill_id: u32) -> bool {
        ![
            SkillEnum::PaShieldchain,
            SkillEnum::CrShieldboomerang,
            SkillEnum::AmAcidterror,
            SkillEnum::MoInvestigate,
            SkillEnum::MoExtremityfist,
            SkillEnum::PaSacrifice,
            SkillEnum::LkSpiralpierce,
        ]
        .into_iter()
        .any(|skill| skill.id() == skill_id)
    }

    fn skill_stacks_refine(skill_id: u32) -> bool {
        skill_id == SkillEnum::LkSpiralpierce.id() || Self::skill_stacks_mastery(skill_id)
    }

    fn physical_damage_with_split(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        skill_modifier: f32,
        is_ranged: bool,
        element: &Element,
        critical: bool,
        skill_id: u32,
        targets: u32,
    ) -> i32 {
        self.physical_damage_with_overrides(
            source_status,
            target_status,
            skill_modifier,
            is_ranged,
            element,
            critical,
            skill_id,
            targets,
            PhysicalAttackOverrides::default(),
        )
    }

    fn physical_damage_with_overrides(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        skill_modifier: f32,
        is_ranged: bool,
        element: &Element,
        critical: bool,
        skill_id: u32,
        targets: u32,
        overrides: PhysicalAttackOverrides,
    ) -> i32 {
        let flags = BattleFlag::Weapon.as_flag()
            | if is_ranged {
                BattleFlag::Long.as_flag()
            } else {
                BattleFlag::Short.as_flag()
            };
        if self.is_infinite_defense(target_status, flags) {
            return 1;
        }
        let skill_attack = skill_id != 0;
        let rules = SkillDamageRules::from_skill(skill_id);
        let elemental_extra = source_status
            .status_change(models::status_change::StatusChangeKind::WeaponAttackElement)
            .filter(|_| !overrides.secondary_hand && !rules.ignore_attack_cards && !rules.ignore_element);
        let base_atk = overrides.base_attack.map_or_else(
            || {
                f32::from(
                    self.status_service
                        .fist_atk(source_status, source_status.right_hand_weapon_type().is_ranged()),
                ) + source_status.base_atk() as f32
                    + f32::from(source_status.bonus_atk())
            },
            |value| value as f32,
        );

        let class = self.target_class(target_status);
        let ignores_def = critical
            || rules.ignore_defense
            || source_status.bonuses().iter().any(|bonus| match bonus.bonus() {
                BonusType::IgnoreDefClass(filter) => *filter == MobClass::All || *filter == class,
                BonusType::IgnoreDefRace(filter) => *filter == MobRace::All || filter == target_status.race(),
                _ => false,
            });
        let ignore_percentage = source_status
            .bonuses()
            .iter()
            .filter_map(|bonus| match bonus.bonus() {
                BonusType::IgnoreDefRacePercentage(filter, percent) if *filter == MobRace::All || filter == target_status.race() => {
                    Some(*percent as i32)
                }
                _ => None,
            })
            .sum::<i32>()
            .clamp(0, 100);
        let def = if ignores_def {
            0.0
        } else {
            Self::reduced_hard_defense(target_status, ignore_percentage)
        };

        let full_vitdef = if !ignores_def || elemental_extra.is_some() {
            self.sample_soft_defense(
                target_status,
                JobName::try_from_value(target_status.job() as usize).is_ok(),
                ignore_percentage,
                overrides.soft_defense_roll,
            )
        } else {
            0.0
        };
        let vitdef: f32 = if ignores_def { 0.0 } else { full_vitdef };
        let bane_skill = sum_bonus(source_status, |bonus| match bonus {
            BonusType::AtkBaneAgainstRace(race, amount) if *race == MobRace::All || race == target_status.race() => {
                Some(i32::from(*amount))
            }
            _ => None,
        }) as f32;
        let mastery_skill = sum_bonus(source_status, |bonus| match bonus {
            BonusType::MasteryDamageUsingWeaponType(weapon, amount) if weapon == source_status.right_hand_weapon_type() => {
                Some(i32::from(*amount))
            }
            _ => None,
        }) as f32;
        let elemental_modifier: f32 = if rules.ignore_element {
            1.0
        } else {
            Self::element_modifier(element, target_status)
        };
        let number_of_hits: f32 = 1.0;
        let _kyrie_eleison_effect: f32 = 0.0;

        let uses_ammo = Self::attack_uses_ammo(source_status, skill_id) || skill_id == SkillEnum::HtPhantasmic.id();
        let weapon_atk = if overrides.raw_attack.is_some() {
            0
        } else {
            self.weapon_atk_with_context(source_status, target_status, critical, uses_ammo)
        };
        let attack_rate = if Self::skill_stacks_mastery(skill_id) {
            sum_bonus(source_status, |bonus| match bonus {
                BonusType::AtkPercentage(percent) => Some(i32::from(*percent)),
                _ => None,
            }) + source_status
                .status_change(models::status_change::StatusChangeKind::Provoke)
                .map_or(0, |change| change.values[1])
        } else {
            0
        };
        let raw_attack = overrides.raw_attack.map_or_else(
            || self.apply_weapon_damage_rate(base_atk + weapon_atk as f32, source_status),
            |value| value as f32,
        );
        let raw_attack = (raw_attack / targets.max(1) as f32).floor();
        let raw_attack = if critical {
            scale_damage(
                raw_attack,
                sum_bonus(source_status, |bonus| match bonus {
                    BonusType::CriticalDamagePercentage(percent) => Some(i32::from(*percent)),
                    _ => None,
                })
                .max(0),
            )
            .floor()
        } else {
            raw_attack
        };
        let total_attack = scale_damage(raw_attack, attack_rate).floor();
        let refine = if Self::skill_stacks_refine(skill_id) {
            f32::from(source_status.weapon_upgrade_damage())
        } else {
            0.0
        };
        let (bane_skill, mastery_skill) = if Self::skill_stacks_mastery(skill_id) {
            (
                bane_skill,
                mastery_skill + self.learned_weapon_mastery_damage(source_status, target_status) as f32,
            )
        } else {
            (0.0, 0.0)
        };
        let def_ratio = !ignores_def
            && ![
                SkillEnum::PaSacrifice,
                SkillEnum::CrGrandcross,
                SkillEnum::NpcGranddarkness,
                SkillEnum::PaShieldchain,
            ]
            .into_iter()
            .any(|skill| skill.id() == skill_id)
            && source_status.bonuses().iter().any(|bonus| match bonus.bonus() {
                BonusType::IncreaseDamageAgainstClassBaseOnDef(filter) => *filter == MobClass::All || *filter == class,
                _ => false,
            });
        let mut atk = if def_ratio {
            ((total_attack * skill_modifier) * (def.min(1.0) * 100.0 + vitdef) / 100.0).floor() + refine
        } else {
            let atk = total_attack * skill_modifier * (1.0 - def);
            (atk - vitdef + bane_skill + refine).max(1.0).floor()
        };
        atk = atk.max(1.0) + mastery_skill;
        atk = (atk * elemental_modifier).floor();
        if !rules.ignore_attack_cards && !rules.ignore_element {
            atk = self.apply_element_attack_bonus(atk, source_status, element, false);
        }
        atk += f32::from(Self::forged_star_damage(source_status, skill_id))
            + f32::from(Self::spirit_sphere_damage(source_status, skill_id, overrides.sphere_hits));
        if skill_id == SkillEnum::TfPoison.id() {
            atk = ((atk + f32::from(overrides.skill_level) * 15.0) * elemental_modifier).floor();
        }
        if let Some(change) = elemental_extra {
            if let Ok(extra_element) = Element::try_from_value(change.values[0].max(0) as usize) {
                let normal_weapon = if skill_attack {
                    self.weapon_atk_with_critical(source_status, target_status, critical)
                } else {
                    weapon_atk
                };
                let normal_attack = scale_damage(
                    self.apply_weapon_damage_rate(base_atk + normal_weapon as f32, source_status),
                    attack_rate,
                );
                let normal_hard_defense = Self::reduced_hard_defense(target_status, ignore_percentage);
                let normal_damage = ((normal_attack * (1.0 - normal_hard_defense)).floor() - full_vitdef
                    + bane_skill
                    + source_status.weapon_upgrade_damage() as f32)
                    .max(1.0)
                    .floor()
                    + mastery_skill;
                let elemental_damage = self.apply_element_attack_bonus(
                    (normal_damage * Self::element_modifier(&extra_element, target_status)).floor(),
                    source_status,
                    &extra_element,
                    false,
                ) + f32::from(Self::forged_star_damage(source_status, 0))
                    + f32::from(Self::spirit_sphere_damage(source_status, skill_id, overrides.sphere_hits));
                atk += (elemental_damage * change.values[1].max(0) as f32 / 100.0).floor();
            }
        }
        if !skill_attack {
            atk = scale_damage(
                atk,
                sum_bonus(source_status, |bonus| match bonus {
                    BonusType::NormalAttackPercentage(percent) => Some(i32::from(*percent)),
                    _ => None,
                }),
            );
        }
        atk += 2.0 * known_skill_level(source_status, SkillEnum::BsWeaponresearch) as f32
            + if skill_id != SkillEnum::McCartrevolution.id() && known_skill_level(source_status, SkillEnum::BsHiltbinding) > 0 {
                4.0
            } else {
                0.0
            };
        atk = self.apply_skill_damage_modifiers(atk, source_status, target_status, skill_id);
        if !rules.ignore_attack_cards {
            atk = self.apply_damage_bonus_modifier_with_rules(atk, source_status, target_status, rules);
        }
        if is_ranged && !rules.ignore_attack_cards {
            atk = scale_damage(
                atk,
                sum_bonus(source_status, |bonus| match bonus {
                    BonusType::DamageRangedAtkPercentage(percent) => Some(*percent as i32),
                    _ => None,
                }),
            );
        }
        let flags = BattleFlag::Weapon.as_flag()
            | if is_ranged {
                BattleFlag::Long.as_flag()
            } else {
                BattleFlag::Short.as_flag()
            }
            | if skill_attack {
                BattleFlag::Skill.as_flag()
            } else {
                BattleFlag::Normal.as_flag()
            };
        if !rules.ignore_attack_cards && !rules.ignore_element {
            atk = scale_damage(
                atk,
                sum_bonus(source_status, |bonus| match bonus {
                    BonusType::PhysicalDamageAgainstElementWithFlags((filter, required), percent)
                        if (*filter == Element::AllElement || filter == target_status.element())
                            && BattleFlag::matches(*required, flags) =>
                    {
                        Some(*percent as i32)
                    }
                    _ => None,
                }),
            );
        }
        atk = self.apply_damage_reduction_with_rules(atk, source_status, target_status, element, flags, rules);
        atk *= number_of_hits;
        atk.floor() as i32
    }

    fn learned_weapon_mastery_damage(&self, source: &StatusSnapshot, target: &StatusSnapshot) -> i32 {
        let level = |skill| known_skill_level(source, skill);
        let weapon = match source.right_hand_weapon_type() {
            WeaponType::Dagger | WeaponType::Sword1H => 4 * level(SkillEnum::SmSword),
            WeaponType::Sword2H => 4 * level(SkillEnum::SmTwohand),
            WeaponType::Spear1H | WeaponType::Spear2H => {
                level(SkillEnum::KnSpearmastery) * if source.state() & SkillState::Riding.as_flag() != 0 { 5 } else { 4 }
            }
            WeaponType::Axe1H | WeaponType::Axe2H => 3 * level(SkillEnum::AmAxemastery),
            WeaponType::Mace | WeaponType::Mace2H => 3 * level(SkillEnum::PrMacemastery),
            WeaponType::Fist => 10 * level(SkillEnum::TkRun) + 3 * level(SkillEnum::MoIronhand),
            WeaponType::Knuckle => 3 * level(SkillEnum::MoIronhand),
            WeaponType::Musical => 3 * level(SkillEnum::BaMusicallesson),
            WeaponType::Whip => 3 * level(SkillEnum::DcDancinglesson),
            WeaponType::Book => 3 * level(SkillEnum::SaAdvancedbook),
            WeaponType::Katar => 3 * level(SkillEnum::AsKatar),
            _ => 0,
        };
        let mut bane = 0;
        if self.configuration_service.get_mob_safe(target.job() as i32).is_some()
            && (matches!(target.race(), MobRace::Demon | MobRace::RUndead) || *target.element() == Element::Undead)
        {
            bane += ((u64::from(source.base_level()) + 60) * level(SkillEnum::AlDemonbane) as u64 / 20).min(i32::MAX as u64) as i32;
        }
        let beast_bane = level(SkillEnum::HtBeastbane);
        if beast_bane > 0 && matches!(target.race(), MobRace::Brute | MobRace::Insect) {
            bane += 4 * beast_bane;
            if source
                .status_change(models::status_change::StatusChangeKind::Spirit)
                .is_some_and(|change| change.values[1] == SkillEnum::SlHunter.id() as i32)
            {
                bane += i32::from(source.str());
            }
        }
        weapon.saturating_add(bane)
    }

    pub fn forged_star_damage(source: &StatusSnapshot, skill_id: u32) -> u16 {
        if [
            SkillEnum::PaShieldchain,
            SkillEnum::CrShieldboomerang,
            SkillEnum::AmAcidterror,
            SkillEnum::MoInvestigate,
            SkillEnum::MoExtremityfist,
            SkillEnum::PaSacrifice,
        ]
        .into_iter()
        .any(|skill| skill.id() == skill_id)
        {
            return 0;
        }
        source.right_hand_weapon().map_or(0, |weapon| weapon.forged_star_damage())
    }

    pub fn spirit_sphere_damage(source: &StatusSnapshot, skill_id: u32, hits: u16) -> u16 {
        if [
            SkillEnum::PaShieldchain,
            SkillEnum::CrShieldboomerang,
            SkillEnum::AmAcidterror,
            SkillEnum::MoInvestigate,
            SkillEnum::MoExtremityfist,
            SkillEnum::PaSacrifice,
            SkillEnum::LkSpiralpierce,
        ]
        .into_iter()
        .any(|skill| skill.id() == skill_id)
        {
            return 0;
        }
        let count = u16::from(source.spirit_sphere_count()) + if skill_id == SkillEnum::MoFingeroffensive.id() { hits } else { 0 };
        count.saturating_mul(3)
    }

    pub fn normal_miss_damage(&self, source: &StatusSnapshot, target: &StatusSnapshot, element: &Element, flags: u32) -> u32 {
        let stars = Self::forged_star_damage(source, 0).saturating_add(Self::spirit_sphere_damage(source, 0, 0));
        let damage = stars as f32
            + 2.0 * known_skill_level(source, SkillEnum::BsWeaponresearch) as f32
            + if known_skill_level(source, SkillEnum::BsHiltbinding) > 0 {
                4.0
            } else {
                0.0
            };
        let damage = self.apply_damage_bonus_modifier(damage, source, target);
        self.apply_damage_reduction(damage, source, target, element, flags).max(0.0).floor() as u32
    }

    pub fn mob_vitdef(&self, target_status: &StatusSnapshot) -> f32 {
        self.sample_soft_defense(target_status, false, 0, None)
    }

    ///  [VIT*0.5] + rnd([VIT*0.3], max([VIT*0.3],[VIT^2/150]-1))
    pub fn player_vitdef(&self, target_status: &StatusSnapshot) -> u16 {
        self.sample_soft_defense(target_status, true, 0, None) as u16
    }

    fn reduced_hard_defense(target: &StatusSnapshot, ignored_percentage: i32) -> f32 {
        let defense = i32::from(target.def());
        (defense - defense * ignored_percentage / 100).min(100) as f32 / 100.0
    }

    fn sample_soft_defense(&self, target: &StatusSnapshot, player_target: bool, ignored_percentage: i32, shared_roll: Option<u64>) -> f32 {
        let vitality = u32::from(if player_target {
            self.status_service.character_vit_def(target)
        } else {
            target.vit()
        });
        let vitality = vitality - vitality * ignored_percentage.clamp(0, 100) as u32 / 100;
        let (base, lower, upper) = if player_target {
            let lower = vitality * 3 / 10;
            (vitality / 2, lower, lower.max((vitality * vitality / 150).saturating_sub(1)))
        } else {
            (vitality, 0, (vitality / 20).pow(2).saturating_sub(1))
        };
        let rolled = match self.battle_result_mode {
            BattleResultMode::TestMin => upper,
            BattleResultMode::TestMax => lower,
            BattleResultMode::Normal => shared_roll.map_or_else(
                || fastrand::u32(lower..=upper),
                |roll| lower + (roll % u64::from(upper - lower + 1)) as u32,
            ),
        };
        (base + rolled).min(u32::from(u16::MAX)) as f32
    }

    //  rnd(min(DEX*(0.8+0.2*WeaponLevel),ATK), ATK)
    pub fn weapon_atk(&self, source_status: &StatusSnapshot, target_status: &StatusSnapshot, _is_ranged: bool) -> u32 {
        self.weapon_atk_with_critical(source_status, target_status, false)
    }

    fn weapon_atk_with_critical(&self, source_status: &StatusSnapshot, target_status: &StatusSnapshot, critical: bool) -> u32 {
        self.weapon_atk_with_context(
            source_status,
            target_status,
            critical,
            source_status.right_hand_weapon_type().is_ranged(),
        )
    }

    fn weapon_atk_with_context(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        critical: bool,
        uses_ammo: bool,
    ) -> u32 {
        let mut rng = fastrand::Rng::new();
        let weapon = source_status.right_hand_weapon();
        let weapon_level = weapon.map_or(0, |weapon| weapon.level());
        let weapon_attack = u32::from(source_status.weapon_atk());
        let weapon_level_rate = if weapon.is_some() { 80 + 20 * u32::from(weapon_level) } else { 100 };
        let mut minimum = (u32::from(source_status.dex()) * weapon_level_rate / 100).min(weapon_attack);
        let mut maximum = weapon_attack;
        if uses_ammo {
            minimum = minimum.saturating_mul(weapon_attack) / 100;
            maximum = maximum.max(minimum);
        }
        let size_modifier = if source_status
            .bonuses()
            .iter()
            .any(|bonus| matches!(bonus.bonus(), BonusType::EnableIgnoreSizeModifier))
        {
            1.0
        } else {
            Self::size_modifier(source_status, target_status)
        };
        let damage = if critical || source_status.has_status_change(models::status_change::StatusChangeKind::MaximizePower) {
            maximum
        } else {
            match self.battle_result_mode {
                BattleResultMode::TestMin => minimum,
                BattleResultMode::TestMax => maximum.saturating_sub(1).max(minimum),
                BattleResultMode::Normal => {
                    if maximum > minimum {
                        rng.u32(minimum..maximum)
                    } else {
                        minimum
                    }
                }
            }
        };
        let arrow = if uses_ammo {
            u32::from(source_status.ammo().map_or(0, |ammo| ammo.attack()))
        } else {
            0
        };
        let arrow = if critical {
            arrow
        } else {
            match self.battle_result_mode {
                BattleResultMode::TestMin => 0,
                BattleResultMode::TestMax => arrow.saturating_sub(1),
                BattleResultMode::Normal => {
                    if arrow > 0 {
                        rng.u32(0..arrow)
                    } else {
                        0
                    }
                }
            }
        };
        let damage = ((damage.saturating_add(arrow)) as f32 * size_modifier).floor() as u32;
        let maximum = source_status
            .right_hand_weapon()
            .map_or(0, |weapon| Self::weapon_over_refine_limit(weapon.level(), weapon.refine()));
        let extra = if maximum == 0 {
            0
        } else {
            match self.battle_result_mode {
                BattleResultMode::TestMin => 1,
                BattleResultMode::TestMax => maximum,
                BattleResultMode::Normal => rng.u32(1..=maximum),
            }
        };
        damage.saturating_add(extra)
    }

    pub fn weapon_over_refine_limit(level: u8, refine: u8) -> u32 {
        let (safe, step) = match level {
            1 => (7, 3),
            2 => (6, 5),
            3 => (5, 8),
            4 => (4, 13),
            _ => return 0,
        };
        u32::from(refine.min(10).saturating_sub(safe)) * step
    }

    /// {rnd(minMATK,maxMATK) * ItemModifier * SkillModifier * (1-MDEF/100) -
    /// INT - VIT/2} * Elemental Modifier
    pub fn magic_damage_character_attack_monster(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        skill_modifier: f32,
        element: &Element,
    ) -> i32 {
        self.magic_damage_from_context(
            source_status,
            target_status,
            self.magic_attack_context(source_status, skill_modifier, *element),
        )
    }

    fn magic_attack_context(&self, source: &StatusSnapshot, modifier: f32, element: Element) -> MagicAttackContext {
        let mut rng = fastrand::Rng::new();
        let matk = match self.battle_result_mode {
            BattleResultMode::TestMin => source.matk_min(),
            BattleResultMode::TestMax => source.matk_max(),
            BattleResultMode::Normal => rng.u16(source.matk_min()..=source.matk_max().max(source.matk_min())),
        };
        MagicAttackContext {
            matk,
            modifier,
            element,
            hits: 1,
            skill_id: 0,
            grand_cross: None,
        }
    }

    pub fn grand_cross_damage_with_context(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        level: u8,
        self_target: bool,
    ) -> (u32, MagicAttackContext) {
        let (amount, context) = self.grand_cross_damage_signed_with_context(source, target, level, self_target);
        (amount.max(0) as u32, context)
    }

    pub fn grand_cross_damage_signed_with_context(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        level: u8,
        self_target: bool,
    ) -> (i32, MagicAttackContext) {
        self.grand_cross_skill_damage_with_context(source, target, level, self_target, SkillEnum::CrGrandcross.id())
    }

    fn grand_cross_skill_damage_with_context(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        level: u8,
        self_target: bool,
        skill_id: u32,
    ) -> (i32, MagicAttackContext) {
        let raw = (i64::from(self.status_service.fist_atk(source, false))
            + i64::from(source.base_atk())
            + i64::from(source.bonus_atk())
            + i64::from(self.weapon_atk_with_critical(source, target, false)))
        .clamp(0, i64::from(u32::MAX)) as u32;
        let rate = sum_bonus(source, |bonus| match bonus {
            BonusType::AtkPercentage(rate) => Some(i32::from(*rate)),
            _ => None,
        }) + source
            .status_change(models::status_change::StatusChangeKind::Provoke)
            .map_or(0, |change| change.values[1]);
        let raw_atk = scale_damage(self.apply_weapon_damage_rate(raw as f32, source), rate).clamp(0.0, u32::MAX as f32) as u32;
        let refine_bonus = source.weapon_upgrade_damage();
        let mut context = self.magic_attack_context(
            source,
            1.0 + 0.4 * f32::from(level),
            if skill_id == SkillEnum::NpcGranddarkness.id() {
                Element::Dark
            } else {
                Element::Holy
            },
        );
        context.skill_id = skill_id;
        context.grand_cross = Some(GrandCrossAttackContext {
            raw_atk,
            refine_bonus,
            self_target,
        });
        (self.magic_damage_from_context(source, target, context), context)
    }

    fn grand_cross_damage_from_context(
        &self,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        context: MagicAttackContext,
        grand: GrandCrossAttackContext,
    ) -> i32 {
        if self.is_infinite_defense(target, BattleFlag::Magic.as_flag()) {
            return 1;
        }
        let rules = SkillDamageRules::from_skill(context.skill_id);
        let class = self.target_class(target);
        let ignore_def = rules.ignore_defense
            || source.bonuses().iter().any(|bonus| match bonus.bonus() {
                BonusType::IgnoreDefClass(value) => *value == MobClass::All || *value == class,
                BonusType::IgnoreDefRace(value) => *value == MobRace::All || value == target.race(),
                _ => false,
            });
        let ignored = sum_bonus(source, |bonus| match bonus {
            BonusType::IgnoreDefRacePercentage(race, value) if *race == MobRace::All || race == target.race() => Some(i32::from(*value)),
            _ => None,
        })
        .clamp(0, 100);
        let (hard_def, vit_def) = if ignore_def {
            (0.0, 0.0)
        } else {
            (
                Self::reduced_hard_defense(target, ignored),
                self.sample_soft_defense(target, JobName::try_from_value(target.job() as usize).is_ok(), ignored, None),
            )
        };
        let physical = (grand.raw_atk as f32 * (1.0 - hard_def)).floor() - vit_def + f32::from(grand.refine_bonus);
        let ignored = sum_bonus(source, |bonus| match bonus {
            BonusType::IgnoreMDefRacePercentage(race, value) if *race == MobRace::All || race == target.race() => Some(i32::from(*value)),
            BonusType::IgnoreMDefClassPercentage(value, rate) if *value == MobClass::All || *value == class => Some(i32::from(*rate)),
            _ => None,
        })
        .clamp(0, 100);
        let hard = i32::from(target.mdef().clamp(0, 100));
        let hard = if rules.ignore_defense { 0 } else { hard - hard * ignored / 100 };
        let soft = if rules.ignore_defense {
            0
        } else {
            i32::from(target.int())
                + if self.configuration_service.get_mob_safe(target.job() as i32).is_some() {
                    0
                } else {
                    i32::from(target.vit()) / 2
                }
        };
        let raw_magic = self.apply_skill_damage_modifiers(f32::from(context.matk), source, target, context.skill_id);
        let magic = (raw_magic * (100 - hard) as f32 / 100.0).floor() - soft as f32;
        let element = if rules.ignore_element {
            1.0
        } else {
            Self::element_modifier(&context.element, target)
        };
        let mut damage = ((physical + magic).max(1.0) * context.modifier).floor();
        damage = (damage * element).floor();
        if !rules.ignore_attack_cards {
            damage = self.apply_element_attack_bonus(damage, source, &context.element, true);
            damage = scale_damage(
                damage,
                sum_bonus(source, |bonus| match bonus {
                    BonusType::MagicalDamageAgainstRacePercentage(race, value) if *race == MobRace::All || race == target.race() => {
                        Some(i32::from(*value))
                    }
                    _ => None,
                }),
            );
            damage = scale_damage(
                damage,
                sum_bonus(source, |bonus| match bonus {
                    BonusType::MagicalDamageAgainstSizePercentage(size, value) if *size == Size::All || size == target.size() => {
                        Some(i32::from(*value))
                    }
                    _ => None,
                }),
            );
        }
        damage = self.apply_damage_reduction_with_rules(
            damage,
            source,
            target,
            &context.element,
            BattleFlag::Magic.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag(),
            rules,
        );
        damage = if grand.self_target {
            (damage / 2.0).floor()
        } else {
            (damage * element).floor()
        };
        damage.clamp(i32::MIN as f32, i32::MAX as f32).floor() as i32
    }

    pub fn magic_damage_from_context(
        &self,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        context: MagicAttackContext,
    ) -> i32 {
        if self.is_infinite_defense(target_status, BattleFlag::Magic.as_flag()) {
            return i32::from(context.hits.max(1));
        }
        if let Some(grand) = context.grand_cross {
            return self.grand_cross_damage_from_context(source_status, target_status, context, grand);
        }
        let rules = SkillDamageRules::from_skill(context.skill_id);
        let element = &context.element;
        let elemental_modifier: f32 = if rules.ignore_element {
            1.0
        } else {
            Self::element_modifier(element, target_status)
        };
        let class = self.target_class(target_status);
        let ignore = sum_bonus(source_status, |bonus| match bonus {
            BonusType::IgnoreMDefRacePercentage(race, percent) if *race == MobRace::All || race == target_status.race() => {
                Some(*percent as i32)
            }
            BonusType::IgnoreMDefClassPercentage(filter, percent) if *filter == MobClass::All || *filter == class => Some(*percent as i32),
            _ => None,
        })
        .clamp(0, 100);
        let hard_mdef = i32::from(target_status.mdef());
        let (mdef, soft_mdef) = if rules.ignore_defense {
            (0.0, 0.0)
        } else {
            let hard_mdef = (hard_mdef - hard_mdef * ignore / 100).min(100);
            let soft_mdef = u32::from(target_status.int())
                + if self.configuration_service.get_mob_safe(target_status.job() as i32).is_some() {
                    0
                } else {
                    u32::from(target_status.vit()) / 2
                };
            (hard_mdef as f32 / 100.0, soft_mdef as f32)
        };
        let raw = self.apply_skill_damage_modifiers(
            context.matk as f32 * context.modifier,
            source_status,
            target_status,
            context.skill_id,
        );
        let mut damage = ((raw * (1.0 - mdef)).floor() - soft_mdef).max(1.0) * elemental_modifier;
        if !rules.ignore_attack_cards {
            if !rules.ignore_element {
                damage = self.apply_element_attack_bonus(damage, source_status, element, true);
            }
            damage = scale_damage(
                damage,
                sum_bonus(source_status, |bonus| match bonus {
                    BonusType::MagicalDamageAgainstRacePercentage(race, percent)
                        if *race == MobRace::All || race == target_status.race() =>
                    {
                        Some(*percent as i32)
                    }
                    _ => None,
                }),
            );
            damage = scale_damage(
                damage,
                sum_bonus(source_status, |bonus| match bonus {
                    BonusType::MagicalDamageAgainstSizePercentage(size, percent) if *size == Size::All || size == target_status.size() => {
                        Some(*percent as i32)
                    }
                    _ => None,
                }),
            );
        }
        let damage = self
            .apply_damage_reduction_with_rules(
                damage,
                source_status,
                target_status,
                element,
                BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
                rules,
            )
            .floor()
            * context.hits.max(1) as f32;
        damage.floor() as i32
    }

    pub fn reflected_magic_damage(
        &self,
        kind: MagicReflectionKind,
        damage: u32,
        caster: &StatusSnapshot,
        context: Option<MagicAttackContext>,
    ) -> Result<u32, String> {
        self.reflected_magic_damage_signed(kind, damage, caster, context)
            .map(|amount| amount.max(0) as u32)
    }

    pub fn reflected_magic_damage_signed(
        &self,
        kind: MagicReflectionKind,
        damage: u32,
        caster: &StatusSnapshot,
        context: Option<MagicAttackContext>,
    ) -> Result<i32, String> {
        match kind {
            MagicReflectionKind::Mirror => Ok(damage.min(i32::MAX as u32) as i32),
            MagicReflectionKind::Equipment => {
                let context = context.ok_or("Reflected magic has no original attack calculation")?;
                Ok(self.magic_damage_from_context(caster, caster, context))
            }
        }
    }

    pub fn attack_element(&self, source_status: &StatusSnapshot, skill: Option<&dyn OffensiveSkill>) -> Element {
        self.resolve_attack_element(
            source_status,
            skill.map(|skill| skill.element()),
            skill.map_or(0, |skill| skill.id()),
            skill.is_some_and(|skill| skill.is_magic()),
            skill.is_none_or(Self::is_weapon_skill),
        )
    }

    pub fn skill_attack_element(
        &self,
        source: &StatusSnapshot,
        metadata: &crate::server::script::skill::metadata::SkillMetadata,
        level: u8,
    ) -> Element {
        self.resolve_attack_element(
            source,
            Some(
                metadata
                    .element(level)
                    .and_then(|name| Element::try_from_string_ignore_case(name).ok())
                    .unwrap_or(Element::Neutral),
            ),
            metadata.id,
            metadata.damage_type.as_deref() == Some("Magic"),
            metadata.damage_type.as_deref() == Some("Weapon"),
        )
    }

    fn resolve_attack_element(
        &self,
        source_status: &StatusSnapshot,
        skill_element: Option<Element>,
        skill_id: u32,
        magical: bool,
        weapon: bool,
    ) -> Element {
        let weapon_element = source_status
            .bonuses()
            .iter()
            .rev()
            .find_map(|bonus| match bonus.bonus() {
                BonusType::ElementWeapon(element) => Some(*element),
                _ => None,
            })
            .unwrap_or_else(|| {
                source_status
                    .right_hand_weapon()
                    .map_or(Element::Neutral, |weapon| *weapon.element())
            });
        let weapon_element = Self::status_attack_element(source_status, weapon_element);
        let uses_ammo = Self::attack_uses_ammo(source_status, skill_id);
        let arrow_element = source_status
            .ammo()
            .map(|ammo| *ammo.element())
            .filter(|element| *element != Element::Neutral);
        let weapon_or_arrow = if uses_ammo {
            arrow_element.unwrap_or(weapon_element)
        } else {
            weapon_element
        };
        let endowed = source_status
            .status_change(models::status_change::StatusChangeKind::EnchantArms)
            .and_then(|change| Element::try_from_value(change.values[0] as usize).ok())
            .unwrap_or(weapon_or_arrow);
        match skill_element {
            Some(Element::Ammo) => endowed,
            Some(Element::Weapon) if magical => weapon_element,
            Some(Element::Weapon) if !weapon => Element::Neutral,
            Some(Element::Weapon) => endowed,
            Some(Element::Endowed) if !magical && !weapon => Element::Neutral,
            Some(Element::Endowed) => Self::status_attack_element(source_status, Element::Neutral),
            Some(Element::Random) => Element::from_value(match self.battle_result_mode {
                BattleResultMode::TestMin => Element::Neutral.value(),
                BattleResultMode::TestMax => Element::Undead.value(),
                BattleResultMode::Normal => fastrand::usize(0..Element::AllElement.value()),
            }),
            Some(element) => element,
            None => endowed,
        }
    }

    fn status_attack_element(source: &StatusSnapshot, default: Element) -> Element {
        use models::status_change::StatusChangeKind;
        source
            .status_change(StatusChangeKind::EnchantArms)
            .and_then(|change| Element::try_from_value(change.values[0] as usize).ok())
            .or_else(|| {
                [
                    (StatusChangeKind::WaterWeapon, Element::Water),
                    (StatusChangeKind::EarthWeapon, Element::Earth),
                    (StatusChangeKind::FireWeapon, Element::Fire),
                    (StatusChangeKind::WindWeapon, Element::Wind),
                    (StatusChangeKind::EnchantPoison, Element::Poison),
                    (StatusChangeKind::Aspersio, Element::Holy),
                    (StatusChangeKind::ShadowWeapon, Element::Dark),
                    (StatusChangeKind::GhostWeapon, Element::Ghost),
                ]
                .into_iter()
                .find_map(|(kind, element)| source.has_status_change(kind).then_some(element))
            })
            .unwrap_or(default)
    }

    fn apply_element_attack_bonus(&self, damage: f32, source: &StatusSnapshot, element: &Element, magic: bool) -> f32 {
        let generic = sum_bonus(source, |bonus| match bonus {
            BonusType::DamageUsingElementPercentage(filter, percent) if *filter == Element::AllElement || filter == element => {
                Some(i32::from(*percent))
            }
            _ => None,
        });
        let damage = scale_damage(damage, generic);
        if magic {
            scale_damage(
                damage,
                sum_bonus(source, |bonus| match bonus {
                    BonusType::MagicalDamageUsingElementPercentage(filter, percent)
                        if *filter == Element::AllElement || filter == element =>
                    {
                        Some(i32::from(*percent))
                    }
                    _ => None,
                }),
            )
        } else {
            damage
        }
    }

    pub fn apply_damage_bonus_modifier(&self, current_atk: f32, source_status: &StatusSnapshot, target_status: &StatusSnapshot) -> f32 {
        self.apply_damage_bonus_modifier_with_rules(current_atk, source_status, target_status, SkillDamageRules::default())
    }

    fn apply_damage_bonus_modifier_with_rules(
        &self,
        current_atk: f32,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        rules: SkillDamageRules,
    ) -> f32 {
        // todo include Star crumb
        // race, mob group, size, element (addRace, addRace2, addSize, addEle)
        // star crumb, ranked blacksmith weapon, ranged attack bonus,
        // based on def (bDefRatioAtk*)
        // Turtle general, Randgris like card (addClass)
        // Frenzy/Edp
        // spirit ball

        // Notes:
        // star crumb: ignored by shield boomerang skill
        // spirit ball, shouldcount spirit ball wehn casting fingeroffensive, otherwise
        // use current value
        let mut current_atk = current_atk;
        if !rules.ignore_element {
            current_atk = scale_damage(
                current_atk,
                sum_bonus(source_status, |bonus| match bonus {
                    BonusType::PhysicalDamageAgainstElementPercentage(element, percent)
                        if *element == Element::AllElement || element == target_status.element() =>
                    {
                        Some(*percent as i32)
                    }
                    _ => None,
                }),
            );
        }
        current_atk = scale_damage(
            current_atk,
            sum_bonus(source_status, |bonus| match bonus {
                BonusType::PhysicalDamageAgainstRacePercentage(race, percent) if *race == MobRace::All || race == target_status.race() => {
                    Some(*percent as i32)
                }
                _ => None,
            }),
        );
        current_atk = scale_damage(
            current_atk,
            sum_bonus(source_status, |bonus| match bonus {
                BonusType::PhysicalDamageAgainstSizePercentage(size, percent) if *size == Size::All || size == target_status.size() => {
                    Some(*percent as i32)
                }
                _ => None,
            }),
        );
        let class = self.target_class(target_status);
        current_atk = scale_damage(
            current_atk,
            sum_bonus(source_status, |bonus| match bonus {
                BonusType::PhysicalDamageAgainstClassPercentage(filter, percent) if *filter == MobClass::All || *filter == class => {
                    Some(*percent as i32)
                }
                _ => None,
            }),
        );
        let advanced_katar = known_skill_level(source_status, SkillEnum::AscKatar);
        if *source_status.right_hand_weapon_type() == WeaponType::Katar && advanced_katar > 0 {
            current_atk = scale_damage(current_atk, 10 + 2 * advanced_katar);
        }
        current_atk = scale_damage(
            current_atk,
            sum_bonus(source_status, |bonus| match bonus {
                BonusType::PhysicalDamageAgainstMobIdPercentage(id, percent) if *id == target_status.job() => Some(*percent as i32),
                _ => None,
            }),
        );
        current_atk = scale_damage(
            current_atk,
            sum_bonus(source_status, |bonus| match bonus {
                BonusType::DamageAgainstMobGroupPercentage(group, percent) if self.matches_mob_group(*group, target_status) => {
                    Some(*percent as i32)
                }
                _ => None,
            }),
        );
        current_atk
    }

    fn target_class(&self, target: &StatusSnapshot) -> MobClass {
        *target.mob_class()
    }

    fn apply_weapon_damage_rate(&self, damage: f32, source: &StatusSnapshot) -> f32 {
        let percent = sum_bonus(source, |bonus| match bonus {
            BonusType::ConditionalWeaponDamagePercentage(weapon, value) if weapon == source.right_hand_weapon_type() => Some(*value),
            _ => None,
        });
        damage + (damage * percent as f32 / 100.0).trunc()
    }

    fn apply_skill_damage_modifiers(&self, mut damage: f32, source: &StatusSnapshot, target: &StatusSnapshot, skill_id: u32) -> f32 {
        if skill_id == 0 {
            return damage;
        }
        let attack = sum_bonus(source, |bonus| match bonus {
            BonusType::SkillIdDamagePercentage(id, percent) if *id == skill_id => Some(i32::from(*percent)),
            _ => None,
        });
        let resistance = sum_bonus(target, |bonus| match bonus {
            BonusType::ResistanceSkillIdPercentage(id, percent) if *id == skill_id => Some(*percent),
            _ => None,
        })
        .min(100);
        damage += (damage * attack as f32 / 100.0).trunc();
        damage -= (damage * resistance as f32 / 100.0).trunc();
        damage
    }

    pub fn is_infinite_defense(&self, target: &StatusSnapshot, flags: u32) -> bool {
        self.configuration_service.get_mob_safe(target.job() as i32).is_some_and(|mob| {
            mob.mode as u32 & MobMode::Plant.as_flag() != 0
                || flags & BattleFlag::Weapon.as_flag() != 0
                    && mob.damage_modes.contains(&if flags & BattleFlag::Long.as_flag() != 0 {
                        MobDamageMode::IgnoreRanged
                    } else {
                        MobDamageMode::IgnoreMelee
                    })
                || flags & BattleFlag::Magic.as_flag() != 0 && mob.damage_modes.contains(&MobDamageMode::IgnoreMagic)
                || flags & BattleFlag::Misc.as_flag() != 0 && mob.damage_modes.contains(&MobDamageMode::IgnoreMisc)
        })
    }

    fn matches_mob_group(&self, group: MobGroup, status: &StatusSnapshot) -> bool {
        if group == MobGroup::Ninja && JobName::try_from_value(status.job() as usize).is_ok_and(|job| job == JobName::Ninja) {
            return true;
        }
        let name = group.as_str().strip_prefix("RC2_").unwrap_or(group.as_str());
        self.configuration_service
            .get_mob_safe(status.job() as i32)
            .is_some_and(|mob| mob.race_groups.iter().any(|candidate| candidate.eq_ignore_ascii_case(name)))
    }

    pub fn apply_damage_reduction(
        &self,
        damage: f32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        element: &Element,
        flags: u32,
    ) -> f32 {
        self.apply_damage_reduction_with_rules(damage, source, target, element, flags, SkillDamageRules::default())
    }

    fn apply_damage_reduction_with_rules(
        &self,
        damage: f32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        element: &Element,
        flags: u32,
        rules: SkillDamageRules,
    ) -> f32 {
        self.apply_damage_reduction_with_options(damage, source, target, element, flags, rules, true)
    }

    fn apply_damage_reduction_with_options(
        &self,
        damage: f32,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        element: &Element,
        flags: u32,
        rules: SkillDamageRules,
        apply_infinite_defense: bool,
    ) -> f32 {
        if rules.ignore_defense_cards {
            return if apply_infinite_defense && damage > 0.0 && flags != 0 && self.is_infinite_defense(target, flags) {
                1.0
            } else {
                damage
            };
        }
        let class = self.target_class(source);
        let miscellaneous = flags & BattleFlag::Misc.as_flag() != 0;
        let mut damage = damage;
        let mut cardfix = 1000_i64;
        let categories = if miscellaneous { [0, 1, 2, 5, 3, 4] } else { [0, 1, 2, 3, 4, 5] };
        for category in categories {
            if category == 0 && rules.ignore_element {
                continue;
            }
            let reduction = sum_bonus(target, |bonus| match (category, bonus) {
                (0, BonusType::ResistanceDamageFromElementPercentage(filter, percent))
                    if *filter == Element::AllElement || filter == element =>
                {
                    Some(*percent as i32)
                }
                (0, BonusType::ResistanceDamageFromElementWithFlags((filter, required), percent))
                    if (*filter == Element::AllElement || filter == element) && BattleFlag::matches(*required, flags) =>
                {
                    Some(*percent as i32)
                }
                (1, BonusType::ResistanceDamageFromRacePercentage(race, percent)) if *race == MobRace::All || race == source.race() => {
                    Some(*percent as i32)
                }
                (1, BonusType::ResistanceDamageFromRaceWithFlags((race, required), percent))
                    if (*race == MobRace::All || race == source.race()) && BattleFlag::matches(*required, flags) =>
                {
                    Some(*percent as i32)
                }
                (2, BonusType::ResistanceDamageFromSizePercentage(size, percent)) if *size == Size::All || size == source.size() => {
                    Some(*percent as i32)
                }
                (3, BonusType::ResistanceDamageFromClassPercentage(filter, percent)) if *filter == MobClass::All || *filter == class => {
                    Some(*percent as i32)
                }
                (4, BonusType::ResistancePhysicalAttackFromMobIdPercentage(id, percent))
                    if *id == source.job() && flags & BattleFlag::Weapon.as_flag() != 0 && flags & BattleFlag::Misc.as_flag() == 0 =>
                {
                    Some(*percent as i32)
                }
                (5, BonusType::ResistanceDamageFromMobGroupPercentage(group, percent)) if self.matches_mob_group(*group, source) => {
                    Some(*percent as i32)
                }
                _ => None,
            });
            if miscellaneous {
                cardfix = cardfix.saturating_mul(100 - i64::from(reduction.min(100))) / 100;
            } else {
                damage = scale_damage(damage, -reduction.min(100));
            }
        }
        if miscellaneous {
            let reduction = sum_bonus(target, |bonus| match bonus {
                BonusType::ResistanceMiscAttackPercentage(percent) => Some(*percent),
                _ => None,
            })
            .min(100);
            cardfix = cardfix.saturating_mul(100 - i64::from(reduction)) / 100;
            if flags & BattleFlag::Long.as_flag() != 0 {
                let reduction = sum_bonus(target, |bonus| match bonus {
                    BonusType::ResistanceRangeAttackPercentage(percent) => Some(i32::from(*percent)),
                    _ => None,
                })
                .min(100);
                cardfix = cardfix.saturating_mul(100 - i64::from(reduction)) / 100;
            }
            let raw = damage.trunc() as i64;
            damage = (i128::from(raw) - i128::from(raw) * (1000 - i128::from(cardfix.max(0))) / 1000) as f32;
        } else if flags & BattleFlag::Magic.as_flag() != 0 {
            damage = scale_damage(
                damage,
                -sum_bonus(target, |bonus| match bonus {
                    BonusType::ResistanceMagicAttackPercentage(percent) => Some(*percent as i32),
                    _ => None,
                })
                .min(100),
            );
        } else if flags & BattleFlag::Long.as_flag() != 0 {
            damage = scale_damage(
                damage,
                -sum_bonus(target, |bonus| match bonus {
                    BonusType::ResistanceRangeAttackPercentage(percent) => Some(*percent as i32),
                    _ => None,
                })
                .min(100),
            );
        }
        if apply_infinite_defense && damage > 0.0 && flags != 0 && self.is_infinite_defense(target, flags) {
            1.0
        } else {
            damage
        }
    }

    pub fn basic_attack(
        &self,
        character: &mut Character,
        target: MapItemSnapshot,
        source_status: &StatusSnapshot,
        target_status: &StatusSnapshot,
        tick: u128,
    ) -> Option<Damage> {
        character.attack?;
        let attack = character.attack();

        let attack_motion = self.status_service.attack_motion(source_status);

        if tick < attack.last_attack_tick + attack_motion as u128 {
            return None;
        }
        // Set atomic timing for cross-thread synchronization
        let canmove_tick = tick + attack_motion as u128;
        character.timing.set_canmove_tick(canmove_tick);
        character.timing.set_canact_tick(canmove_tick);

        if !attack.repeat {
            // one shot attack
            character.clear_attack();
        } else {
            character.update_last_attack_tick(tick);
            character.update_last_attack_motion(attack_motion);
        }
        let mut packet_zc_notify_act3 = PacketZcNotifyAct::new(self.configuration_service.packetver());
        packet_zc_notify_act3.set_target_gid(attack.target);
        packet_zc_notify_act3.set_action(ActionType::Attack.value() as u8);
        packet_zc_notify_act3.set_gid(character.char_id);
        packet_zc_notify_act3.set_attack_mt(attack_motion as i32 / 2);
        let ranged = source_status.right_hand_weapon_type().is_ranged();
        let mut rng = fastrand::Rng::new();
        let rolls_enabled = matches!(self.battle_result_mode, BattleResultMode::Normal);
        let outcome = if rolls_enabled {
            Self::normal_attack_roll(source_status, target_status, ranged, &mut rng)
        } else {
            NormalAttackRoll::Hit
        };
        let lucky_dodge = outcome == NormalAttackRoll::LuckyDodge;
        let double_attack = outcome == NormalAttackRoll::DoubleAttack;
        let critical = outcome == NormalAttackRoll::Critical;
        let hit = !matches!(outcome, NormalAttackRoll::Miss | NormalAttackRoll::LuckyDodge);
        if lucky_dodge {
            packet_zc_notify_act3.set_action(ActionType::AttackLucky.value() as u8);
        } else if critical {
            packet_zc_notify_act3.set_action(ActionType::AttackCritical.value() as u8);
        } else if double_attack {
            packet_zc_notify_act3.set_action(ActionType::AttackMultiple.value() as u8);
        }
        let (right_hand_damage, left_hand_damage) = self.normal_weapon_damage_signed_parts(source_status, target_status, outcome);
        let target_damage_motion = if matches!(target.map_item.object_type(), MapItemType::Mob) {
            self.configuration_service
                .get_mob(target.map_item.client_item_class() as i32)
                .damage_motion as u32
        } else {
            480
        };
        let signed_damage = right_hand_damage.saturating_add(left_hand_damage);
        let damage = signed_damage.max(0) as u32;
        packet_zc_notify_act3.set_attacked_mt(target_damage_motion as i32);
        packet_zc_notify_act3.set_damage(right_hand_damage.clamp(i16::MIN as i32, i16::MAX as i32) as i16);
        packet_zc_notify_act3.set_left_damage(left_hand_damage.clamp(i16::MIN as i32, i16::MAX as i32) as i16);
        packet_zc_notify_act3.set_count(if double_attack { 2 } else { 1 });
        packet_zc_notify_act3.fill_raw();
        self.client_notification_sender
            .send(Notification::Area(AreaNotification::new(
                character.current_map_name().clone(),
                character.current_map_instance(),
                AreaNotificationRangeType::Fov {
                    x: character.x,
                    y: character.y,
                    exclude_id: None,
                },
                mem::take(packet_zc_notify_act3.raw_mut()),
            )))
            .unwrap_or_else(|_| error!("Failed to send notification packet_zc_notify_act3 to client"));
        Some(Damage {
            target_id: attack.target,
            attacker_id: character.char_id,
            damage,
            healing: if signed_damage < 0 { signed_damage.unsigned_abs() } else { 0 },
            right_hand_damage: Some(right_hand_damage.max(0) as u32),
            attacked_at: tick + attack_motion as u128,
            damage_motion: target_damage_motion,
            battle_flags: BattleFlag::Weapon.as_flag()
                | BattleFlag::Normal.as_flag()
                | if ranged {
                    BattleFlag::Long.as_flag()
                } else {
                    BattleFlag::Short.as_flag()
                },
            skill_id: 0,
            skill_level: 0,
            landed: hit,
            proc_depth: 0,
            credit_id: character.char_id,
            defenses_applied: true,
            magic_context: None,
        })
    }

    #[inline]
    pub fn size_modifier(source_status: &StatusSnapshot, target_status: &StatusSnapshot) -> f32 {
        // Size Modifiers for Weapons
        // Size 	Fist 	Dagger 	1H Sword 	2H Sword 	Spear 	Spear+Peco 	Axe 	Mace 	Rod
        // Bow 	Katar 	Book 	Claw 	Instrument 	Whip 	Gun 	Huuma Shuriken
        // Small 	100 	100 	75       	75       	75  	75 	        50  	75  	100 	100 	75
        // 100 	100 	75 	        75 	    100 	100 Medium 	100 	75  	100      	75
        // 75  	100 	    75  	100 	100 	100 	100 	100 	75 	    100 	    100 	100 	100
        // Large 	100 	50  	75      	100     	100 	100 	    100 	100 	100 	75  	75
        // 50 	    50 	    75 	        50 	    100 	100
        let weapon_type = source_status.right_hand_weapon_type();
        match weapon_type {
            WeaponType::Fist => match target_status.size() {
                Size::Small => 1.0,
                Size::Medium => 1.0,
                Size::Large => 1.0,
                _ => 1.0,
            },
            WeaponType::Dagger => match target_status.size() {
                Size::Small => 1.0,
                Size::Medium => 0.75,
                Size::Large => 0.5,
                _ => 1.0,
            },
            WeaponType::Sword1H => match target_status.size() {
                Size::Small => 0.75,
                Size::Medium => 1.0,
                Size::Large => 0.75,
                _ => 1.0,
            },
            WeaponType::Sword2H => match target_status.size() {
                Size::Small => 0.75,
                Size::Medium => 0.75,
                Size::Large => 1.0,
                _ => 1.0,
            },
            WeaponType::Spear1H | WeaponType::Spear2H => {
                if source_status.state() & SkillState::Riding.as_flag() > 0 {
                    match target_status.size() {
                        Size::Small => 0.75,
                        Size::Medium => 1.0,
                        Size::Large => 1.0,
                        _ => 1.0,
                    }
                } else {
                    match target_status.size() {
                        Size::Small => 0.75,
                        Size::Medium => 0.75,
                        Size::Large => 1.0,
                        _ => 1.0,
                    }
                }
            }
            WeaponType::Axe1H | WeaponType::Axe2H => match target_status.size() {
                Size::Small => 0.5,
                Size::Medium => 0.75,
                Size::Large => 1.0,
                _ => 1.0,
            },
            WeaponType::Mace | WeaponType::Mace2H => match target_status.size() {
                Size::Small => 0.75,
                Size::Medium => 1.0,
                Size::Large => 1.0,
                _ => 1.0,
            },
            WeaponType::Staff | WeaponType::Staff2H => match target_status.size() {
                Size::Small => 1.0,
                Size::Medium => 1.0,
                Size::Large => 1.0,
                _ => 1.0,
            },
            WeaponType::Bow => match target_status.size() {
                Size::Small => 1.0,
                Size::Medium => 1.0,
                Size::Large => 0.75,
                _ => 1.0,
            },
            WeaponType::Knuckle => match target_status.size() {
                Size::Small => 1.0,
                Size::Medium => 0.75,
                Size::Large => 0.5,
                _ => 1.0,
            },
            WeaponType::Musical => match target_status.size() {
                Size::Small => 0.75,
                Size::Medium => 1.0,
                Size::Large => 0.75,
                _ => 1.0,
            },
            WeaponType::Whip => match target_status.size() {
                Size::Small => 0.75,
                Size::Medium => 1.0,
                Size::Large => 0.5,
                _ => 1.0,
            },
            WeaponType::Book => match target_status.size() {
                Size::Small => 1.0,
                Size::Medium => 1.0,
                Size::Large => 0.5,
                _ => 1.0,
            },
            WeaponType::Katar => match target_status.size() {
                Size::Small => 0.75,
                Size::Medium => 1.0,
                Size::Large => 0.75,
                _ => 1.0,
            },
            WeaponType::Revolver | WeaponType::Rifle | WeaponType::Gatling | WeaponType::Shotgun | WeaponType::Grenade => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0,
                    _ => 1.0,
                }
            }
            WeaponType::Huuma | WeaponType::Shuriken => match target_status.size() {
                Size::Small => 1.0,
                Size::Medium => 1.0,
                Size::Large => 1.0,
                _ => 1.0,
            },
            _ => 1.0,
        }
    }

    #[inline]
    pub fn element_modifier(element: &Element, target_status: &StatusSnapshot) -> f32 {
        if target_status.element_level() == 1 || target_status.element_level() == 0 {
            match target_status.element() {
                Element::Neutral => {
                    if matches!(element, Element::Ghost) {
                        0.25
                    } else {
                        1.0
                    }
                }
                Element::Water => {
                    if matches!(element, Element::Water) {
                        0.25
                    } else if matches!(element, Element::Fire) {
                        0.5
                    } else if matches!(element, Element::Wind) {
                        1.75
                    } else {
                        1.0
                    }
                }
                Element::Earth => {
                    if matches!(element, Element::Fire) {
                        1.5
                    } else if matches!(element, Element::Wind) {
                        0.5
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Fire => {
                    if matches!(element, Element::Water) {
                        1.5
                    } else if matches!(element, Element::Earth) {
                        0.5
                    } else if matches!(element, Element::Fire) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Wind => {
                    if matches!(element, Element::Water) {
                        0.5
                    } else if matches!(element, Element::Earth) {
                        1.5
                    } else if matches!(element, Element::Wind) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Poison => {
                    if matches!(element, Element::Dark) || matches!(element, Element::Undead) {
                        0.5
                    } else if matches!(element, Element::Poison) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Holy => {
                    if matches!(element, Element::Dark) {
                        1.25
                    } else if matches!(element, Element::Holy) {
                        0.0
                    } else if matches!(element, Element::Undead) || matches!(element, Element::Neutral) {
                        1.0
                    } else {
                        0.75
                    }
                }
                Element::Dark => {
                    if matches!(element, Element::Dark) {
                        0.0
                    } else if matches!(element, Element::Holy) {
                        1.25
                    } else if matches!(element, Element::Poison) {
                        0.5
                    } else if matches!(element, Element::Ghost) {
                        0.75
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Ghost => {
                    if matches!(element, Element::Neutral) {
                        0.25
                    } else if matches!(element, Element::Ghost) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Undead => {
                    if matches!(element, Element::Fire) {
                        1.25
                    } else if matches!(element, Element::Holy) {
                        1.5
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Dark) {
                        -0.25
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else {
                        1.0
                    }
                }
                _ => 1.0,
            }
        } else if target_status.element_level() == 2 {
            match target_status.element() {
                Element::Neutral => {
                    if matches!(element, Element::Ghost) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Water => {
                    if matches!(element, Element::Water) {
                        0.0
                    } else if matches!(element, Element::Fire) {
                        0.25
                    } else if matches!(element, Element::Wind) {
                        1.75
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Ghost) || matches!(element, Element::Undead)
                    {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Earth => {
                    if matches!(element, Element::Fire) {
                        1.75
                    } else if matches!(element, Element::Earth) {
                        0.5
                    } else if matches!(element, Element::Wind) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Fire => {
                    if matches!(element, Element::Water) {
                        1.75
                    } else if matches!(element, Element::Earth) {
                        0.25
                    } else if matches!(element, Element::Fire) {
                        0.0
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Wind => {
                    if matches!(element, Element::Water) {
                        0.25
                    } else if matches!(element, Element::Earth) {
                        1.75
                    } else if matches!(element, Element::Wind) {
                        0.0
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Poison => {
                    if matches!(element, Element::Dark) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Holy => {
                    if matches!(element, Element::Dark) {
                        1.5
                    } else if matches!(element, Element::Holy) {
                        -0.25
                    } else if matches!(element, Element::Undead) {
                        1.25
                    } else if matches!(element, Element::Neutral) {
                        1.0
                    } else {
                        0.5
                    }
                }
                Element::Dark => {
                    if matches!(element, Element::Neutral) {
                        1.0
                    } else if matches!(element, Element::Dark) {
                        -0.25
                    } else if matches!(element, Element::Holy) {
                        1.5
                    } else if matches!(element, Element::Poison) {
                        0.25
                    } else if matches!(element, Element::Ghost) {
                        0.5
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else {
                        0.75
                    }
                }
                Element::Ghost => {
                    if matches!(element, Element::Neutral) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        0.75
                    } else if matches!(element, Element::Ghost) {
                        1.5
                    } else {
                        1.0
                    }
                }
                Element::Undead => {
                    if matches!(element, Element::Fire) {
                        1.5
                    } else if matches!(element, Element::Holy) {
                        1.75
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Dark) {
                        -0.5
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        1.25
                    } else {
                        1.0
                    }
                }
                _ => 1.0,
            }
        } else if target_status.element_level() == 3 {
            match target_status.element() {
                Element::Neutral => {
                    if matches!(element, Element::Ghost) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Water => {
                    if matches!(element, Element::Water) {
                        -0.25
                    } else if matches!(element, Element::Fire) {
                        0.0
                    } else if matches!(element, Element::Wind) {
                        2.0
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Ghost) || matches!(element, Element::Undead)
                    {
                        0.5
                    } else {
                        1.0
                    }
                }
                Element::Earth => {
                    if matches!(element, Element::Fire) {
                        2.0
                    } else if matches!(element, Element::Earth) {
                        0.0
                    } else if matches!(element, Element::Wind) {
                        0.0
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.5
                    } else {
                        1.0
                    }
                }
                Element::Fire => {
                    if matches!(element, Element::Water) {
                        2.0
                    } else if matches!(element, Element::Earth) {
                        0.0
                    } else if matches!(element, Element::Fire) {
                        -0.25
                    } else if matches!(element, Element::Poison) {
                        1.0
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.5
                    } else {
                        1.0
                    }
                }
                Element::Wind => {
                    if matches!(element, Element::Water) {
                        0.0
                    } else if matches!(element, Element::Earth) {
                        2.0
                    } else if matches!(element, Element::Wind) {
                        -0.25
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.5
                    } else {
                        1.0
                    }
                }
                Element::Poison => {
                    if matches!(element, Element::Undead) || matches!(element, Element::Dark) || matches!(element, Element::Poison) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        0.5
                    } else if matches!(element, Element::Holy) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Holy => {
                    if matches!(element, Element::Dark) {
                        1.75
                    } else if matches!(element, Element::Holy) {
                        -0.5
                    } else if matches!(element, Element::Undead) {
                        1.5
                    } else if matches!(element, Element::Neutral) {
                        1.0
                    } else {
                        0.25
                    }
                }
                Element::Dark => {
                    if matches!(element, Element::Dark) {
                        -0.5
                    } else if matches!(element, Element::Holy) {
                        1.5
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Undead) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        0.25
                    } else {
                        0.5
                    }
                }
                Element::Ghost => {
                    if matches!(element, Element::Neutral) {
                        0.0
                    } else if matches!(element, Element::Poison) {
                        0.5
                    } else if matches!(element, Element::Ghost) {
                        1.75
                    } else {
                        1.0
                    }
                }
                Element::Undead => {
                    if matches!(element, Element::Fire) {
                        1.75
                    } else if matches!(element, Element::Water) {
                        1.25
                    } else if matches!(element, Element::Earth) {
                        0.75
                    } else if matches!(element, Element::Holy) {
                        2.0
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Dark) {
                        -0.75
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        1.5
                    } else {
                        1.0
                    }
                }
                _ => 1.0,
            }
        } else if target_status.element_level() == 4 {
            match target_status.element() {
                Element::Neutral => {
                    if matches!(element, Element::Ghost) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Water => {
                    if matches!(element, Element::Water) {
                        -0.5
                    } else if matches!(element, Element::Fire) {
                        0.0
                    } else if matches!(element, Element::Wind) {
                        2.0
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Ghost) || matches!(element, Element::Undead)
                    {
                        0.25
                    } else if matches!(element, Element::Holy) || matches!(element, Element::Dark) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Earth => {
                    if matches!(element, Element::Fire) {
                        2.0
                    } else if matches!(element, Element::Earth) {
                        -0.25
                    } else if matches!(element, Element::Wind) {
                        0.0
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Holy) || matches!(element, Element::Dark) || matches!(element, Element::Poison) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Fire => {
                    if matches!(element, Element::Water) {
                        2.0
                    } else if matches!(element, Element::Earth) {
                        0.0
                    } else if matches!(element, Element::Fire) {
                        -0.5
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Holy) || matches!(element, Element::Dark) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Wind => {
                    if matches!(element, Element::Water) {
                        0.0
                    } else if matches!(element, Element::Earth) {
                        2.0
                    } else if matches!(element, Element::Wind) {
                        -0.5
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Holy) || matches!(element, Element::Dark) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Poison => {
                    if matches!(element, Element::Undead) || matches!(element, Element::Dark) {
                        -0.25
                    } else if matches!(element, Element::Ghost) {
                        0.25
                    } else if matches!(element, Element::Holy) {
                        1.25
                    } else if matches!(element, Element::Neutral) {
                        1.0
                    } else if matches!(element, Element::Poison) {
                        0.0
                    } else {
                        0.75
                    }
                }
                Element::Holy => {
                    if matches!(element, Element::Dark) {
                        2.0
                    } else if matches!(element, Element::Holy) {
                        -1.0
                    } else if matches!(element, Element::Undead) {
                        1.75
                    } else if matches!(element, Element::Neutral) {
                        1.0
                    } else {
                        0.0
                    }
                }
                Element::Dark => {
                    if matches!(element, Element::Dark) {
                        -1.0
                    } else if matches!(element, Element::Holy) {
                        2.0
                    } else if matches!(element, Element::Neutral) {
                        1.0
                    } else if matches!(element, Element::Poison) {
                        -0.25
                    } else if matches!(element, Element::Undead) || matches!(element, Element::Ghost) {
                        0.0
                    } else {
                        0.25
                    }
                }
                Element::Ghost => {
                    if matches!(element, Element::Neutral) {
                        0.0
                    } else if matches!(element, Element::Poison) {
                        0.25
                    } else if matches!(element, Element::Ghost) {
                        2.0
                    } else {
                        1.0
                    }
                }
                Element::Undead => {
                    if matches!(element, Element::Fire) || matches!(element, Element::Holy) {
                        2.0
                    } else if matches!(element, Element::Water) {
                        1.5
                    } else if matches!(element, Element::Earth) {
                        0.5
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Dark) {
                        -1.0
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        1.75
                    } else {
                        1.0
                    }
                }
                _ => 1.0,
            }
        } else {
            1.0
        }
    }
}
