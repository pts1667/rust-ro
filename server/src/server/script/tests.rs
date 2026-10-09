use std::sync::Arc;

use models::enums::bonus::BonusType;
use models::status::Status;
use script_runtime::{Host, WasmRuntime};
use script_sdk::{Function, Reply, Request, Value};

use super::item_script_handler::{ItemEffect, ItemScriptHost};

fn runtime() -> Arc<WasmRuntime> {
    crate::tests::common::test_systems_runtime()
}

/// The `systems` script for the numeric id the hand-written NPCs and battleground events were first registered under.
fn system_entry(kind: &str, id: u32) -> script_sdk::Entry {
    const NPCS: [&str; 19] = [
        "counter", "variables", "warper", "stylist", "job_master", "shop", "mount_master", "castle_steward", "castle_lever", "castle_kafra",
        "wedding_staff", "wedding_bishop", "wedding_divorce", "breeder", "battleground_arena", "battleground_kvm", "battleground_tierra",
        "battleground_recruiter", "castle_flag",
    ];
    match kind {
        "run_npc" => script_sdk::Entry::Npc(NPCS[id as usize - 1].into()),
        _ => {
            let (family, base) = match id {
                1000..=1999 => ("arena", 1000),
                2000..=2999 => ("kvm", 2000),
                _ => ("tierra", 3000),
            };
            script_sdk::Entry::Event(format!("{family}_{}_{}", (id - base) / 100, (id - base) % 100))
        }
    }
}

fn item_vm() -> Arc<crate::server::script::ItemVm> {
    crate::tests::common::test_item_vm()
}

#[test]
fn compiled_potion_stages_healing_without_mutating_character_status() {
    let status = Status {
        hp: 10,
        sp: 4,
        ..Status::default()
    };
    let (host, result) = futures::executor::block_on(item_vm().run_item(ItemScriptHost::consumable(status, 501), 501));
    assert!(result.is_ok(), "{:?}", host.error);
    assert_eq!(host.status.hp, 10);
    assert_eq!(host.status.sp, 4);
    assert!(matches!(host.effects.as_slice(), [ItemEffect::Heal {
        hp: 45..=65,
        sp: 0,
        percentage: false,
        item_scaling: true,
        item_id: 501
    }]));
}

#[test]
fn equipment_context_rejects_consumable_healing_effects() {
    let (host, result) = futures::executor::block_on(item_vm().run_item(
        ItemScriptHost::bonuses(
            Status {
                hp: 10,
                ..Status::default()
            },
            526,
        ),
        526,
    ));
    assert!(result.is_err());
    assert_eq!(host.status.hp, 10);
    assert!(host.error.unwrap().contains("ItemHeal"));
    assert!(host.effects.is_empty());
}

#[test]
fn generated_static_bonus_matches_original_item() {
    let item: serde_json::Value =
        serde_json::from_slice(&std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/items.json")).unwrap())
            .unwrap();
    let id = item["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["script"] == "bonus bStr,1;")
        .unwrap()["id"]
        .as_u64()
        .unwrap() as u32;
    let (host, result) = futures::executor::block_on(item_vm().run_item(ItemScriptHost::bonuses(Status::default(), id), id));
    assert!(result.is_ok(), "{:?}", host.error);
    assert_eq!(host.bonuses.drain(), vec![BonusType::Str(1)]);
}

#[test]
fn item_catalog_fingerprint_rejects_stale_bundle_metadata() {
    let bytes = std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/items.json")).unwrap();
    let bytes: Vec<_> = bytes.into_iter().filter(|byte| *byte != b'\r').collect();
    let digest = md5::compute(&bytes);
    let fingerprint = u64::from_le_bytes(digest.0[..8].try_into().unwrap());
    assert_eq!(item_vm().catalog_hash().unwrap(), fingerprint);
    let mut changed = bytes;
    changed.push(b' ');
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../target/stale-script-manifest-{nonce}.json"));
    std::fs::write(&path, changed).unwrap();
    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::server::service::item_service::ItemService::load_item_scripts(&mut vec![], item_vm(), &path)
    }));
    std::fs::remove_file(path).unwrap();
    assert!(rejected.is_err());
}

