use database::model::CharacterRecord;
use models::enums::skill::UseSkillFailure;

use super::{ScriptWorldService, install_state, protocol};
use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, ScriptWorld};
use crate::server::model::game_systems::{PartyChange, PartyInvitation, PartyRecord, ScriptWorldRequest};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const PARTY_SHARE_LEVEL: u32 = 15;

fn cached_party(character: &Character) -> Option<&PartyRecord> {
    character
        .game_systems
        .party
        .as_ref()
        .filter(|party| party.id == character.game_systems.party_id && party.members.contains(&character.char_id))
}

fn live_member<'a>(state: &'a ServerState, source: &'a Character, id: u32) -> Option<&'a Character> {
    let member = if id == source.char_id {
        Some(source)
    } else {
        state.characters().get(&id)
    }?;
    member.loaded_from_client_side.then_some(member)
}

fn share_level_allowed(state: &ServerState, source: &Character, party: &PartyRecord) -> bool {
    let mut levels = party
        .members
        .iter()
        .filter_map(|id| live_member(state, source, *id))
        .map(|member| member.status.base_level);
    let Some(first) = levels.next() else {
        return true;
    };
    let (min, max) = levels.fold((first, first), |(min, max), level| (min.min(level), max.max(level)));
    max.saturating_sub(min) <= PARTY_SHARE_LEVEL
}

pub fn party_experience_awards(
    state: &ServerState,
    owner: &Character,
    base: u32,
    job: u32,
    map: &str,
    instance: u8,
) -> Vec<(u32, u32, u32)> {
    party_experience_awards_with_bonus(state, owner, base, job, map, instance, 0)
}

pub fn party_experience_awards_with_bonus(
    state: &ServerState,
    owner: &Character,
    base: u32,
    job: u32,
    map: &str,
    instance: u8,
    per_additional_member_bonus: u16,
) -> Vec<(u32, u32, u32)> {
    let Some(party) = cached_party(owner).filter(|party| party.exp_share && share_level_allowed(state, owner, party)) else {
        return vec![(owner.char_id, base, job)];
    };
    let eligible: Vec<_> = party
        .members
        .iter()
        .filter_map(|id| live_member(state, owner, *id))
        .filter(|member| {
            member.game_systems.party_id == party.id
                && member.status.hp > 0
                && member
                    .current_map_name()
                    .trim_end_matches(".gat")
                    .eq_ignore_ascii_case(map.trim_end_matches(".gat"))
                && member.current_map_instance() == instance
        })
        .map(|member| member.char_id)
        .collect();
    let count = eligible.len() as u32;
    if count == 0 {
        return Vec::new();
    }
    let rate = 100u64 + u64::from(per_additional_member_bonus) * u64::from(count - 1);
    let share = |experience: u32| (u64::from(experience / count) * rate / 100).min(u64::from(u32::MAX)) as u32;
    eligible.into_iter().map(|id| (id, share(base), share(job))).collect()
}

pub fn party_can_pick_up(picker: &Character, owner_id: u32) -> bool {
    picker.char_id == owner_id || cached_party(picker).is_some_and(|party| party.item_pickup && party.members.contains(&owner_id))
}

pub fn party_loot_candidates(state: &ServerState, picker: &Character) -> Vec<u32> {
    let Some(party) = cached_party(picker).filter(|party| party.item_share) else {
        return vec![picker.char_id];
    };
    let mut eligible: Vec<_> = party
        .members
        .iter()
        .filter_map(|id| live_member(state, picker, *id))
        .filter(|member| {
            member.game_systems.party_id == party.id
                && member.status.hp > 0
                && member.current_map_name() == picker.current_map_name()
                && member.current_map_instance() == picker.current_map_instance()
        })
        .map(|member| member.char_id)
        .collect();
    if !eligible.contains(&picker.char_id) {
        eligible.push(picker.char_id);
    }
    eligible
}

fn options_packet(party: &PartyRecord, denied: bool) -> Vec<u8> {
    let mut packet = protocol::header(0x07D8);
    packet.extend_from_slice(&(if denied { 2u32 } else { u32::from(party.exp_share) }).to_le_bytes());
    packet.extend_from_slice(&[u8::from(party.item_pickup), u8::from(party.item_share)]);
    packet
}

