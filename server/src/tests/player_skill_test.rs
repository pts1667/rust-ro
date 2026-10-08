//! Player skills that run through the generic metadata paths.
use models::enums::skill_enums::SkillEnum;
use models::status::KnownSkill;
use models::status_change::StatusChangeKind;

use crate::server::Server;
use crate::server::model::events::game_event::CharacterUseSkill;

fn cast_and_settle(skill: SkillEnum, level: u8, until: u128) -> (super::ServerServiceTestContext, u32) {
    let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    let char_id = character.char_id;
    character.status.known_skills = vec![KnownSkill { value: skill, level }];
    context.server.state_mut().insert_character(character);
    context
        .server
        .handle_character_skill(&mut *context.server.state_mut(), CharacterUseSkill { char_id, target_id: char_id, skill_id: skill.id(), skill_level: level }, 0)
        .unwrap();
    for tick in (40..=until).step_by(40) {
        Server::game_loop_iteration(&context.server, tick);
    }
    (context, char_id)
}

#[test]
fn auto_berserk_toggles_through_the_metadata_status_path() {
    let (context, char_id) = cast_and_settle(SkillEnum::SmAutoberserk, 1, 400);
    assert!(context.server.state().get_character(char_id).unwrap().status.has_status_change(StatusChangeKind::AutoBerserk));
    context
        .server
        .handle_character_skill(
            &mut *context.server.state_mut(),
            CharacterUseSkill { char_id, target_id: char_id, skill_id: SkillEnum::SmAutoberserk.id(), skill_level: 1 },
            100_000,
        )
        .unwrap();
    for tick in (100_040..=100_400).step_by(40) {
        Server::game_loop_iteration(&context.server, tick);
    }
    assert!(!context.server.state().get_character(char_id).unwrap().status.has_status_change(StatusChangeKind::AutoBerserk));
}