#[test]
fn compiled_catalog_accepts_windows_line_endings() {
    let source = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/items.json")).unwrap();
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../target/crlf-script-manifest-{nonce}.json"));
    std::fs::write(&path, source.replace("\r\n", "\n").replace('\n', "\r\n")).unwrap();
    let counts = crate::server::service::item_service::ItemService::load_item_scripts(&mut vec![], item_vm(), &path);
    std::fs::remove_file(path).unwrap();
    assert_eq!(counts, (0, 0));
}

#[test]
fn npc_manifest_preserves_enabled_placements() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/npcs.json");
    let scripts = crate::server::boot::script_loader::ScriptLoader::load_scripts(root.to_str().unwrap()).unwrap();
    let systems = |entry: &str| super::entries::system_npc(entry);
    assert_eq!(scripts.values().flatten().filter(|npc| super::entries::resolve(npc.entry_id).is_some_and(|script| script.module == "systems")).count(), 736);
    let prontera = &scripts["prontera"];
    assert!(prontera.iter().any(|npc| npc.name == "Job Master" && npc.entry_id == systems("job_master")));
    assert!(prontera.iter().any(|npc| npc.entry_id == systems("shop") && !npc.constructor_args.is_empty()));
    let gate = scripts["bat_a01"].iter().find(|npc| npc.name == "barri_warp_up#bat_a01_a").unwrap();
    assert_eq!((gate.x_size, gate.y_size), (7, 0));
}

#[test]
fn npc_dialogue_runs_compiled_code_across_player_wait() {
    struct Dialogue {
        messages: Vec<String>,
        waits: usize,
    }
    #[async_trait::async_trait]
    impl Host for Dialogue {
        async fn invoke(&mut self, request: Request) -> Reply {
            match request {
                Request::VariableRead { .. } => Ok(0.into()),
                Request::VariablesWrite(_) => Ok(Value::default()),
                Request::VariablesIncrement(_) => Ok(Value::Array(vec![1.into(), 1.into()])),
                Request::Call {
                    function: Function::Mes,
                    arguments,
                } => {
                    self.messages.extend(arguments.iter().map(Value::text));
                    Ok(Value::default())
                }
                Request::Call {
                    function: Function::Next, ..
                } => {
                    self.waits += 1;
                    tokio::task::yield_now().await;
                    Ok(Value::default())
                }
                Request::Call {
                    function: Function::Close, ..
                } => Ok(Value::default()),
                Request::ReportError(error) => Err(error),
                _ => Err("Unexpected dialogue call".into()),
            }
        }
    }
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let (host, result) = runtime.block_on(self::runtime().execute_named(
        Dialogue {
            messages: vec![],
            waits: 0,
        },
        &system_entry("run_npc", 1),
    ));
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(host.waits, 1);
    assert_eq!(host.messages, [
        "Npc counter variable: 1\nNPC instance counter variable: 1",
        "Close"
    ]);
}

mod battleground_arena {
    use std::collections::HashMap;

    use super::*;

    #[derive(Default)]
    struct ArenaHost {
        variables: HashMap<String, Value>,
        calls: Vec<(Function, Vec<Value>)>,
        arguments: Vec<Value>,
        mob_count: i32,
        team_members: i32,
        character_team: i32,
        carried_badges: i32,
        selection: i32,
        selections: Vec<i32>,
    }

    #[async_trait::async_trait]
    impl Host for ArenaHost {
        async fn invoke(&mut self, request: Request) -> Reply {
            match request {
                Request::Read(name) => Ok(self.variables.get(&name).cloned().unwrap_or_default()),
                Request::Write { name, value } => {
                    self.variables.insert(name, value);
                    Ok(Value::default())
                }
                Request::Arguments => Ok(Value::Array(self.arguments.clone())),
                Request::ReportError(error) => Err(error),
                Request::Call { function, arguments } => {
                    self.calls.push((function, arguments));
                    Ok(match function {
                        Function::MobCount => self.mob_count.into(),
                        Function::BgGetData => self.team_members.into(),
                        Function::GetCharacterId => self.character_team.into(),
                        Function::CountItem => self.carried_badges.into(),
                        Function::StrCharInfo => "Tester".into(),
                        Function::Select => if self.selections.is_empty() { self.selection } else { self.selections.remove(0) }.into(),
                        Function::GetItemName => "Item".into(),
                        _ => Value::default(),
                    })
                }
                other => Err(format!("Unexpected request {other:?}")),
            }
        }
    }

