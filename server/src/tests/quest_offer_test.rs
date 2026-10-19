use std::sync::Arc;

use models::enums::EnumWithMaskValueU16;
use script_sdk::Entry;

use crate::server::Server;
use crate::server::model::map::Map;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_item::MapItems;
use crate::server::model::quest_info::{Condition, QuestInfoEntry};
use crate::server::model::script::Script;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::entries::intern;
use crate::server::script::skill::actor::INVISIBLE_NPC_SPRITE;

const CLERIC: u32 = 610;
const GUARD: u32 = 611;
const MAIA_AREA: u32 = 612;

fn npc(id: u32, module: &str, name: &str) -> Script {
    Script {
        appearance: None, speed: None,
        id,
        scope_instance: 0,
        entry_id: intern(module, Entry::Npc(name.into())),
        map_name: "empty".into(),
        name: name.into(),
        sprite: 0,
        x: 40 + id as u16 % 10,
        y: 50,
        dir: 0,
        x_size: 0,
        y_size: 0,
        constructor_args: vec![],
    }
}

/// The acolyte test giver of `quest_offers.json`, an NPC that offers nothing and the touch area of a giver, on the
/// character's map.
fn fixture() -> (Server, u32) {
    let (context, _repository, character) = super::native_payment_tests::fixture(false, false);
    let touch_area = Script { sprite: INVISIBLE_NPC_SPRITE, x_size: 3, y_size: 3, ..npc(MAIA_AREA, "jobs_misc", "Maia#link2") };
    let scripts = vec![npc(CLERIC, "jobs_pre_re_jobs_s_1_1", "Cleric#aco"), npc(GUARD, "jobs_pre_re_jobs_s_1_1", "Guard#aco"), touch_area];
    let map = Box::leak(Box::new(Map::new(100, 100, 10_000, "empty".into(), "empty.gat".into(), vec![], vec![], scripts)));
    let cells = vec![models::enums::cell::CellType::Walkable.as_flag(); 10_000];
    let instance = MapInstance::from_map(crate::tests::common::test_npc_vm(), map, 0, cells, context.client_notification_sender.clone(), MapItems::new(0), Arc::new(TasksQueue::new()));
    context.server.state_mut().map_instances_mut().insert("empty".into(), vec![Arc::new(instance)]);
    let char_id = character.char_id;
    context.server.state_mut().insert_character(character);
    (context.server, char_id)
}

fn icons(server: &Server, char_id: u32) -> Vec<(u32, (u16, u8))> {
    let state = server.state();
    let mut icons: Vec<_> = state.get_character(char_id).unwrap().game_systems.quest_icons.iter().map(|(npc, icon)| (*npc, *icon)).collect();
    icons.sort();
    icons
}

#[test]
fn a_quest_giver_shows_its_mark_until_the_player_takes_one_of_its_quests() {
    let (server, char_id) = fixture();
    server.quest_login(&mut server.state_mut(), char_id);
    assert_eq!(icons(&server, char_id), [(CLERIC, (1, 1))], "a yellow \"!\" with a colour the client does not read as removal, none on a touch area");

    server.quest_add(&mut server.state_mut(), char_id, 1002).unwrap();
    assert_eq!(icons(&server, char_id), [], "1001 and 1003 are the other versions of the same test");

    server.quest_complete(&mut server.state_mut(), char_id, 1002).unwrap();
    assert_eq!(icons(&server, char_id), [], "a completed quest still counts as taken");

    server.quest_erase(&mut server.state_mut(), char_id, 1002).unwrap();
    assert_eq!(icons(&server, char_id), [(CLERIC, (1, 1))]);
}

#[test]
fn the_questinfo_of_a_script_decides_for_its_npc() {
    let (server, char_id) = fixture();
    let never = QuestInfoEntry { icon: 6, color: 2, condition: Some(Condition::parse("BaseLevel > 999").unwrap()) };
    server.state_mut().quest_infos.register(("empty.gat".into(), 0, CLERIC), never);
    server.quest_login(&mut server.state_mut(), char_id);
    assert_eq!(icons(&server, char_id), []);
}
