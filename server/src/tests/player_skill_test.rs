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

#[test]
#[ignore = "prints the route of every skill"]
fn print_player_skill_routes() {
    use crate::server::script::skill::ScriptSkillService;
    use crate::server::script::skill::metadata::SkillMetadata;
    use models::enums::skill::SkillType;
    let mut native = std::collections::BTreeMap::<String, Vec<String>>::new();
    for metadata in SkillMetadata::all() {
        let Ok(skill) = SkillEnum::try_from_value(metadata.id) else { continue };
        let route = if ScriptSkillService::operation(&metadata.name).is_some() {
            "script".to_string()
        } else {
            match skills::skill_enums::to_object(skill, 1) {
                Some(object) => format!("native-{:?}", object.skill_type()),
                None => "none".to_string(),
            }
        };
        let _ = SkillType::Passive;
        native.entry(route).or_default().push(format!("{}[{}]", metadata.name, metadata.target_type.as_deref().unwrap_or("-")));
    }
    for (route, names) in &native {
        println!("ROUTE {route} ({}): {}", names.len(), names.join(" "));
    }
}
