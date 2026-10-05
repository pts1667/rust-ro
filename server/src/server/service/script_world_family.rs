use models::enums::EnumWithNumberValue;
use models::enums::class::JobName;
use script_sdk::Value;

use super::ScriptWorldService;
use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{GameEvent, ScriptWarp};
use crate::server::model::map_flags::MapFlag;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const WE_BABY: i32 = 408;
const WE_CALLPARENT: i32 = 409;
const WE_CALLBABY: i32 = 410;
const PERMANENT_GRANT: i32 = 3;
const WEDDING_RINGS: [i32; 2] = [2634, 2635];
const MINIMUM_PARENT_LEVEL: u32 = 70;
const REPLY_MORE_CHILDREN: u32 = 0;
const REPLY_LEVEL_70: u32 = 1;
const REPLY_MARRIED: u32 = 2;

enum Refusal {
    Silent,
    Reply(u32),
}

fn baby_job(job: JobName) -> Option<JobName> {
    Some(match job {
        JobName::Novice => JobName::BabyNovice,
        JobName::Swordsman => JobName::BabySwordsman,
        JobName::Mage => JobName::BabyMage,
        JobName::Archer => JobName::BabyArcher,
        JobName::Acolyte => JobName::BabyAcolyte,
        JobName::Merchant => JobName::BabyMerchant,
        JobName::Thief => JobName::BabyThief,
        JobName::SuperNovice => JobName::SuperBaby,
        _ => return None,
    })
}

fn ring_equipped(character: &Character) -> bool {
    character
        .inventory_iter()
        .any(|(_, item)| item.equip != 0 && WEDDING_RINGS.contains(&item.item_id))
}

fn check_adoption(father: &Character, mother: &Character, baby: &Character) -> Result<JobName, Refusal> {
    if baby.game_systems.father_id != 0 || baby.game_systems.mother_id != 0 || baby.game_systems.adopt_invite != 0 {
        return Err(Refusal::Silent);
    }
    let party = father.game_systems.party_id;
    if father.game_systems.partner_id == 0 || party == 0 || party != baby.game_systems.party_id || party != mother.game_systems.party_id {
        return Err(Refusal::Silent);
    }
    if father.game_systems.partner_id != mother.char_id || mother.game_systems.partner_id != father.char_id {
        return Err(Refusal::Silent);
    }
    if !ring_equipped(father) || !ring_equipped(mother) {
        return Err(Refusal::Silent);
    }
    if father.game_systems.child_id != 0 || mother.game_systems.child_id != 0 {
        return Err(Refusal::Reply(REPLY_MORE_CHILDREN));
    }
    if father.status.base_level < MINIMUM_PARENT_LEVEL || mother.status.base_level < MINIMUM_PARENT_LEVEL {
        return Err(Refusal::Reply(REPLY_LEVEL_70));
    }
    if baby.game_systems.partner_id != 0 {
        return Err(Refusal::Reply(REPLY_MARRIED));
    }
    baby_job(JobName::from_value(baby.status.job as usize)).ok_or(Refusal::Silent)
}