    fn arena_host() -> ArenaHost {
        let mut host = ArenaHost::default();
        host.variables.insert("$@FlaviusBG1_id1".into(), 7.into());
        host.variables.insert("$@FlaviusBG1_id2".into(), 8.into());
        host
    }

    fn run(host: ArenaHost, entry: &str, id: u32) -> ArenaHost {
        let (host, result) = tokio::runtime::Runtime::new().unwrap().block_on(runtime().execute_named(host, &system_entry(entry, id)));
        assert!(result.is_ok(), "{result:?}");
        host
    }

    fn called(host: &ArenaHost, function: Function) -> Vec<&Vec<Value>> {
        host.calls.iter().filter(|(candidate, _)| *candidate == function).map(|(_, arguments)| arguments).collect()
    }

    fn number(host: &ArenaHost, name: &str) -> i32 {
        host.variables.get(name).map_or(0, |value| value.number_value().unwrap())
    }

    #[test]
    fn the_ready_check_arms_the_arena_once_and_sends_both_teams_to_their_camps() {
        let host = run(arena_host(), "run_event", 1001);
        assert_eq!(number(&host, "$@FlaviusBG1"), 1);
        assert_eq!(called(&host, Function::BgMonster).len(), 6);
        assert_eq!(called(&host, Function::SetMobImmunity).len(), 2);
        let warps: Vec<_> = called(&host, Function::BgWarp).iter().map(|arguments| (arguments[0].number_value().unwrap(), arguments[2].number_value().unwrap())).collect();
        assert_eq!(warps, [(7, 87), (8, 311)]);
        assert!(called(&host, Function::BgUpdateScore).iter().all(|arguments| arguments[1..] == [0.into(), 0.into()]));

        let calls_before = host.calls.len();
        let host = run(host, "run_event", 1001);
        assert_eq!(host.calls.len(), calls_before);
    }

    #[test]
    fn the_first_crystal_resets_the_round_and_the_second_ends_the_match() {
        let mut host = arena_host();
        host.variables.insert("$@FlaviusBG1".into(), 1.into());
        let host = run(host, "run_event", 1010);
        assert_eq!((number(&host, "$@Croix_ScoreBG1"), number(&host, "$@FlaviusBG1_Victory")), (1, 0));
        assert_eq!(called(&host, Function::BgMonster).len(), 6);
        assert!(called(&host, Function::BgReserve).is_empty());

        let mut host = run(host, "run_event", 1010);
        assert_eq!((number(&host, "$@Croix_ScoreBG1"), number(&host, "$@FlaviusBG1_Victory")), (2, 2));
        assert_eq!(called(&host, Function::BgReserve).len(), 1);
        assert_eq!(called(&host, Function::EnableNpc).len(), 2 + 2);
        host.mob_count = 1;
        let calls_before = host.calls.len();
        let host = run(host, "run_event", 1010);
        assert_eq!(host.calls.len(), calls_before + 1);
    }

    #[test]
    fn slaying_the_guardians_opens_the_camp_and_exposes_the_crystal() {
        let host = run(arena_host(), "run_event", 1012);
        assert_eq!(called(&host, Function::SetCell).len(), 2);
        let immunity = called(&host, Function::SetMobImmunity);
        assert_eq!(immunity.len(), 1);
        assert_eq!(immunity[0][1], "OBJ#bat_b01_a::OnMyMobDead".into());
        assert_eq!(immunity[0][2], 0.into());
    }

    #[test]
    fn vintenars_hand_out_badges_to_their_own_team_only_up_to_the_carry_limit() {
        let mut host = arena_host();
        host.arguments = vec![0.into(), 2.into()];
        host.variables.insert("$@FlaviusBG1_Victory".into(), 1.into());
        host.character_team = 7;
        let winner = run(host, "run_npc", 15);
        assert_eq!(called(&winner, Function::GetItem), [&vec![7829.into(), 9.into()]]);
        assert_eq!(called(&winner, Function::BgLeave).len(), 1);

        let mut host = arena_host();
        host.arguments = vec![0.into(), 3.into()];
        host.variables.insert("$@FlaviusBG1_Victory".into(), 1.into());
        host.character_team = 8;
        host.carried_badges = 499;
        let loser = run(host, "run_npc", 15);
        assert_eq!(called(&loser, Function::GetItem), [&vec![7829.into(), 1.into()]]);

        let mut host = arena_host();
        host.arguments = vec![0.into(), 2.into()];
        host.character_team = 8;
        let outsider = run(host, "run_npc", 15);
        assert!(called(&outsider, Function::GetItem).is_empty());
        assert!(called(&outsider, Function::BgLeave).is_empty());
    }

