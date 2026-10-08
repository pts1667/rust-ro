use models::enums::EnumWithNumberValue;
use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::status_change::StatusChangeKind;

use crate::server::request_handler::atcommand_extra;
use crate::server::service::script_character_service::learned_level;
use crate::server::service::status_effect_service::StatusEffectService;

#[test]
fn feelreset_and_hatereset_forget_the_star_memory_of_a_star_gladiator() {
    let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    character.status.job = JobName::StarGladiator.value() as u32;
    character.game_systems.star_places[0] = "prontera".into();
    character.game_systems.star_hates[1] = 1038;
    let char_id = character.char_id;
    context.server.state_mut().insert_character(character);
    let mut state = context.server.state_mut();

    let hated = atcommand_extra::handle(&context.server, &mut state, char_id, "hatereset", &[]).unwrap();
    assert_eq!(hated, vec!["Reset 'Hatred' monsters.".to_string()]);
    let character = state.get_character(char_id).unwrap();
    assert_eq!(character.game_systems.star_hates, [0; 3]);
    assert_eq!(character.game_systems.star_places[0], "prontera");

    let felt = atcommand_extra::handle(&context.server, &mut state, char_id, "feelreset", &[]).unwrap();
    assert_eq!(felt, vec!["Reset 'Feeling' maps.".to_string()]);
    assert!(state.get_character(char_id).unwrap().game_systems.star_places.iter().all(String::is_empty));
}

#[test]
fn star_memory_commands_refuse_other_classes() {
    let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    character.game_systems.star_places[0] = "prontera".into();
    let char_id = character.char_id;
    context.server.state_mut().insert_character(character);
    let mut state = context.server.state_mut();

    let refused = atcommand_extra::handle(&context.server, &mut state, char_id, "feelreset", &[]).unwrap();
    assert_eq!(refused, vec!["You can't use this command with this class.".to_string()]);
    assert_eq!(state.get_character(char_id).unwrap().game_systems.star_places[0], "prontera");
}

#[test]
fn allskill_raises_the_job_tree_to_maximum_levels_except_devil() {
    let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    character.status.job = JobName::StarGladiator.value() as u32;
    let char_id = character.char_id;
    context.server.state_mut().insert_character(character);
    let mut state = context.server.state_mut();

    let reply = atcommand_extra::handle(&context.server, &mut state, char_id, "allskill", &[]).unwrap();
    assert_eq!(reply, vec!["All skills have been added to your skill tree.".to_string()]);
    let character = state.get_character(char_id).unwrap();
    assert_eq!(learned_level(&character.status, SkillEnum::SgFriend.id()), 3);
    assert_eq!(learned_level(&character.status, SkillEnum::SgDevil.id()), 0);
    assert_eq!(character.status.skill_point, 0);
}

#[test]
fn skilltree_reports_the_requirements_of_a_skill_in_the_target_tree() {
    let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    character.status.job = JobName::StarGladiator.value() as u32;
    character.loaded_from_client_side = true;
    let char_id = character.char_id;
    let name = character.name.clone();
    context.server.state_mut().insert_character(character);
    let mut state = context.server.state_mut();

    let missing = atcommand_extra::handle(&context.server, &mut state, char_id, "skilltree", &["1", &name]).unwrap();
    assert!(missing[0].starts_with("Player is using "));
    let unknown = atcommand_extra::handle(&context.server, &mut state, char_id, "skilltree", &["1", "nobody"]).unwrap();
    assert_eq!(unknown, vec!["Character not found.".to_string()]);
}

fn learn(character: &mut crate::server::state::character::Character, skill: SkillEnum, level: u8) {
    character.status.known_skills.push(models::status::KnownSkill { value: skill, level });
}

#[test]
fn friend_counter_raises_the_triple_attack_rate_of_party_monks_on_the_same_map() {
    let (context, _repository, mut caster) = super::native_payment_tests::fixture(false, false);
    caster.status.job = JobName::StarGladiator.value() as u32;
    caster.game_systems.party_id = 77;
    learn(&mut caster, SkillEnum::SgFriend, 2);
    let mut monk = crate::tests::common::character_helper::create_character();
    monk.char_id = caster.char_id + 1;
    monk.status.hp = 100;
    monk.status.job = JobName::Monk.value() as u32;
    monk.game_systems.party_id = 77;
    monk.map_instance_key = caster.map_instance_key.clone();
    learn(&mut monk, SkillEnum::MoTripleattack, 5);
    let (caster_id, monk_id) = (caster.char_id, monk.char_id);
    context.server.state_mut().insert_character(caster);
    context.server.state_mut().insert_character(monk);
    let mut state = context.server.state_mut();

    context.server.script_skill_service().share_friend_rate(&context.server, &mut state, caster_id, SkillEnum::TkCounter.id(), 0).unwrap();

    let change = state.get_character(monk_id).unwrap().status.status_change(StatusChangeKind::SkillRateUp).cloned().unwrap();
    assert_eq!(change.values[0], SkillEnum::MoTripleattack.id() as i32);
    assert_eq!(change.values[1], 150);
}

#[test]
fn friend_combo_finish_raises_the_counter_rate_of_party_star_gladiators_with_a_ready_counter() {
    let (context, _repository, mut monk) = super::native_payment_tests::fixture(false, false);
    monk.status.job = JobName::Monk.value() as u32;
    monk.game_systems.party_id = 78;
    let mut gladiator = crate::tests::common::character_helper::create_character();
    gladiator.char_id = monk.char_id + 1;
    gladiator.status.hp = 100;
    gladiator.status.job = JobName::StarGladiator.value() as u32;
    gladiator.game_systems.party_id = 78;
    gladiator.map_instance_key = monk.map_instance_key.clone();
    learn(&mut gladiator, SkillEnum::SgFriend, 1);
    let mut request = models::status_change::StatusChangeRequest::guaranteed(StatusChangeKind::ReadyCounter, 60_000, 0);
    request.values = [0; 4];
    StatusEffectService::apply_status(&mut gladiator.status, request, 0, 0).unwrap();
    let (monk_id, gladiator_id) = (monk.char_id, gladiator.char_id);
    context.server.state_mut().insert_character(monk);
    context.server.state_mut().insert_character(gladiator);
    let mut state = context.server.state_mut();

    context.server.script_skill_service().share_friend_rate(&context.server, &mut state, monk_id, SkillEnum::MoCombofinish.id(), 0).unwrap();

    let change = state.get_character(gladiator_id).unwrap().status.status_change(StatusChangeKind::SkillRateUp).cloned().unwrap();
    assert_eq!(change.values[0], SkillEnum::TkCounter.id() as i32);
    assert_eq!(change.values[1], 100);
}