impl ScriptWorldService {
    fn adoption_reply(&self, char_id: u32, code: u32) {
        let mut packet = 0x0216_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&code.to_le_bytes());
        let _ = self.notifications.try_send(Notification::Char(CharNotification::new(char_id, packet)));
    }

    pub(crate) fn request_adoption(&self, state: &mut ServerState, parent: &Character, baby_account: u32) -> Result<(), String> {
        let partner_id = parent.game_systems.partner_id;
        let baby_id = state
            .characters()
            .values()
            .find(|candidate| candidate.account_id == baby_account)
            .map(|candidate| candidate.char_id)
            .ok_or("The adoption candidate is not online")?;
        let (Some(partner), Some(baby)) = (state.characters().get(&partner_id), state.characters().get(&baby_id)) else {
            return Err("Both parents and the child must be online".into());
        };
        match check_adoption(parent, partner, baby) {
            Ok(_) => {}
            Err(Refusal::Reply(code)) => {
                self.adoption_reply(parent.char_id, code);
                return Ok(());
            }
            Err(Refusal::Silent) => return Ok(()),
        }
        let mut packet = 0x01F6_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&parent.account_id.to_le_bytes());
        packet.extend_from_slice(&partner.account_id.to_le_bytes());
        let mut name = [0_u8; 24];
        let bytes = parent.name.as_bytes();
        let length = bytes.len().min(23);
        name[..length].copy_from_slice(&bytes[..length]);
        packet.extend_from_slice(&name);
        let _ = self.notifications.try_send(Notification::Char(CharNotification::new(baby_id, packet)));
        if let Some(baby) = state.characters_mut().get_mut(&baby_id) {
            baby.game_systems.adopt_invite = parent.account_id;
        }
        Ok(())
    }

    pub(crate) fn answer_adoption(
        &self,
        server: &Server,
        state: &mut ServerState,
        baby: &mut Character,
        father_account: u32,
        mother_account: u32,
        accept: bool,
    ) -> Result<(), String> {
        let invited_by = std::mem::take(&mut baby.game_systems.adopt_invite);
        if !accept || invited_by != father_account {
            return Ok(());
        }
        let find = |state: &ServerState, account: u32| {
            state
                .characters()
                .values()
                .find(|candidate| candidate.account_id == account)
                .map(|candidate| candidate.char_id)
        };
        let (Some(father_id), Some(mother_id)) = (find(state, father_account), find(state, mother_account)) else {
            return Ok(());
        };
        let (Some(mut father), Some(mut mother)) = (state.characters_mut().remove(&father_id), state.characters_mut().remove(&mother_id)) else {
            return Err("Adoptive parents are unavailable".into());
        };
        let result = self.adopt(server, &mut father, &mut mother, baby);
        state.insert_character(father);
        state.insert_character(mother);
        result
    }

    fn adopt(&self, server: &Server, father: &mut Character, mother: &mut Character, baby: &mut Character) -> Result<(), String> {
        let job = match check_adoption(father, mother, baby) {
            Ok(job) => job,
            Err(Refusal::Reply(code)) => {
                self.adoption_reply(father.char_id, code);
                return Ok(());
            }
            Err(Refusal::Silent) => return Ok(()),
        };
        self.repository
            .adopt_character(father.char_id, mother.char_id, baby.char_id)
            .map_err(|error| error.to_string())?;
        father.game_systems.child_id = baby.char_id;
        mother.game_systems.child_id = baby.char_id;
        baby.game_systems.father_id = father.char_id;
        baby.game_systems.mother_id = mother.char_id;
        for member in [&mut *father, &mut *mother, &mut *baby] {
            member.game_systems.revision = self
                .repository
                .character_game_systems(member.char_id)
                .map_err(|error| error.to_string())?
                .revision;
        }
        server.character_service().change_job_keeping_level(baby, job);
        for skill in [WE_BABY, WE_CALLPARENT] {
            crate::server::service::script_character_service::grant_skill(
                server,
                baby,
                &[Value::Number(skill), Value::Number(1), Value::Number(PERMANENT_GRANT)],
            )?;
        }
        for parent in [father, mother] {
            crate::server::service::script_character_service::grant_skill(
                server,
                parent,
                &[Value::Number(WE_CALLBABY), Value::Number(1), Value::Number(PERMANENT_GRANT)],
            )?;
        }
        Ok(())
    }

    /// Warps the caster's baby (married parents only) or both parents next to the caster.
    pub(crate) fn call_family(&self, server: &Server, state: &ServerState, character: &Character, parents: bool) -> Result<(), String> {
        let targets: Vec<u32> = if parents {
            [character.game_systems.father_id, character.game_systems.mother_id]
                .into_iter()
                .filter(|id| *id != 0)
                .collect()
        } else if character.game_systems.partner_id != 0 && character.game_systems.child_id != 0 {
            vec![character.game_systems.child_id]
        } else {
            Vec::new()
        };
        if state.map_flags(&character.map_instance_key).enabled(MapFlag::NoWarpTo) {
            return Err("Family members cannot be called to this map".into());
        }
        let mut called = 0;
        for id in targets {
            let Some(member) = state.characters().get(&id).filter(|member| member.status.hp > 0 && !member.is_dead()) else {
                continue;
            };
            let origin = state.map_flags(&member.map_instance_key);
            if origin.enabled(MapFlag::NoWarp) || origin.enabled(MapFlag::NoTeleport) {
                continue;
            }
            server.add_to_next_tick(GameEvent::ScriptWarp(ScriptWarp {
                char_id: member.char_id,
                map: character.current_map_name().clone(),
                x: character.x,
                y: character.y,
                destination_instance: Some(character.current_map_instance()),
            }));
            called += 1;
        }
        if called == 0 {
            return Err("No family member can be called".into());
        }
        Ok(())
    }
}