    #[test]
    fn the_cleanup_poll_releases_the_arena_once_both_teams_are_empty() {
        let mut host = arena_host();
        host.variables.insert("$@FlaviusBG1".into(), 1.into());
        host.team_members = 3;
        let busy = run(host, "run_event", 1050);
        assert_eq!(number(&busy, "$@FlaviusBG1"), 1);
        assert!(called(&busy, Function::BgDestroy).is_empty());

        let mut host = arena_host();
        host.variables.insert("$@FlaviusBG1".into(), 1.into());
        let empty = run(host, "run_event", 1050);
        assert_eq!((number(&empty, "$@FlaviusBG1"), number(&empty, "$@FlaviusBG1_id1")), (0, 0));
        assert_eq!(called(&empty, Function::BgDestroy).len(), 2);
        assert_eq!(called(&empty, Function::BgUnbook).len(), 1);
    }

    fn tierra_host() -> ArenaHost {
        let mut host = ArenaHost::default();
        host.variables.insert("$@TierraBG1_id1".into(), 7.into());
        host.variables.insert("$@TierraBG1_id2".into(), 8.into());
        host
    }

    #[test]
    fn tierra_ready_check_spawns_both_objectives_the_barricades_and_the_neutral_flag_once() {
        let host = run(tierra_host(), "run_event", 3001);
        assert_eq!(number(&host, "$@TierraBG1"), 1);
        assert_eq!(called(&host, Function::BgMonster).len(), 2 + 34);
        assert_eq!(called(&host, Function::Monster).len(), 1);
        assert!(called(&host, Function::SetCell).iter().all(|arguments| arguments[0] == "bat_a01".into()));
        let warps: Vec<_> = called(&host, Function::BgWarp).iter().map(|arguments| arguments[2].number_value().unwrap()).collect();
        assert_eq!(warps, [352, 353]);
        let calls_before = host.calls.len();
        assert_eq!(run(host, "run_event", 3001).calls.len(), calls_before);
    }

    #[test]
    fn tierra_destroying_a_food_store_hands_the_other_team_the_victory() {
        let host = run(tierra_host(), "run_event", 3010);
        assert_eq!(number(&host, "$@TierraBG1_Victory"), 2);
        assert_eq!(called(&host, Function::EnableNpc).len(), 2);
        assert_eq!(called(&host, Function::BgReserve).len(), 1);
        let host = run(tierra_host(), "run_event", 3011);
        assert_eq!(number(&host, "$@TierraBG1_Victory"), 1);
    }

    #[test]
    fn tierra_a_broken_barricade_opens_the_wall_and_calls_the_blacksmith() {
        let mut host = tierra_host();
        host.mob_count = 16;
        let host = run(host, "run_event", 3014);
        assert_eq!(called(&host, Function::EnableNpc), [&vec!["Guillaume Blacksmith#a01".into()]]);
        let wall = called(&host, Function::SetCell);
        assert_eq!(wall.len(), 1);
        assert_eq!((wall[0][1].clone(), wall[0][3].clone(), wall[0][6].clone()), (186.into(), 201.into(), 1.into()));

        let mut host = tierra_host();
        host.mob_count = 17;
        assert!(run(host, "run_event", 3014).calls.iter().all(|(function, _)| *function == Function::MobCount));
    }

    #[test]
    fn tierra_neutral_flag_goes_to_the_capturing_team_with_guardians_and_a_new_respawn() {
        let mut host = tierra_host();
        host.character_team = 8;
        let host = run(host, "run_event", 3012);
        assert_eq!(called(&host, Function::BgTeamSetXy), [&vec![8.into(), 56.into(), 212.into()]]);
        let guardians = called(&host, Function::BgMonster);
        assert_eq!(guardians.len(), 3);
        assert!(guardians.iter().all(|arguments| arguments[0] == 8.into() && arguments[4] == "Croix Camp Guardian".into()));

        let mut host = tierra_host();
        host.character_team = 99;
        assert!(called(&run(host, "run_event", 3012), Function::BgMonster).is_empty());
    }