fn invitation_result(name: &str, result: u32) -> Vec<u8> {
    let mut packet = protocol::header(0x02C5);
    protocol::fixed_string(&mut packet, name, 24);
    packet.extend_from_slice(&result.to_le_bytes());
    packet
}

fn withdrawal(record: &CharacterRecord, expelled: bool) -> Vec<u8> {
    let mut packet = protocol::header(0x0105);
    packet.extend_from_slice(&(record.account_id as u32).to_le_bytes());
    protocol::fixed_string(&mut packet, &record.name, 24);
    packet.push(u8::from(expelled));
    packet
}

fn position_packet(account_id: u32, x: u16, y: u16) -> Vec<u8> {
    let mut packet = protocol::header(0x0107);
    packet.extend_from_slice(&account_id.to_le_bytes());
    packet.extend_from_slice(&x.to_le_bytes());
    packet.extend_from_slice(&y.to_le_bytes());
    packet
}

fn health_packet(account_id: u32, hp: u32, max_hp: u32) -> Vec<u8> {
    let mut packet = protocol::header(0x080E);
    packet.extend_from_slice(&account_id.to_le_bytes());
    packet.extend_from_slice(&hp.to_le_bytes());
    packet.extend_from_slice(&max_hp.to_le_bytes());
    packet
}

fn map_name(map: &str) -> String {
    if map.ends_with(".gat") {
        map.to_string()
    } else {
        format!("{map}.gat")
    }
}

fn names_packet(character: &Character) -> Vec<u8> {
    let mut packet = protocol::header(0x0195);
    packet.extend_from_slice(&character.char_id.to_le_bytes());
    for text in [
        character.name.as_str(),
        character.game_systems.party_name.as_str(),
        character.guild_name.as_str(),
        "",
    ] {
        protocol::fixed_string(&mut packet, text, 24);
    }
    packet
}

fn roster_packet(party: &PartyRecord, roster: &[(CharacterRecord, bool)]) -> Vec<u8> {
    let mut packet = protocol::header(0x00FB);
    packet.extend_from_slice(&(28u16 + roster.len() as u16 * 46).to_le_bytes());
    protocol::fixed_string(&mut packet, &party.name, 24);
    for (member, online) in roster {
        packet.extend_from_slice(&(member.account_id as u32).to_le_bytes());
        protocol::fixed_string(&mut packet, &member.name, 24);
        protocol::fixed_string(&mut packet, &map_name(&member.last_map), 16);
        packet.push(u8::from(member.char_id as u32 != party.leader_char_id));
        packet.push(u8::from(!online));
    }
    packet
}

fn member_packet(party: &PartyRecord, member: &Character) -> Vec<u8> {
    let mut packet = protocol::header(0x01E9);
    packet.extend_from_slice(&member.account_id.to_le_bytes());
    packet.extend_from_slice(&u32::from(member.char_id != party.leader_char_id).to_le_bytes());
    packet.extend_from_slice(&member.x.to_le_bytes());
    packet.extend_from_slice(&member.y.to_le_bytes());
    packet.push(0);
    protocol::fixed_string(&mut packet, &party.name, 24);
    protocol::fixed_string(&mut packet, &member.name, 24);
    protocol::fixed_string(&mut packet, &map_name(member.current_map_name()), 16);
    packet.extend_from_slice(&[u8::from(party.item_pickup), u8::from(party.item_share)]);
    packet
}