fn status_character(kinds: &[(StatusChangeKind, i32)]) -> crate::server::state::character::Character {
    let (_context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    for (kind, level) in kinds {
        crate::server::service::status_effect_service::StatusEffectService::apply_status(
            &mut character.status,
            models::status_change::StatusChangeRequest::guaranteed(*kind, 60_000, *level),
            0,
            0,
        )
        .unwrap();
    }
    character
}

#[test]
fn memorize_halves_the_cast_time_of_five_casts() {
    use crate::server::script::skill::ScriptSkillService;
    use crate::server::service::status_service::StatusService;
    let mut character = status_character(&[(StatusChangeKind::Memorize, 1)]);
    let (sender, _receiver) = std::sync::mpsc::sync_channel(64);
    let snapshot = StatusService::instance().to_snapshot(&character.status);
    let plain = StatusService::instance().to_snapshot(&super::native_payment_tests::fixture(false, false).2.status);
    let firebolt = SkillEnum::MgFirebolt.id();
    assert_eq!(StatusService::skill_cast_modifier(&snapshot, firebolt) * 2.0, StatusService::skill_cast_modifier(&plain, firebolt));
    for _ in 0..5 {
        assert!(character.status.has_status_change(StatusChangeKind::Memorize));
        ScriptSkillService::spend_cast_statuses(&mut character, firebolt, 0, &sender);
    }
    assert!(!character.status.has_status_change(StatusChangeKind::Memorize));
}

#[test]
fn mystical_amplification_boosts_one_magic_cast_and_ends_with_the_next() {
    use crate::server::script::skill::ScriptSkillService;
    let mut character = status_character(&[(StatusChangeKind::MagicPower, 5)]);
    let (sender, _receiver) = std::sync::mpsc::sync_channel(64);
    let active = |character: &crate::server::state::character::Character| character.status.status_change(StatusChangeKind::MagicPower).map(|change| change.values[3]);
    ScriptSkillService::spend_cast_statuses(&mut character, SkillEnum::AlHeal.id(), 0, &sender);
    assert_eq!(active(&character), Some(0));
    ScriptSkillService::spend_cast_statuses(&mut character, SkillEnum::MgFirebolt.id(), 0, &sender);
    assert_eq!(active(&character), Some(1));
    assert!(character.status.status_change(StatusChangeKind::MagicPower).unwrap().bonuses().iter().any(|bonus| matches!(bonus, models::enums::bonus::BonusType::MatkPercentage(25))));
    ScriptSkillService::spend_cast_statuses(&mut character, SkillEnum::MgFirebolt.id(), 0, &sender);
    assert_eq!(active(&character), None);
}

#[test]
fn foresight_started_from_the_skill_holds_five_charges() {
    let (context, char_id) = cast_and_settle(SkillEnum::PfMemorize, 1, 6000);
    let change = context.server.state().get_character(char_id).unwrap().status.status_change(StatusChangeKind::Memorize).cloned().unwrap();
    assert_eq!((change.values[1], change.expires_at), (5, None));
}

#[test]
fn martyrs_reckoning_spends_a_charge_and_nine_percent_hp_per_strike() {
    use crate::server::script::skill::ScriptSkillService;
    use crate::server::service::status_service::StatusService;
    let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    crate::server::service::status_effect_service::StatusEffectService::apply_status(
        &mut character.status,
        models::status_change::StatusChangeRequest::guaranteed(StatusChangeKind::Sacrifice, 0, 5),
        0,
        0,
    )
    .unwrap();
    let (sender, _receiver) = std::sync::mpsc::sync_channel(64);
    let max_hp = StatusService::instance().to_snapshot(&character.status).max_hp();
    character.status.hp = max_hp;
    ScriptSkillService::pay_martyrs_reckoning(&context.server, &mut character, 0, &sender);
    assert_eq!(character.status.hp, max_hp - max_hp * 9 / 100);
    assert_eq!(character.status.status_change(StatusChangeKind::Sacrifice).unwrap().values[1], 4);
    for _ in 0..4 {
        ScriptSkillService::pay_martyrs_reckoning(&context.server, &mut character, 0, &sender);
    }
    assert!(!character.status.has_status_change(StatusChangeKind::Sacrifice));
    character.status.hp = 3;
    ScriptSkillService::pay_martyrs_reckoning(&context.server, &mut character, 0, &sender);
    assert_eq!(character.status.hp, 1);
}

#[test]
fn double_casting_repeats_only_bolts_inside_its_chance() {
    use crate::server::model::action::Damage;
    use crate::server::model::map_item::MapItemType;
    use crate::server::script::skill::ScriptSkillService;
    let character = status_character(&[(StatusChangeKind::DoubleCast, 2)]);
    let hit = || {
        vec![(
            MapItemType::Mob,
            Damage {
                notification: None,
                source_kind: models::enums::actor::CombatActorKind::Player,
                skill_damage_adjusted: false,
                healing: 0,
                right_hand_damage: None,
                target_id: 1,
                attacker_id: 2,
                damage: 10,
                attacked_at: 0,
                damage_motion: 0,
                battle_flags: 0,
                skill_id: SkillEnum::MgFirebolt.id(),
                skill_level: 1,
                proc_depth: 0,
                credit_id: 0,
                defenses_applied: true,
                magic_context: None,
                landed: true,
            },
        )]
    };
    assert_eq!(character.status.status_change(StatusChangeKind::DoubleCast).unwrap().values[1], 50);
    assert_eq!(ScriptSkillService::double_cast(&character, SkillEnum::MgFirebolt.id(), hit(), 49).len(), 2);
    assert_eq!(ScriptSkillService::double_cast(&character, SkillEnum::MgFirebolt.id(), hit(), 50).len(), 1);
    assert_eq!(ScriptSkillService::double_cast(&character, SkillEnum::MgNapalmbeat.id(), hit(), 0).len(), 1);
}

#[test]
fn poison_react_envenoms_attackers_until_its_counters_are_spent() {
    use crate::server::script::skill::ScriptSkillService;
    use models::enums::bonus::BonusType;
    use models::status_bonus::CombatTrigger;
    let mut character = status_character(&[(StatusChangeKind::PoisonReact, 5)]);
    let (sender, _receiver) = std::sync::mpsc::sync_channel(64);
    let proc = |character: &crate::server::state::character::Character| {
        character.status.status_change(StatusChangeKind::PoisonReact).and_then(|change| {
            change.bonuses().into_iter().find_map(|bonus| if let BonusType::CombatProc(proc, _) = bonus { Some(proc) } else { None })
        })
    };
    let proc_now = proc(&character).unwrap();
    assert_eq!((proc_now.trigger, proc_now.rate, proc_now.value, proc_now.level), (CombatTrigger::Hit, 5000, SkillEnum::TfPoison.id(), 5));
    ScriptSkillService::spend_poison_react(&mut character, 0, &sender);
    assert!(character.status.has_status_change(StatusChangeKind::PoisonReact));
    ScriptSkillService::spend_poison_react(&mut character, 0, &sender);
    assert!(!character.status.has_status_change(StatusChangeKind::PoisonReact));
}

#[test]
fn marionette_shares_half_of_the_puppeteers_stats() {
    use models::enums::bonus::BonusType;
    let mut character = status_character(&[]);
    let mut request = models::status_change::StatusChangeRequest::guaranteed(StatusChangeKind::Marionette2, -1, 1);
    request.values = [1, 7, 10 | 5 << 8 | 3 << 16, 0];
    crate::server::service::status_effect_service::StatusEffectService::apply_status(&mut character.status, request, 0, 0).unwrap();
    let change = character.status.status_change(StatusChangeKind::Marionette2).unwrap();
    assert_eq!(change.expires_at, None);
    let bonuses = change.bonuses();
    assert!(bonuses.contains(&BonusType::Str(10)) && bonuses.contains(&BonusType::Agi(5)) && bonuses.contains(&BonusType::Vit(3)));
    let mut puppeteer = status_character(&[]);
    let mut request = models::status_change::StatusChangeRequest::guaranteed(StatusChangeKind::Marionette, -1, 1);
    request.values = [1, 9, 10, 0];
    crate::server::service::status_effect_service::StatusEffectService::apply_status(&mut puppeteer.status, request, 0, 0).unwrap();
    assert!(puppeteer.status.status_change(StatusChangeKind::Marionette).unwrap().bonuses().contains(&BonusType::Str(-10)));
}

#[test]
fn skill_nocast_db_bans_skills_by_map_kind() {
    use crate::server::model::map_flags::{MapFlag, MapFlags};
    use crate::server::script::skill::ScriptSkillService;
    let mut flags = MapFlags::default();
    let backsliding = SkillEnum::TfBacksliding.id();
    assert!(!ScriptSkillService::forbidden_on_map(backsliding, &flags));
    flags.set(MapFlag::Gvg, true, &[]).unwrap();
    assert!(ScriptSkillService::forbidden_on_map(backsliding, &flags));
    assert!(!ScriptSkillService::forbidden_on_map(SkillEnum::SmEndure.id(), &flags));
    let mut pvp = MapFlags::default();
    pvp.set(MapFlag::Pvp, true, &[]).unwrap();
    assert!(!ScriptSkillService::forbidden_on_map(backsliding, &pvp));
    assert!(ScriptSkillService::forbidden_on_map(SkillEnum::BsGreed.id(), &pvp));
    let mut zone = MapFlags::default();
    zone.set(MapFlag::Restricted, true, &[1]).unwrap();
    assert!(ScriptSkillService::forbidden_on_map(SkillEnum::SmEndure.id(), &zone));
}
