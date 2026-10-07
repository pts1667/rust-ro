use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::quest::{QuestCheck, quest_catalog};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::tests::common::character_helper::create_character;

#[test]
fn party_members_near_the_monster_share_hunting_credit_and_the_log_is_checked() {
    let context = super::before_each();
    let server = &context.server;
    let quest = quest_catalog()
        .values()
        .find(|quest| {
            quest.objectives.len() == 1
                && quest.objectives[0].mob != 0
                && GlobalConfigService::instance().get_mob_safe(i32::from(quest.objectives[0].mob)).is_some()
        })
        .expect("a hunting quest");
    let mob_id = quest.objectives[0].mob;
    let key = MapInstanceKey::new("prontera".into(), 0);
    let mut state = server.state_mut();
    for (char_id, x, party_id) in [(150_001_u32, 50_u16, 7_u32), (150_002, 55, 7), (150_003, 90, 7), (150_004, 50, 0)] {
        let mut character = create_character();
        character.char_id = char_id;
        character.account_id = char_id;
        character.map_instance_key = key.clone();
        character.x = x;
        character.y = 50;
        character.game_systems.party_id = party_id;
        state.insert_character(character);
        server.quest_add(&mut state, char_id, quest.id).unwrap();
    }
    server.quest_kill(&mut state, 150_001, mob_id, &key, 50, 50);
    let count = |char_id| state.get_character(char_id).unwrap().game_systems.quests.find(quest.id).unwrap().counts[0];
    assert_eq!([count(150_001), count(150_002), count(150_003), count(150_004)], [1, 1, 0, 0]);
    assert_eq!(crate::server::Server::quest_check(&state, 150_001, quest.id, QuestCheck::Have), 1);
    server.quest_complete(&mut state, 150_001, quest.id).unwrap();
    assert_eq!(crate::server::Server::quest_check(&state, 150_001, quest.id, QuestCheck::Have), 2);
    assert!(server.quest_add(&mut state, 150_001, quest.id).is_err());
}