fn roster(state: &ServerState, source: &Character, records: &[CharacterRecord], offline: Option<u32>) -> Vec<(CharacterRecord, bool)> {
    records
        .iter()
        .map(|record| {
            let mut record = record.clone();
            let live = if record.char_id as u32 == source.char_id {
                Some(source)
            } else {
                state.characters().get(&(record.char_id as u32))
            }
            .filter(|member| Some(member.char_id) != offline);
            if let Some(member) = live {
                record.name = member.name.clone();
                record.last_map = member.current_map_name().clone();
                record.last_x = member.x as i16;
                record.last_y = member.y as i16;
                record.base_level = member.status.base_level.min(i32::MAX as u32) as i32;
            }
            (record, live.is_some())
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartyRequest {
    CreateParty {
            name: String,
            item_pickup: bool,
            item_share: bool,
        },
    InviteParty(u32),
    InvitePartyByName(String),
    AnswerPartyInvite {
            party_id: u32,
            accept: bool,
        },
    LeaveParty,
    ExpelParty {
            account_id: u32,
            name: String,
        },
    ChangePartyLeader(u32),
    ChangePartyOptions {
            exp_share: bool,
            item_rules: Option<(bool, bool)>,
        },
    DisablePartyInvites(bool),
    PartyMessage(String),
    RefreshParty,
}


impl ScriptWorldService {
    pub fn initialize_party(&self, server: &Server, character: &mut Character) -> Result<(), String> {
        self.send(character.char_id, vec![
            0xC9,
            0x02,
            u8::from(character.game_systems.party_invite_disabled),
        ])?;
        if character.game_systems.party_id == 0 {
            return Ok(());
        }
        let party = self
            .repository
            .party(character.game_systems.party_id)
            .map_err(|error| error.to_string())?
            .filter(|party| party.members.contains(&character.char_id))
            .ok_or("Character party no longer exists")?;
        character.game_systems.party = Some(party.clone());
        let records = self.repository.party_member_records(party.id).map_err(|error| error.to_string())?;
        let state = server.state();
        let packet = roster_packet(&party, &roster(state, character, &records, None));
        for member in &party.members {
            self.send(*member, packet.clone())?;
            self.send(*member, options_packet(&party, false))?;
            self.send(*member, member_packet(&party, character))?;
            if let Some(other) = live_member(state, character, *member) {
                if other.char_id != character.char_id
                    && other.current_map_name() == character.current_map_name()
                    && other.current_map_instance() == character.current_map_instance()
                {
                    self.send(character.char_id, position_packet(other.account_id, other.x, other.y))?;
                    self.send(
                        character.char_id,
                        health_packet(
                            other.account_id,
                            other.status.hp,
                            crate::server::service::status_service::StatusService::instance()
                                .to_snapshot(&other.status)
                                .max_hp(),
                        ),
                    )?;
                }
            }
        }
        server.add_to_next_tick(GameEvent::ScriptWorld(ScriptWorld {
            char_id: character.char_id,
            request: ScriptWorldRequest::Party(PartyRequest::RefreshParty),
        }));
        self.area(character, names_packet(character))
    }

    pub fn disconnect_party(&self, server: &Server, character: &mut Character) -> Result<(), String> {
        character.game_systems.party_invitation = None;
        if let Some(party) = cached_party(character) {
            let records = self.repository.party_member_records(party.id).map_err(|error| error.to_string())?;
            let state = server.state();
            let packet = roster_packet(party, &roster(state, character, &records, Some(character.char_id)));
            for member in &party.members {
                if *member != character.char_id {
                    self.send(*member, packet.clone())?;
                    self.send(*member, position_packet(character.account_id, u16::MAX, u16::MAX))?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn party_request(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        request: PartyRequest,
    ) -> Result<(), String> {
        match request {
            PartyRequest::CreateParty {
                name,
                item_pickup,
                item_share,
            } => {
                if character
                    .status
                    .known_skills
                    .iter()
                    .find(|skill| skill.value.id() == 1)
                    .is_none_or(|skill| skill.level < 7)
                {
                    return self.send(character.char_id, protocol::skill_failed(1, UseSkillFailure::Level));
                }
                if character.game_systems.party_id != 0 {
                    return self.send(character.char_id, vec![0xFA, 0, 2]);
                }
                let change = match self.repository.create_party(character.char_id, name, item_pickup, item_share) {
                    Ok(change) => change,
                    Err(error) => {
                        self.send(character.char_id, vec![0xFA, 0, 1])?;
                        return Err(error.to_string());
                    }
                };
                character.game_systems.party_invitation = None;
                self.install_party_change(server, state, character, change, false)?;
                self.send(character.char_id, vec![0xFA, 0, 0])
            }
            PartyRequest::InviteParty(account_id) => {
                let target = state
                    .characters()
                    .values()
                    .find(|other| other.account_id == account_id && other.loaded_from_client_side)
                    .map(|other| other.char_id);
                self.invite_party(state, character, target)
            }
            PartyRequest::InvitePartyByName(name) => {
                let target = state
                    .characters()
                    .values()
                    .find(|other| other.name == name && other.loaded_from_client_side)
                    .map(|other| other.char_id);
                self.invite_party(state, character, target)
            }
            PartyRequest::AnswerPartyInvite { party_id, accept } => {
                let invitation = character
                    .game_systems
                    .party_invitation
                    .take()
                    .filter(|invite| invite.party_id == party_id)
                    .ok_or("Party reply does not match a pending invitation")?;
                if character.game_systems.party_id != 0 {
                    return Err("Character already belongs to a party".into());
                }
                if !accept {
                    return self.send(invitation.inviter_id, invitation_result(&character.name, 1));
                }
                let inviter = state
                    .characters()
                    .get(&invitation.inviter_id)
                    .filter(|inviter| inviter.loaded_from_client_side && inviter.game_systems.party_id == party_id)
                    .ok_or("Party inviter is no longer online")?;
                let party = self
                    .repository
                    .party(party_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Party was disbanded")?;
                if party.leader_char_id != inviter.char_id {
                    return Err("Party invitation is no longer authorized".into());
                }
                if party.members.len() >= 12 {
                    return self.send(invitation.inviter_id, invitation_result(&character.name, 3));
                }
                let change = self
                    .repository
                    .join_party(character.char_id, party_id, invitation.inviter_id)
                    .map_err(|error| error.to_string())?;
                self.install_party_change(server, state, character, change, false)?;
                self.send(invitation.inviter_id, invitation_result(&character.name, 2))?;
                self.refresh_party(server, state, character)
            }
            PartyRequest::LeaveParty => {
                let change = self
                    .repository
                    .leave_party(character.char_id, None)
                    .map_err(|error| error.to_string())?;
                self.install_party_change(server, state, character, change, false)
            }
            PartyRequest::ExpelParty { account_id, name } => {
                let party = self
                    .repository
                    .party(character.game_systems.party_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Character has no party")?;
                if party.leader_char_id != character.char_id {
                    return Err("Only the party leader can expel members".into());
                }
                let records = self.repository.party_member_records(party.id).map_err(|error| error.to_string())?;
                let target = records
                    .iter()
                    .find(|member| member.account_id as u32 == account_id && member.name == name)
                    .ok_or("Party member identity does not match")?;
                let change = self
                    .repository
                    .leave_party(target.char_id as u32, Some(character.char_id))
                    .map_err(|error| error.to_string())?;
                self.install_party_change(server, state, character, change, true)
            }
            PartyRequest::ChangePartyLeader(account_id) => {
                let party = cached_party(character).ok_or("Character has no party")?.clone();
                let target = state
                    .characters()
                    .values()
                    .find(|other| {
                        other.account_id == account_id && other.loaded_from_client_side && other.game_systems.party_id == party.id
                    })
                    .ok_or("New party leader is not online in this party")?;
                if target.current_map_name() != character.current_map_name()
                    || target.current_map_instance() != character.current_map_instance()
                {
                    return Err("New party leader must be on the same map".into());
                }
                let target_id = target.char_id;
                let mut packet = protocol::header(0x07FC);
                packet.extend_from_slice(&character.account_id.to_le_bytes());
                packet.extend_from_slice(&target.account_id.to_le_bytes());
                let change = self
                    .repository
                    .change_party_leader(character.char_id, target_id, party.revision)
                    .map_err(|error| error.to_string())?;
                self.install_party_change(server, state, character, change, false)?;
                for member in &party.members {
                    self.send(*member, packet.clone())?;
                }
                Ok(())
            }
            PartyRequest::ChangePartyOptions { exp_share, item_rules } => {
                let party = cached_party(character).ok_or("Character has no party")?.clone();
                if party.leader_char_id != character.char_id {
                    return Err("Only the party leader can change sharing".into());
                }
                let allowed = !exp_share || share_level_allowed(state, character, &party);
                let (pickup, share) = item_rules.unwrap_or((party.item_pickup, party.item_share));
                let change = self
                    .repository
                    .change_party_options(character.char_id, party.revision, exp_share && allowed, pickup, share)
                    .map_err(|error| error.to_string())?;
                self.install_party_change(server, state, character, change, false)?;
                if !allowed {
                    self.send(character.char_id, options_packet(cached_party(character).unwrap(), true))?;
                }
                Ok(())
            }
            PartyRequest::DisablePartyInvites(disabled) => {
                let mut systems = character.game_systems.clone();
                systems.party_invite_disabled = disabled;
                let saved = self
                    .repository
                    .save_character_game_systems(character.char_id, &systems)
                    .map_err(|error| error.to_string())?;
                install_state(character, saved);
                self.send(character.char_id, vec![0xC9, 0x02, u8::from(disabled)])
            }
            PartyRequest::PartyMessage(text) => {
                let party = cached_party(character).ok_or("Character has no party")?;
                let prefix = format!("{} : ", character.name);
                let body = text
                    .strip_prefix(&prefix)
                    .ok_or("Party chat author does not match the authenticated character")?;
                if text.len() > 254 || body.is_empty() || body.chars().any(|ch| ch == '\0' || ch == '\r' || ch == '\n') {
                    return Err("Invalid party chat message".into());
                }
                let mut packet = protocol::header(0x0109);
                packet.extend_from_slice(&(9u16 + text.len() as u16).to_le_bytes());
                packet.extend_from_slice(&character.account_id.to_le_bytes());
                packet.extend_from_slice(text.as_bytes());
                packet.push(0);
                for member in &party.members {
                    self.send(*member, packet.clone())?;
                }
                Ok(())
            }
            PartyRequest::RefreshParty => self.refresh_party(server, state, character),
        }
    }

    fn invite_party(&self, state: &mut ServerState, character: &Character, target_id: Option<u32>) -> Result<(), String> {
        let party = cached_party(character).ok_or("Inviter does not belong to a party")?;
        if party.leader_char_id != character.char_id {
            return Err("Only the party leader can invite members".into());
        }
        let Some(target_id) = target_id else {
            return self.send(character.char_id, invitation_result("", 7));
        };
        let target = state.characters().get(&target_id).ok_or("Party invite target is not online")?;
        let result = if target.game_systems.party_id != 0
            || target.game_systems.party_invitation.is_some()
            || target.game_systems.guild_invitation.is_some()
        {
            Some(0)
        } else if target.game_systems.party_invite_disabled {
            Some(5)
        } else if party.members.len() >= 12 {
            Some(3)
        } else if self
            .repository
            .party_member_records(party.id)
            .map_err(|error| error.to_string())?
            .iter()
            .any(|member| member.account_id as u32 == target.account_id)
        {
            Some(4)
        } else {
            None
        };
        if let Some(result) = result {
            return self.send(character.char_id, invitation_result(&target.name, result));
        }
        let target = state.characters_mut().get_mut(&target_id).unwrap();
        target.game_systems.party_invitation = Some(PartyInvitation {
            party_id: party.id,
            inviter_id: character.char_id,
        });
        let mut packet = protocol::header(0x02C6);
        packet.extend_from_slice(&party.id.to_le_bytes());
        protocol::fixed_string(&mut packet, &party.name, 24);
        self.send(target_id, packet)
    }

    fn install_party_change(
        &self,
        _server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        change: PartyChange,
        expelled: bool,
    ) -> Result<(), String> {
        let recipients = change.before.as_ref().map(|party| party.members.clone()).unwrap_or_default();
        let affected_ids: Vec<_> = change.member_states.keys().copied().collect();
        for (id, saved) in change.member_states {
            let actor = if id == character.char_id {
                Some(&mut *character)
            } else {
                state.characters_mut().get_mut(&id)
            };
            if let Some(actor) = actor {
                install_state(actor, saved);
                actor.game_systems.party = change.party.as_ref().filter(|party| party.members.contains(&id)).cloned();
                actor.game_systems.party_position = None;
                actor.game_systems.party_health = None;
            }
        }
        for removed in &change.removed_members {
            let record = change
                .member_records
                .iter()
                .find(|record| record.char_id as u32 == *removed)
                .ok_or("Missing departed party member")?;
            let packet = withdrawal(record, expelled);
            for recipient in &recipients {
                self.send(*recipient, packet.clone())?;
            }
        }
        for id in affected_ids {
            let actor = if id == character.char_id {
                Some(&*character)
            } else {
                state.characters().get(&id)
            };
            if let Some(actor) = actor {
                self.area(actor, names_packet(actor))?;
            }
        }
        if let Some(party) = &change.party {
            let packet = roster_packet(
                party,
                &roster(
                    state,
                    character,
                    &change
                        .member_records
                        .iter()
                        .filter(|record| party.members.contains(&(record.char_id as u32)))
                        .cloned()
                        .collect::<Vec<_>>(),
                    None,
                ),
            );
            for member in &party.members {
                self.send(*member, packet.clone())?;
                self.send(*member, options_packet(party, false))?;
            }
            if party.members.contains(&character.char_id) {
                let entry = member_packet(party, character);
                for member in &party.members {
                    self.send(*member, entry.clone())?;
                }
            }
        }
        Ok(())
    }

    fn refresh_party(&self, server: &Server, state: &mut ServerState, character: &mut Character) -> Result<(), String> {
        let Some(party) = cached_party(character).cloned() else {
            return Ok(());
        };
        if party.exp_share && !share_level_allowed(state, character, &party) {
            let change = self
                .repository
                .change_party_options(party.leader_char_id, party.revision, false, party.item_pickup, party.item_share)
                .map_err(|error| error.to_string())?;
            return self.install_party_change(server, state, character, change, false);
        }
        let records = self.repository.party_member_records(party.id).map_err(|error| error.to_string())?;
        let packet = roster_packet(&party, &roster(state, character, &records, None));
        for member in &party.members {
            self.send(*member, packet.clone())?;
        }
        Ok(())
    }

    pub(crate) fn party_tick(&self, server: &Server, character: &mut Character, now: u64) -> Result<(), String> {
        let Some(party) = cached_party(character).cloned() else {
            return Ok(());
        };
        let hp = (
            character.status.hp,
            crate::server::service::status_service::StatusService::instance()
                .to_snapshot(&character.status)
                .max_hp(),
        );
        let health_changed = character.game_systems.party_health != Some(hp);
        let position = (
            character.current_map_name().clone(),
            character.current_map_instance(),
            character.x,
            character.y,
            character.status.base_level,
        );
        let map_changed = character
            .game_systems
            .party_position
            .as_ref()
            .is_none_or(|previous| previous.0 != position.0 || previous.1 != position.1 || previous.4 != position.4);
        let position_changed = now >= character.game_systems.last_party_update.saturating_add(1000)
            && character.game_systems.party_position.as_ref() != Some(&position);
        if map_changed && character.game_systems.party_position.is_some() {
            server.add_to_next_tick(GameEvent::ScriptWorld(ScriptWorld {
                char_id: character.char_id,
                request: ScriptWorldRequest::Party(PartyRequest::RefreshParty),
            }));
        }
        if health_changed || position_changed || map_changed {
            let state = server.state();
            for member in &party.members {
                if *member == character.char_id {
                    continue;
                }
                let Some(other) = live_member(state, character, *member) else {
                    continue;
                };
                let same_map = other.current_map_name() == character.current_map_name()
                    && other.current_map_instance() == character.current_map_instance();
                if position_changed || map_changed {
                    self.send(
                        *member,
                        position_packet(
                            character.account_id,
                            if same_map { character.x } else { u16::MAX },
                            if same_map { character.y } else { u16::MAX },
                        ),
                    )?;
                }
                if health_changed && same_map && other.is_map_item_in_fov(character.char_id) {
                    self.send(*member, health_packet(character.account_id, hp.0, hp.1))?;
                }
            }
            character.game_systems.party_health = Some(hp);
            if position_changed || map_changed {
                character.game_systems.party_position = Some(position);
                character.game_systems.last_party_update = now;
            }
        }
        Ok(())
    }
}

impl ScriptWorldService {
}

#[cfg(test)]
mod tests {
    use models::status::Status;

    use super::*;
    use crate::server::model::map_item::MapItems;

    fn member(id: u32, party: &PartyRecord, map: &str, instance: u8, hp: u32) -> Character {
        let mut character = Character::new(
            format!("Member{id}"),
            id,
            id + 2_000_000,
            Status {
                hp,
                max_hp: 100,
                base_level: 50,
                ..Status::default()
            },
            50,
            50,
            0,
            map.into(),
            0,
            Vec::new(),
        );
        character.loaded_from_client_side = true;
        character.map_instance_key = crate::server::model::map_instance::MapInstanceKey::new(map.into(), instance);
        character.game_systems.party_id = party.id;
        character.game_systems.party_leader_id = party.leader_char_id;
        character.game_systems.party_members = party.members.clone();
        character.game_systems.party_name = party.name.clone();
        character.game_systems.party = Some(party.clone());
        character
    }

    #[test]
    fn party_experience_and_loot_rules_use_alive_online_members_and_exact_map_instances() {
        let party = PartyRecord {
            id: 77,
            revision: 1,
            name: "Party".into(),
            leader_char_id: 1,
            members: vec![1, 2, 3, 4, 5],
            exp_share: true,
            item_pickup: true,
            item_share: true,
        };
        let mut state = ServerState::new(MapItems::default());
        let mut owner = member(1, &party, "prontera.gat", 0, 100);
        state.insert_character(member(2, &party, "prontera.gat", 0, 100));
        state.insert_character(member(3, &party, "prontera.gat", 0, 0));
        state.insert_character(member(4, &party, "prontera.gat", 1, 100));
        state.insert_character(member(5, &party, "geffen.gat", 0, 100));
        assert_eq!(party_experience_awards(&state, &owner, 101, 51, "prontera", 0), vec![
            (1, 50, 25),
            (2, 50, 25)
        ]);
        assert_eq!(party_loot_candidates(&state, &owner), vec![1, 2]);
        state.characters_mut().get_mut(&3).unwrap().status.hp = 100;
        assert_eq!(
            party_experience_awards_with_bonus(&state, &owner, 101, 51, "prontera.gat", 0, 10),
            vec![(1, 39, 20), (2, 39, 20), (3, 39, 20)]
        );
        state.characters_mut().get_mut(&3).unwrap().status.hp = 0;
        assert!(party_can_pick_up(&owner, 2));
        assert!(!party_can_pick_up(&owner, 6));
        owner.game_systems.party.as_mut().unwrap().item_pickup = false;
        assert!(!party_can_pick_up(&owner, 2));
        assert!(party_can_pick_up(&owner, 1));
        state.characters_mut().get_mut(&5).unwrap().status.base_level = 66;
        assert_eq!(party_experience_awards(&state, &owner, 101, 51, "prontera.gat", 0), vec![(
            1, 101, 51
        )]);
        state.characters_mut().get_mut(&5).unwrap().loaded_from_client_side = false;
        assert_eq!(party_experience_awards(&state, &owner, 101, 51, "prontera.gat", 0), vec![
            (1, 50, 25),
            (2, 50, 25)
        ]);
        owner.game_systems.party.as_mut().unwrap().item_share = false;
        assert_eq!(party_loot_candidates(&state, &owner), vec![1]);
    }

    #[test]
    fn party_client_roster_entry_and_withdrawal_encode_real_account_ids_and_roles() {
        let party = PartyRecord {
            id: 77,
            revision: 1,
            name: "Party".into(),
            leader_char_id: 1,
            members: vec![1, 2],
            exp_share: false,
            item_pickup: true,
            item_share: false,
        };
        let actor = member(1, &party, "prontera.gat", 0, 100);
        let entry = member_packet(&party, &actor);
        assert_eq!(entry.len(), 81);
        assert_eq!(u32::from_le_bytes(entry[2..6].try_into().unwrap()), actor.account_id);
        assert_eq!(u32::from_le_bytes(entry[6..10].try_into().unwrap()), 0);
        assert_eq!(&entry[79..], &[1, 0]);
        let leader = CharacterRecord {
            char_id: 1,
            account_id: actor.account_id as i32,
            name: actor.name.clone(),
            last_map: "prontera".into(),
            ..CharacterRecord::default()
        };
        let member = CharacterRecord {
            char_id: 2,
            account_id: 2_000_002,
            name: "Member2".into(),
            last_map: "geffen".into(),
            ..CharacterRecord::default()
        };
        let roster = roster_packet(&party, &[(leader.clone(), true), (member, false)]);
        assert_eq!(roster.len(), 120);
        assert_eq!(&roster[72..74], &[0, 0]);
        assert_eq!(&roster[118..120], &[1, 1]);
        let withdraw = withdrawal(&leader, true);
        assert_eq!(withdraw.len(), 31);
        assert_eq!(withdraw[30], 1);
    }
}
