use database::model::CharacterRecord;
use models::enums::EnumWithMaskValueU32;

use super::{ScriptWorldService, install_state, protocol, world_data};
use crate::server::Server;
use crate::server::model::game_systems::{GuildInvitation, GuildMenu, GuildPermission, GuildPosition, GuildRecord};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub fn guild_actor_packet(actor_id: u32, guild_id: u32) -> Vec<u8> {
    let mut packet = protocol::header(0x01B4);
    packet.extend_from_slice(&actor_id.to_le_bytes());
    packet.extend_from_slice(&guild_id.to_le_bytes());
    packet.extend_from_slice(&0u16.to_le_bytes());
    packet
}

fn belong_packet(actor: &Character, guild: Option<&GuildRecord>) -> Vec<u8> {
    let mut packet = protocol::header(0x016C);
    let master = guild.is_some_and(|guild| guild.master_char_id == actor.char_id);
    packet.extend_from_slice(&guild.map_or(0, |guild| guild.id).to_le_bytes());
    packet.extend_from_slice(&0u32.to_le_bytes());
    let mut permissions = 0;
    if guild.is_some_and(|guild| guild.can_invite(actor.char_id)) {
        permissions |= GuildPermission::Invite.as_flag();
    }
    if guild.is_some_and(|guild| guild.can_punish(actor.char_id)) {
        permissions |= GuildPermission::Expel.as_flag();
    }
    packet.extend_from_slice(&permissions.to_le_bytes());
    packet.push(u8::from(master));
    packet.extend_from_slice(&0u32.to_le_bytes());
    protocol::fixed_string(&mut packet, guild.map_or("", |guild| guild.name.as_str()), 24);
    packet
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuildRequest {
    CreateGuild(String),
    InviteGuild(u32),
    AnswerGuildInvite {
            guild_id: u32,
            accept: bool,
        },
    GuildMenu,
    GuildInformation(u32),
    LeaveGuild {
            guild_id: u32,
            reason: String,
        },
    ExpelGuild {
            guild_id: u32,
            member_id: u32,
            reason: String,
        },
    DisbandGuild(String),
    GuildNotice { subject: String, body: String },
    GuildPositions(Vec<(u32, GuildPosition)>),
    GuildMemberPositions(Vec<(u32, u32)>),
    GuildEmblem(Vec<u8>),
    GuildEmblemRequest(u32),
    GuildSkillUp(u32),
    GuildAllianceRequest(u32),
    GuildAllianceReply { inviter: u32, accept: bool },
    GuildOpposition(u32),
    GuildRelationBreak { guild_id: u32, hostile: bool },
    GuildMessage(String),
}


impl ScriptWorldService {
    pub fn initialize_guild(&self, server: &Server, character: &Character) -> Result<(), String> {
        if character.game_systems.guild_id == 0 {
            return Ok(());
        }
        let guild = self
            .repository
            .guild(character.game_systems.guild_id)
            .map_err(|error| error.to_string())?
            .ok_or("Character guild no longer exists")?;
        self.send(character.char_id, belong_packet(character, Some(&guild)))?;
        if !guild.notice.subject.is_empty() || !guild.notice.body.is_empty() {
            self.send(character.char_id, guild_notice_packet(&guild))?;
        }
        self.send(character.char_id, self.guild_relations_packet(&guild)?)?;
        self.send(character.char_id, self.guild_summary_packet(server, character, &guild)?)
    }

    fn guild_relations_packet(&self, guild: &GuildRecord) -> Result<Vec<u8>, String> {
        let mut packet = protocol::header(0x014C);
        packet.extend_from_slice(&0u16.to_le_bytes());
        for (relation, ids) in [(0u32, &guild.allies), (1, &guild.opposition)] {
            for id in ids {
                let other = self
                    .repository
                    .guild(*id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Related guild no longer exists")?;
                packet.extend_from_slice(&relation.to_le_bytes());
                packet.extend_from_slice(&other.id.to_le_bytes());
                protocol::fixed_string(&mut packet, &other.name, 24);
            }
        }
        protocol::set_length(&mut packet);
        Ok(packet)
    }

    fn refresh_guild_relations(&self, guilds: &[&GuildRecord]) -> Result<(), String> {
        for guild in guilds {
            self.broadcast_guild_packet(guild, self.guild_relations_packet(guild)?)?;
        }
        Ok(())
    }

    fn guild_master_of(&self, character: &Character) -> Result<GuildRecord, String> {
        self.repository
            .guild(character.game_systems.guild_id)
            .map_err(|error| error.to_string())?
            .filter(|guild| guild.master_char_id == character.char_id)
            .ok_or_else(|| "Only the guild master can manage guild relations".to_string())
    }

    pub(crate) fn guild_request(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        request: GuildRequest,
    ) -> Result<(), String> {
        match request {
            GuildRequest::CreateGuild(name) => {
                if character.game_systems.guild_id != 0 {
                    return self.send(character.char_id, vec![0x67, 0x01, 1]);
                }
                if !character
                    .inventory
                    .iter()
                    .filter_map(Option::as_ref)
                    .any(|item| item.item_id == 714 && item.amount > 0 && item.equip == 0)
                {
                    return self.send(character.char_id, vec![0x67, 0x01, 3]);
                }
                let guild = match self.repository.create_guild(character.char_id, name) {
                    Ok(guild) => guild,
                    Err(error) => {
                        self.send(character.char_id, vec![0x67, 0x01, 2])?;
                        return Err(error.to_string());
                    }
                };
                self.reload_guild_member(character)?;
                character.guild_name = guild.name.clone();
                server
                    .inventory_service()
                    .reload_inventory(server.runtime(), character.char_id, character);
                self.send(character.char_id, vec![0x67, 0x01, 0])?;
                self.send(character.char_id, belong_packet(character, Some(&guild)))?;
                self.send(character.char_id, self.guild_summary_packet(server, character, &guild)?)?;
                self.area(character, guild_actor_packet(character.char_id, guild.id))
            }
            GuildRequest::InviteGuild(target_id) => {
                let guild = self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Inviter does not belong to a guild")?;
                if !guild.can_invite(character.char_id) {
                    return Err("Your guild position cannot invite members".into());
                }
                let target = state
                    .characters_mut()
                    .get_mut(&target_id)
                    .ok_or("Guild invite target is not online")?;
                if target.game_systems.guild_id != 0 || target.game_systems.guild_invitation.is_some() {
                    return self.send(character.char_id, vec![0x69, 0x01, 0]);
                }
                if guild.members.len() >= guild.max_members() {
                    return self.send(character.char_id, vec![0x69, 0x01, 3]);
                }
                target.game_systems.guild_invitation = Some(GuildInvitation {
                    guild_id: guild.id,
                    inviter_id: character.char_id,
                });
                let mut packet = protocol::header(0x016A);
                packet.extend_from_slice(&guild.id.to_le_bytes());
                protocol::fixed_string(&mut packet, &guild.name, 24);
                self.send(target.char_id, packet)
            }
            GuildRequest::AnswerGuildInvite { guild_id, accept } => {
                if character
                    .game_systems
                    .guild_invitation
                    .as_ref()
                    .is_none_or(|invite| invite.guild_id != guild_id)
                {
                    return Err("Guild invitation reply does not match a pending invitation".into());
                }
                let invitation = character.game_systems.guild_invitation.take().unwrap();
                if !accept || character.game_systems.guild_id != 0 {
                    return self.send(invitation.inviter_id, vec![0x69, 0x01, 1]);
                }
                let inviter = state
                    .characters()
                    .get(&invitation.inviter_id)
                    .filter(|inviter| inviter.game_systems.guild_id == guild_id)
                    .ok_or("Guild inviter is no longer available")?;
                let guild = self
                    .repository
                    .guild(guild_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Inviting guild was dissolved")?;
                if !guild.can_invite(inviter.char_id) {
                    return Err("Guild invitation is no longer authorized".into());
                }
                if guild.members.len() >= guild.max_members() {
                    return self.send(invitation.inviter_id, vec![0x69, 0x01, 3]);
                }
                let guild = self
                    .repository
                    .join_guild(character.char_id, guild_id)
                    .map_err(|error| error.to_string())?;
                self.reload_guild_member(character)?;
                character.guild_name = guild.name.clone();
                self.send(invitation.inviter_id, vec![0x69, 0x01, 2])?;
                self.send(character.char_id, belong_packet(character, Some(&guild)))?;
                self.broadcast_guild_summary(server, character, &guild)?;
                self.area(character, guild_actor_packet(character.char_id, guild.id))
            }
            GuildRequest::GuildMenu => {
                let guild = self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?;
                let mut mode = GuildMenu::Members.as_flag() | GuildMenu::Skills.as_flag() | GuildMenu::Notice.as_flag();
                if guild.as_ref().is_some_and(|guild| guild.master_char_id == character.char_id) {
                    mode |= GuildMenu::Positions.as_flag();
                }
                if guild.as_ref().is_some_and(|guild| guild.can_punish(character.char_id)) {
                    mode |= GuildMenu::Expulsions.as_flag();
                }
                let mut packet = protocol::header(0x014E);
                packet.extend_from_slice(&mode.to_le_bytes());
                self.send(character.char_id, packet)
            }
            GuildRequest::GuildInformation(kind) => {
                let guild = self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Character does not belong to a guild")?;
                match kind {
                    0 => self.send(character.char_id, self.guild_summary_packet(server, character, &guild)?),
                    3 => self.send(character.char_id, guild_skills_packet(&guild)),
                    1 => {
                        self.send(character.char_id, guild_positions(false, &guild))?;
                        self.send(character.char_id, self.guild_members_packet(server, character, &guild)?)
                    }
                    2 => {
                        self.send(character.char_id, guild_positions(false, &guild))?;
                        self.send(character.char_id, guild_positions(true, &guild))
                    }
                    4 => self.send(character.char_id, vec![0x63, 0x01, 4, 0]),
                    6 => {
                        self.send(character.char_id, guild_notice_packet(&guild))
                    }
                    _ => Err("Unknown guild information section".into()),
                }
            }
            GuildRequest::LeaveGuild { guild_id, reason } => {
                self.guild_remove_member(server, state, character, guild_id, character.char_id, reason, false)
            }
            GuildRequest::ExpelGuild {
                guild_id,
                member_id,
                reason,
            } => self.guild_remove_member(server, state, character, guild_id, member_id, reason, true),
            GuildRequest::DisbandGuild(name) => {
                if self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?
                    .is_some_and(|guild| guild.name == name && guild.master_char_id == character.char_id && guild.members.len() > 1)
                {
                    return self.send(character.char_id, vec![0x5E, 0x01, 2, 0, 0, 0]);
                }
                let guild = match self.repository.disband_guild(character.char_id, &name) {
                    Ok(guild) => guild,
                    Err(error) => {
                        self.send(character.char_id, vec![0x5E, 0x01, 1, 0, 0, 0])?;
                        return Err(error.to_string());
                    }
                };
                server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::CastleLifecycle(crate::server::model::events::game_event::CastleLifecycle::GuildBroken {
                    guild_id: guild.id,
                }));
                for member in &guild.members {
                    self.send(*member, vec![0x5E, 0x01, 0, 0, 0, 0])?;
                    if *member == character.char_id {
                        self.reload_guild_member(character)?;
                        character.guild_name.clear();
                        self.area(character, guild_actor_packet(*member, 0))?;
                    } else if let Some(other) = state.characters_mut().get_mut(member) {
                        self.reload_guild_member(other)?;
                        other.guild_name.clear();
                        self.area(other, guild_actor_packet(*member, 0))?;
                    }
                }
                Ok(())
            }
            GuildRequest::GuildNotice { subject, body } => {
                let guild = self
                    .repository
                    .guild_set_notice(character.char_id, crate::server::model::game_systems::GuildNotice { subject, body })
                    .map_err(|error| error.to_string())?;
                self.broadcast_guild_packet(&guild, guild_notice_packet(&guild))
            }
            GuildRequest::GuildPositions(changes) => {
                let current = self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Character does not belong to a guild")?;
                let mut positions = current.positions.clone();
                for (index, position) in changes {
                    let slot = positions.get_mut(index as usize).ok_or("Unknown guild position")?;
                    *slot = position;
                }
                let guild = self
                    .repository
                    .guild_set_positions(character.char_id, positions)
                    .map_err(|error| error.to_string())?;
                self.broadcast_guild_packet(&guild, guild_positions(false, &guild))?;
                self.broadcast_guild_packet(&guild, guild_positions(true, &guild))
            }
            GuildRequest::GuildMemberPositions(changes) => {
                let mut latest = None;
                for (member, position) in changes {
                    let position = u8::try_from(position).map_err(|_| "Invalid guild position")?;
                    latest = Some(
                        self.repository
                            .guild_set_member_position(character.char_id, member, position)
                            .map_err(|error| error.to_string())?,
                    );
                }
                let guild = latest.ok_or("Empty guild position change")?;
                for member in &guild.members {
                    self.send(*member, self.guild_members_packet(server, character, &guild)?)?;
                }
                Ok(())
            }
            GuildRequest::GuildEmblem(emblem) => {
                let guild = self
                    .repository
                    .guild_set_emblem(character.char_id, emblem)
                    .map_err(|error| error.to_string())?;
                for member in &guild.members {
                    self.send(*member, guild_emblem_packet(&guild))?;
                    if *member == character.char_id {
                        self.area(character, guild_actor_packet(*member, guild.id))?;
                    } else if let Some(other) = state.characters().get(member) {
                        self.area(other, guild_actor_packet(*member, guild.id))?;
                    }
                }
                Ok(())
            }
            GuildRequest::GuildEmblemRequest(guild_id) => {
                if let Some(guild) = self.repository.guild(guild_id).map_err(|error| error.to_string())? {
                    if !guild.emblem.is_empty() {
                        self.send(character.char_id, guild_emblem_packet(&guild))?;
                    }
                }
                Ok(())
            }
            GuildRequest::GuildMessage(message) => {
                let prefix = format!("{} : ", character.name);
                if !message.starts_with(&prefix) {
                    return Err("Guild chat sender does not match the character".into());
                }
                if server.battleground_chat(state, character, &message) {
                    return Ok(());
                }
                let guild = self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Character does not belong to a guild")?;
                let mut packet = protocol::header(0x017F);
                packet.extend_from_slice(&0u16.to_le_bytes());
                packet.extend_from_slice(message.as_bytes());
                packet.push(0);
                protocol::set_length(&mut packet);
                self.broadcast_guild_packet(&guild, packet)
            }
            GuildRequest::GuildSkillUp(skill_id) => {
                let guild = self
                    .repository
                    .guild_upgrade_skill(character.char_id, skill_id)
                    .map_err(|error| error.to_string())?;
                let level = guild.skill_level(skill_id);
                let mut ack = protocol::header(0x010E);
                for value in [skill_id as u16, u16::from(level), 0, 0] {
                    ack.extend_from_slice(&value.to_le_bytes());
                }
                ack.push(1);
                self.send(character.char_id, ack)?;
                self.broadcast_guild_packet(&guild, guild_skills_packet(&guild))?;
                self.broadcast_guild_summary(server, character, &guild)
            }
            GuildRequest::GuildAllianceRequest(target_id) => {
                let guild = self.guild_master_of(character)?;
                if state.siege_active() {
                    return Err("Alliances cannot be made during Guild Wars".into());
                }
                let target = state
                    .characters()
                    .values()
                    .find(|other| other.account_id == target_id || other.char_id == target_id)
                    .ok_or("Alliance target is not online")?;
                let target_guild = self
                    .repository
                    .guild(target.game_systems.guild_id)
                    .map_err(|error| error.to_string())?
                    .filter(|other| other.master_char_id == target.char_id && other.id != guild.id)
                    .ok_or("Alliance target is not the master of another guild")?;
                let target_char = target.char_id;
                let inviter_account = character.account_id;
                if guild.allies.contains(&target_guild.id) {
                    return self.send(character.char_id, vec![0x73, 0x01, 0]);
                }
                if guild.allies.len() >= usize::from(server.configuration.game.guild_max_alliances) {
                    return self.send(character.char_id, vec![0x73, 0x01, 4]);
                }
                if target_guild.allies.len() >= usize::from(server.configuration.game.guild_max_alliances) {
                    return self.send(character.char_id, vec![0x73, 0x01, 3]);
                }
                if !self.guild_alliance_requests.request(target_char, character.char_id, guild.id) {
                    return self.send(character.char_id, vec![0x73, 0x01, 1]);
                }
                let mut packet = protocol::header(0x0171);
                packet.extend_from_slice(&inviter_account.to_le_bytes());
                protocol::fixed_string(&mut packet, &guild.name, 24);
                self.send(target_char, packet)
            }
            GuildRequest::GuildAllianceReply { inviter, accept } => {
                let (inviter_char, inviting_guild) = self
                    .guild_alliance_requests
                    .take(character.char_id)
                    .ok_or("No pending guild alliance request")?;
                if !state
                    .characters()
                    .get(&inviter_char)
                    .is_some_and(|other| other.account_id == inviter && other.game_systems.guild_id == inviting_guild)
                {
                    return Err("Guild alliance inviter is no longer available".into());
                }
                if !accept {
                    return self.send(inviter_char, vec![0x73, 0x01, 2]);
                }
                if state.siege_active() {
                    return Err("Alliances cannot be made during Guild Wars".into());
                }
                let (first, second) = self
                    .repository
                    .guild_form_alliance(inviter_char, character.char_id)
                    .map_err(|error| error.to_string())?;
                self.send(inviter_char, vec![0x73, 0x01, 1])?;
                self.send(character.char_id, vec![0x73, 0x01, 1])?;
                self.refresh_guild_relations(&[&first, &second])
            }
            GuildRequest::GuildOpposition(target_id) => {
                let guild = self.guild_master_of(character)?;
                let target = state
                    .characters()
                    .values()
                    .find(|other| other.account_id == target_id || other.char_id == target_id)
                    .filter(|other| other.game_systems.guild_id != 0 && other.game_systems.guild_id != guild.id)
                    .ok_or("Opposition target is not a member of another guild")?;
                let target_guild = target.game_systems.guild_id;
                if guild.opposition.contains(&target_guild) {
                    return self.send(character.char_id, vec![0x81, 0x01, 2]);
                }
                if guild.opposition.len() >= 3 {
                    return self.send(character.char_id, vec![0x81, 0x01, 1]);
                }
                let guild = self
                    .repository
                    .guild_declare_opposition(character.char_id, target_guild)
                    .map_err(|error| error.to_string())?;
                self.send(character.char_id, vec![0x81, 0x01, 0])?;
                let opposed = self.repository.guild(target_guild).map_err(|error| error.to_string())?;
                let mut guilds = vec![&guild];
                guilds.extend(opposed.as_ref());
                self.refresh_guild_relations(&guilds)
            }
            GuildRequest::GuildRelationBreak { guild_id, .. } => {
                let before = self.guild_master_of(character)?;
                if state.siege_active() {
                    return Err("Guild relations cannot change during Guild Wars".into());
                }
                let hostile = before.opposition.contains(&guild_id);
                let guild = self
                    .repository
                    .guild_break_relation(character.char_id, guild_id)
                    .map_err(|error| error.to_string())?;
                let other = self.repository.guild(guild_id).map_err(|error| error.to_string())?;
                let mut guilds = vec![&guild];
                guilds.extend(other.as_ref());
                self.refresh_guild_relations(&guilds)?;
                self.broadcast_guild_packet(&guild, relation_deleted_packet(guild_id, hostile))?;
                if let Some(other) = other.as_ref() {
                    self.broadcast_guild_packet(other, relation_deleted_packet(guild.id, hostile))?;
                }
                Ok(())
            }
        }
    }

    fn broadcast_guild_packet(&self, guild: &GuildRecord, packet: Vec<u8>) -> Result<(), String> {
        for member in &guild.members {
            self.send(*member, packet.clone())?;
        }
        Ok(())
    }

    fn reload_guild_member(&self, character: &mut Character) -> Result<(), String> {
        let saved = self
            .repository
            .character_game_systems(character.char_id)
            .map_err(|error| error.to_string())?;
        if character.game_systems.guild_storage_open.is_some_and(|guild_id| guild_id != saved.guild_id) { self.close_storage(character)?; }
        install_state(character, saved);
        Ok(())
    }

    fn guild_roster(&self, server: &Server, character: &Character, guild: &GuildRecord) -> Result<Vec<(CharacterRecord, bool)>, String> {
        Ok(self
            .repository
            .guild_member_records(guild.id)
            .map_err(|error| error.to_string())?
            .into_iter()
            .map(|mut record| {
                let online = if record.char_id as u32 == character.char_id {
                    Some(character)
                } else {
                    server.state().characters().get(&(record.char_id as u32))
                };
                if let Some(member) = online {
                    record.name = member.name.clone();
                    record.class = member.status.job.min(i16::MAX as u32) as i16;
                    record.base_level = member.status.base_level.min(i32::MAX as u32) as i32;
                    record.sex = if member.sex == 1 { "M" } else { "F" }.into();
                }
                (record, online.is_some())
            })
            .collect())
    }

    fn castles_held_by(&self, guild_id: u32) -> usize {
        crate::server::service::castle_service::castles()
            .iter()
            .filter(|castle| {
                self.repository
                    .castle_value(&castle.map, crate::server::service::castle_service::CD_GUILD_ID)
                    .is_ok_and(|owner| u32::try_from(owner).is_ok_and(|owner| owner == guild_id))
            })
            .count()
    }

    pub(crate) fn guild_summary_packet(&self, server: &Server, character: &Character, guild: &GuildRecord) -> Result<Vec<u8>, String> {
        let roster = self.guild_roster(server, character, guild)?;
        let mut packet = protocol::guild_basic(
            &guild.name,
            guild.id,
            guild.level,
            guild.experience,
            0,
            world_data().guild_experience.get(guild.level as usize - 1).copied().unwrap_or(0),
        );
        packet[10..14].copy_from_slice(&(roster.iter().filter(|(_, online)| *online).count() as u32).to_le_bytes());
        let average = roster.iter().map(|(record, _)| record.base_level.max(1) as u32).sum::<u32>() / roster.len().max(1) as u32;
        packet[18..22].copy_from_slice(&average.to_le_bytes());
        if let Some((master, _)) = roster.iter().find(|(record, _)| record.char_id as u32 == guild.master_char_id) {
            let mut name = Vec::new();
            protocol::fixed_string(&mut name, &master.name, 24);
            packet[70..94].copy_from_slice(&name);
        }
        packet[14..18].copy_from_slice(&(guild.max_members() as u32).to_le_bytes());
        let mut land = Vec::new();
        protocol::fixed_string(&mut land, castle_holdings_text(self.castles_held_by(guild.id)), 16);
        packet[94..110].copy_from_slice(&land);
        packet.extend_from_slice(&[0x62, 0x01, 6, 0]);
        packet.extend_from_slice(&guild.skill_points.to_le_bytes());
        Ok(packet)
    }

    pub(crate) fn broadcast_guild_summary(&self, server: &Server, character: &Character, guild: &GuildRecord) -> Result<(), String> {
        let packet = self.guild_summary_packet(server, character, guild)?;
        for member in &guild.members {
            self.send(*member, packet.clone())?;
        }
        Ok(())
    }

    fn guild_members_packet(&self, server: &Server, character: &Character, guild: &GuildRecord) -> Result<Vec<u8>, String> {
        let mut packet = protocol::header(0x0154);
        packet.extend_from_slice(&0u16.to_le_bytes());
        for (record, online) in self.guild_roster(server, character, guild)? {
            packet.extend_from_slice(&(record.char_id as u32).to_le_bytes());
            packet.extend_from_slice(&(record.char_id as u32).to_le_bytes());
            for value in [
                record.hair,
                record.hair_color,
                i16::from(record.sex == "M"),
                record.class,
                record.base_level.clamp(1, i32::from(i16::MAX)) as i16,
            ] {
                packet.extend_from_slice(&value.to_le_bytes());
            }
            packet.extend_from_slice(&0u32.to_le_bytes());
            packet.extend_from_slice(&u32::from(online).to_le_bytes());
            packet.extend_from_slice(&u32::from(guild.position_of(record.char_id as u32)).to_le_bytes());
            packet.extend_from_slice(&[0; 50]);
            protocol::fixed_string(&mut packet, &record.name, 24);
        }
        protocol::set_length(&mut packet);
        Ok(packet)
    }

    fn guild_remove_member(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        guild_id: u32,
        member: u32,
        reason: String,
        expelled: bool,
    ) -> Result<(), String> {
        if character.game_systems.guild_id != guild_id || reason.len() > 40 || reason.chars().any(char::is_control) {
            return Err("Invalid guild departure request".into());
        }
        let old = self
            .repository
            .guild(guild_id)
            .map_err(|error| error.to_string())?
            .ok_or("Guild does not exist")?;
        let record = self
            .repository
            .guild_member_records(guild_id)
            .map_err(|error| error.to_string())?
            .into_iter()
            .find(|record| record.char_id as u32 == member)
            .ok_or("Guild member does not exist")?;
        let guild = if expelled {
            self.repository.guild_expel_member(character.char_id, member)
        } else {
            self.repository.leave_guild(member, guild_id, None)
        }
        .map_err(|error| error.to_string())?;
        let mut packet = protocol::header(if expelled { 0x0839 } else { 0x015A });
        protocol::fixed_string(&mut packet, &record.name, 24);
        protocol::fixed_string(&mut packet, &reason, 40);
        for recipient in &old.members {
            self.send(*recipient, packet.clone())?;
        }
        if member == character.char_id {
            self.reload_guild_member(character)?;
            character.guild_name.clear();
            self.send(member, belong_packet(character, None))?;
            self.area(character, guild_actor_packet(member, 0))?;
        } else if let Some(other) = state.characters_mut().get_mut(&member) {
            self.reload_guild_member(other)?;
            other.guild_name.clear();
            self.send(member, belong_packet(other, None))?;
            self.area(other, guild_actor_packet(member, 0))?;
        }
        self.broadcast_guild_summary(server, character, &guild)
    }
}

const CASTLE_HOLDINGS: [&str; 21] = [
    "None Taken",
    "One Castle",
    "Two Castles",
    "Three Castles",
    "Four Castles",
    "Five Castles",
    "Six Castles",
    "Seven Castles",
    "Eight Castles",
    "Nine Castles",
    "Ten Castles",
    "Eleven Castles",
    "Twelve Castles",
    "Thirteen Castles",
    "Fourteen Castles",
    "Fifteen Castles",
    "Sixteen Castles",
    "Seventeen Castles",
    "Eighteen Castles",
    "Nineteen Castles",
    "Twenty Castles",
];

fn castle_holdings_text(count: usize) -> &'static str {
    CASTLE_HOLDINGS[count.min(CASTLE_HOLDINGS.len() - 1)]
}

fn guild_positions(info: bool, guild: &GuildRecord) -> Vec<u8> {
    let mut packet = protocol::header(if info { 0x0160 } else { 0x0166 });
    packet.extend_from_slice(&0u16.to_le_bytes());
    for (index, position) in guild.positions.iter().enumerate() {
        packet.extend_from_slice(&(index as u32).to_le_bytes());
        if info {
            let mut mode = 0;
            if position.invite {
                mode |= GuildPermission::Invite.as_flag();
            }
            if position.punish {
                mode |= GuildPermission::Expel.as_flag();
            }
            packet.extend_from_slice(&mode.to_le_bytes());
            packet.extend_from_slice(&(index as u32).to_le_bytes());
            packet.extend_from_slice(&u32::from(position.exp_tax).to_le_bytes());
        } else {
            protocol::fixed_string(&mut packet, &position.name, 24);
        }
    }
    protocol::set_length(&mut packet);
    packet
}

fn relation_deleted_packet(guild_id: u32, hostile: bool) -> Vec<u8> {
    let mut packet = protocol::header(0x0184);
    packet.extend_from_slice(&guild_id.to_le_bytes());
    packet.extend_from_slice(&u32::from(hostile).to_le_bytes());
    packet
}

fn guild_skills_packet(guild: &GuildRecord) -> Vec<u8> {
    let mut packet = protocol::header(0x0162);
    packet.extend_from_slice(&0u16.to_le_bytes());
    packet.extend_from_slice(&guild.skill_points.to_le_bytes());
    for (id, name, maximum, _) in crate::server::model::game_systems::GUILD_SKILL_TREE {
        if !guild.skill_available(*id) {
            continue;
        }
        let level = guild.skill_level(*id);
        packet.extend_from_slice(&(*id as u16).to_le_bytes());
        packet.extend_from_slice(&0u32.to_le_bytes());
        packet.extend_from_slice(&u16::from(level).to_le_bytes());
        packet.extend_from_slice(&0u16.to_le_bytes());
        packet.extend_from_slice(&0u16.to_le_bytes());
        protocol::fixed_string(&mut packet, name, 24);
        packet.push(u8::from(level < *maximum && guild.skill_points > 0));
    }
    protocol::set_length(&mut packet);
    packet
}

fn guild_notice_packet(guild: &GuildRecord) -> Vec<u8> {
    let mut packet = protocol::header(0x016F);
    protocol::fixed_string(&mut packet, &guild.notice.subject, 60);
    protocol::fixed_string(&mut packet, &guild.notice.body, 120);
    packet
}

fn guild_emblem_packet(guild: &GuildRecord) -> Vec<u8> {
    let mut packet = protocol::header(0x0152);
    packet.extend_from_slice(&0u16.to_le_bytes());
    packet.extend_from_slice(&guild.id.to_le_bytes());
    packet.extend_from_slice(&guild.emblem_version.to_le_bytes());
    packet.extend_from_slice(&guild.emblem);
    protocol::set_length(&mut packet);
    packet
}

impl ScriptWorldService {
}
