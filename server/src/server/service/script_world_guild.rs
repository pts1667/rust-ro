use database::model::CharacterRecord;
use models::enums::EnumWithMaskValueU32;

use super::{ScriptWorldService, install_state, protocol, world_data};
use crate::server::Server;
use crate::server::model::game_systems::{GuildInvitation, GuildMenu, GuildPermission, GuildRecord, ScriptWorldRequest};
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
    packet.extend_from_slice(
        &(if master {
            GuildPermission::Invite.as_flag() | GuildPermission::Expel.as_flag()
        } else {
            0
        })
        .to_le_bytes(),
    );
    packet.push(u8::from(master));
    packet.extend_from_slice(&0u32.to_le_bytes());
    protocol::fixed_string(&mut packet, guild.map_or("", |guild| guild.name.as_str()), 24);
    packet
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
        self.send(character.char_id, self.guild_summary_packet(server, character, &guild)?)
    }

    pub(crate) fn guild_request(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        request: ScriptWorldRequest,
    ) -> Result<(), String> {
        match request {
            ScriptWorldRequest::CreateGuild(name) => {
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
            ScriptWorldRequest::InviteGuild(target_id) => {
                let guild = self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Inviter does not belong to a guild")?;
                if guild.master_char_id != character.char_id {
                    return Err("Only the guild master can invite members".into());
                }
                let target = state
                    .characters_mut()
                    .get_mut(&target_id)
                    .ok_or("Guild invite target is not online")?;
                if target.game_systems.guild_id != 0 || target.game_systems.guild_invitation.is_some() {
                    return self.send(character.char_id, vec![0x69, 0x01, 0]);
                }
                if guild.members.len() >= 16 {
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
            ScriptWorldRequest::AnswerGuildInvite { guild_id, accept } => {
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
                if guild.master_char_id != inviter.char_id {
                    return Err("Guild invitation is no longer authorized".into());
                }
                if guild.members.len() >= 16 {
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
            ScriptWorldRequest::GuildMenu => {
                let guild = self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?;
                let mut mode = GuildMenu::Members.as_flag() | GuildMenu::Skills.as_flag() | GuildMenu::Notice.as_flag();
                if guild.as_ref().is_some_and(|guild| guild.master_char_id == character.char_id) {
                    mode |= GuildMenu::Positions.as_flag() | GuildMenu::Expulsions.as_flag();
                }
                let mut packet = protocol::header(0x014E);
                packet.extend_from_slice(&mode.to_le_bytes());
                self.send(character.char_id, packet)
            }
            ScriptWorldRequest::GuildInformation(kind) => {
                let guild = self
                    .repository
                    .guild(character.game_systems.guild_id)
                    .map_err(|error| error.to_string())?
                    .ok_or("Character does not belong to a guild")?;
                match kind {
                    0 | 3 => self.send(character.char_id, self.guild_summary_packet(server, character, &guild)?),
                    1 => {
                        self.send(character.char_id, guild_positions(false))?;
                        self.send(character.char_id, self.guild_members_packet(server, character, &guild)?)
                    }
                    2 => {
                        self.send(character.char_id, guild_positions(false))?;
                        self.send(character.char_id, guild_positions(true))
                    }
                    4 => self.send(character.char_id, vec![0x63, 0x01, 4, 0]),
                    6 => {
                        let mut packet = protocol::header(0x016F);
                        packet.extend_from_slice(&[0; 180]);
                        self.send(character.char_id, packet)
                    }
                    _ => Err("Unknown guild information section".into()),
                }
            }
            ScriptWorldRequest::LeaveGuild { guild_id, reason } => {
                self.guild_remove_member(server, state, character, guild_id, character.char_id, reason, false)
            }
            ScriptWorldRequest::ExpelGuild {
                guild_id,
                member_id,
                reason,
            } => self.guild_remove_member(server, state, character, guild_id, member_id, reason, true),
            ScriptWorldRequest::DisbandGuild(name) => {
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
            _ => Err("Unknown guild operation".into()),
        }
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
            packet.extend_from_slice(&(if record.char_id as u32 == guild.master_char_id { 0u32 } else { 19 }).to_le_bytes());
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
        let guild = self
            .repository
            .leave_guild(member, guild_id, expelled.then_some(character.char_id))
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

fn guild_positions(info: bool) -> Vec<u8> {
    let mut packet = protocol::header(if info { 0x0160 } else { 0x0166 });
    packet.extend_from_slice(&0u16.to_le_bytes());
    for position in 0u32..20 {
        packet.extend_from_slice(&position.to_le_bytes());
        if info {
            packet.extend_from_slice(
                &(if position == 0 {
                    GuildPermission::Invite.as_flag() | GuildPermission::Expel.as_flag()
                } else {
                    0
                })
                .to_le_bytes(),
            );
            packet.extend_from_slice(&position.to_le_bytes());
            packet.extend_from_slice(&0u32.to_le_bytes());
        } else {
            protocol::fixed_string(&mut packet, if position == 0 { "Guild Master" } else { "Member" }, 24);
        }
    }
    protocol::set_length(&mut packet);
    packet
}