    #[test]
    fn tierra_blacksmith_repairs_the_barricade_for_fifty_stones_and_only_for_the_own_team() {
        let mut host = tierra_host();
        host.arguments = vec![0.into(), 5.into()];
        host.character_team = 7;
        host.carried_badges = 60;
        host.selection = 1;
        let repaired = run(host, "run_npc", 17);
        assert_eq!(called(&repaired, Function::DelItem), [&vec![7049.into(), 50.into()]]);
        assert_eq!(called(&repaired, Function::BgMonster).len(), 17);

        let mut host = tierra_host();
        host.arguments = vec![0.into(), 5.into()];
        host.character_team = 7;
        host.carried_badges = 10;
        host.selection = 1;
        assert!(called(&run(host, "run_npc", 17), Function::DelItem).is_empty());

        let mut host = tierra_host();
        host.arguments = vec![0.into(), 5.into()];
        host.character_team = 8;
        host.carried_badges = 60;
        assert!(called(&run(host, "run_npc", 17), Function::BgMonster).is_empty());
    }

    #[test]
    fn tierra_vintenars_pay_bravery_badges() {
        let mut host = tierra_host();
        host.arguments = vec![0.into(), 3.into()];
        host.variables.insert("$@TierraBG1_Victory".into(), 1.into());
        host.character_team = 7;
        assert_eq!(called(&run(host, "run_npc", 17), Function::GetItem), [&vec![7828.into(), 3.into()]]);

        let mut host = tierra_host();
        host.arguments = vec![0.into(), 3.into()];
        host.variables.insert("$@TierraBG1_Victory".into(), 2.into());
        host.character_team = 7;
        assert_eq!(called(&run(host, "run_npc", 17), Function::GetItem), [&vec![7828.into(), 1.into()]]);
    }

    #[test]
    fn maroll_recruiters_remember_the_return_town_and_the_teleporter_sends_the_player_back() {
        let mut host = ArenaHost::default();
        host.arguments = vec![0.into(), 4.into()];
        host.selection = 1;
        let host = run(host, "run_npc", 18);
        assert_eq!(number(&host, "bat_return"), 4);
        assert_eq!(called(&host, Function::Warp), [&vec!["bat_room".into(), 154.into(), 150.into()]]);

        let mut host = ArenaHost::default();
        host.arguments = vec![0.into(), 4.into()];
        host.selection = 2;
        assert!(called(&run(host, "run_npc", 18), Function::Warp).is_empty());

        let mut host = ArenaHost::default();
        host.arguments = vec![1.into(), 0.into()];
        host.selection = 1;
        host.variables.insert("bat_return".into(), 5.into());
        assert_eq!(called(&run(host, "run_npc", 18), Function::Warp), [&vec!["payon".into(), 161.into(), 58.into()]]);
    }

    #[test]
    fn erundek_exchanges_weapons_for_a_hundred_badges_and_armor_for_the_chosen_badge() {
        let mut host = ArenaHost::default();
        host.arguments = vec![4.into(), 0.into()];
        host.selections = vec![1, 1, 1, 2, 2, 1];
        host.carried_badges = 100;
        let weapon = run(host, "run_npc", 18);
        assert_eq!(called(&weapon, Function::DelItem), [&vec![7829.into(), 100.into()]]);
        assert_eq!(called(&weapon, Function::GetItem), [&vec![13037.into(), 1.into()]]);

        let mut host = ArenaHost::default();
        host.arguments = vec![4.into(), 0.into()];
        host.selections = vec![1, 2, 2, 1, 2, 1];
        host.carried_badges = 79;
        let short = run(host, "run_npc", 18);
        assert!(called(&short, Function::GetItem).is_empty());

        let mut host = ArenaHost::default();
        host.arguments = vec![4.into(), 0.into()];
        host.selections = vec![1, 2, 2, 1, 2, 2];
        host.carried_badges = 80;
        let armor = run(host, "run_npc", 18);
        assert_eq!(called(&armor, Function::DelItem), [&vec![7829.into(), 80.into()]]);
        assert_eq!(called(&armor, Function::GetItem), [&vec![2376.into(), 1.into()]]);
    }

