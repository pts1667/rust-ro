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