    fn kvm_host() -> ArenaHost {
        let mut host = ArenaHost::default();
        host.variables.insert("$@KvM01BG_id1".into(), 7.into());
        host.variables.insert("$@KvM01BG_id2".into(), 8.into());
        host
    }

    #[test]
    fn kvm_starts_the_fight_with_the_team_sizes_and_ends_when_a_side_is_wiped_out() {
        let mut host = kvm_host();
        host.team_members = 5;
        let host = run(host, "run_event", 2018);
        assert_eq!((number(&host, "$@KvM01BG"), number(&host, "$@KvM01_Guillaume_Count"), number(&host, "$@KvM01_Croix_Count")), (2, 5, 5));
        let warps: Vec<_> = called(&host, Function::BgWarp).iter().map(|arguments| arguments[2].number_value().unwrap()).collect();
        assert_eq!(warps, [61, 138]);

        let mut host = kvm_host();
        host.variables.insert("$@KvM01BG".into(), 2.into());
        host.variables.insert("$@KvM01_Guillaume_Count".into(), 1.into());
        host.variables.insert("$@KvM01_Croix_Count".into(), 4.into());
        let host = run(host, "run_event", 2001);
        assert_eq!((number(&host, "$@KvM01BG"), number(&host, "$@KvM01BG_Victory")), (3, 2));
        assert_eq!(called(&host, Function::BgReserve).len(), 1);
        assert_eq!(called(&host, Function::EnableNpc).len(), 2);

        let mut host = kvm_host();
        host.variables.insert("$@KvM01BG".into(), 2.into());
        host.variables.insert("$@KvM01_Croix_Count".into(), 4.into());
        host.variables.insert("$@KvM01_Guillaume_Count".into(), 3.into());
        let host = run(host, "run_event", 2002);
        assert_eq!((number(&host, "$@KvM01BG"), number(&host, "$@KvM01_Croix_Count")), (2, 3));
        assert!(called(&host, Function::EnableNpc).is_empty());

        let mut host = kvm_host();
        host.variables.insert("$@KvM01_Croix_Count".into(), 4.into());
        let idle = run(host, "run_event", 2002);
        assert_eq!(number(&idle, "$@KvM01_Croix_Count"), 4);
    }

    #[test]
    fn kvm_time_limit_awards_the_larger_side_or_declares_a_draw() {
        let mut host = kvm_host();
        host.variables.insert("$@KvM01_Guillaume_Count".into(), 4.into());
        host.variables.insert("$@KvM01_Croix_Count".into(), 2.into());
        assert_eq!(number(&run(host, "run_event", 2024), "$@KvM01BG_Victory"), 1);

        let mut host = kvm_host();
        host.variables.insert("$@KvM01_Guillaume_Count".into(), 3.into());
        host.variables.insert("$@KvM01_Croix_Count".into(), 3.into());
        assert_eq!(number(&run(host, "run_event", 2024), "$@KvM01BG_Victory"), 3);
    }

    #[test]
    fn kvm_officers_pay_winning_or_losing_points_to_the_battle_participants_only() {
        let mut host = kvm_host();
        host.arguments = vec![0.into(), 1.into()];
        host.variables.insert("$@KvM01BG_Victory".into(), 1.into());
        host.variables.insert("kvm_point".into(), 10.into());
        host.character_team = 7;
        let winner = run(host, "run_npc", 16);
        assert_eq!((number(&winner, "kvm_point"), called(&winner, Function::BgLeave).len()), (15, 1));

        let mut host = kvm_host();
        host.arguments = vec![0.into(), 2.into()];
        host.variables.insert("$@KvM01BG_Victory".into(), 1.into());
        host.character_team = 8;
        let loser = run(host, "run_npc", 16);
        assert_eq!(number(&loser, "kvm_point"), 1);

        let mut host = kvm_host();
        host.arguments = vec![0.into(), 1.into()];
        host.variables.insert("$@KvM01BG_Victory".into(), 1.into());
        host.character_team = 99;
        let outsider = run(host, "run_npc", 16);
        assert!(called(&outsider, Function::BgLeave).is_empty());
    }
}
