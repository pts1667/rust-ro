use super::*;
use crate::server::model::game_systems::{GUILD_POSITION_COUNT, GuildNotice, GuildPosition, GuildRecord};

const MAX_ALLIES: usize = 3;
const MAX_OPPOSITION: usize = 3;
const MAX_EXP_TAX: u8 = 50;
const MAX_NOTICE_SUBJECT: usize = 60;
const MAX_NOTICE_BODY: usize = 120;
const MAX_EMBLEM_BYTES: usize = 2048;

type Tx<T> = ConflictableTransactionResult<T, Error>;

fn guild_key(id: u32) -> Vec<u8> {
    key(b"guild/", id)
}

fn member_guild(systems: &TransactionalTree, char_id: u32) -> Tx<GuildRecord> {
    let state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
    if state.guild_id == 0 {
        return abort("Character does not belong to a guild");
    }
    let guild: GuildRecord = tx_required(systems, &guild_key(state.guild_id))?;
    if !guild.members.contains(&char_id) {
        return abort("Guild membership is inconsistent");
    }
    Ok(guild)
}

fn master_guild(systems: &TransactionalTree, actor: u32) -> Tx<GuildRecord> {
    let guild = member_guild(systems, actor)?;
    if guild.master_char_id != actor {
        return abort("Only the guild master can do this");
    }
    Ok(guild)
}

fn save(systems: &TransactionalTree, guild: &GuildRecord) -> Tx<()> {
    tx_write(systems, &guild_key(guild.id), guild)
}

pub fn set_positions(repository: &SledRepository, actor: u32, positions: Vec<GuildPosition>) -> Result<GuildRecord, Error> {
    if positions.len() != GUILD_POSITION_COUNT
        || positions.iter().any(|position| {
            position.name.trim().is_empty()
                || position.name.len() > 24
                || position.name.chars().any(char::is_control)
                || position.exp_tax > MAX_EXP_TAX
        })
        || !positions[0].invite
        || !positions[0].punish
        || positions[0].exp_tax != 0
    {
        return Err(Error::new("Invalid guild position table".into()));
    }
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut guild = master_guild(systems, actor)?;
        guild.positions = positions.clone();
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn set_member_position(repository: &SledRepository, actor: u32, target: u32, position: u8) -> Result<GuildRecord, Error> {
    if position == 0 || usize::from(position) >= GUILD_POSITION_COUNT {
        return Err(Error::new("Invalid guild position".into()));
    }
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut guild = master_guild(systems, actor)?;
        if target == guild.master_char_id || !guild.members.contains(&target) {
            return abort("Position target is not an ordinary member of this guild");
        }
        guild.member_positions.insert(target, position);
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn set_notice(repository: &SledRepository, actor: u32, notice: GuildNotice) -> Result<GuildRecord, Error> {
    if notice.subject.len() > MAX_NOTICE_SUBJECT
        || notice.body.len() > MAX_NOTICE_BODY
        || notice.subject.chars().chain(notice.body.chars()).any(|c| c.is_control() && c != '\n')
    {
        return Err(Error::new("Invalid guild notice".into()));
    }
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut guild = master_guild(systems, actor)?;
        guild.notice = notice.clone();
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn set_emblem(repository: &SledRepository, actor: u32, emblem: Vec<u8>) -> Result<GuildRecord, Error> {
    if emblem.len() > MAX_EMBLEM_BYTES {
        return Err(Error::new("Guild emblem is too large".into()));
    }
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut guild = master_guild(systems, actor)?;
        guild.emblem = emblem.clone();
        guild.emblem_version = guild.emblem_version.wrapping_add(1);
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn upgrade_skill(repository: &SledRepository, actor: u32, skill_id: u32) -> Result<GuildRecord, Error> {
    let (_, _, maximum, _) = *crate::server::model::game_systems::GUILD_SKILL_TREE
        .iter()
        .find(|(id, ..)| *id == skill_id)
        .ok_or_else(|| Error::new("Unknown guild skill".into()))?;
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut guild = master_guild(systems, actor)?;
        if guild.skill_points == 0 || guild.skill_level(skill_id) >= maximum || !guild.skill_available(skill_id) {
            return abort("This guild skill cannot be upgraded");
        }
        guild.skills.insert(skill_id, guild.skill_level(skill_id) + 1);
        guild.skill_points -= 1;
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn invite_member(repository: &SledRepository, inviter: u32, target: u32) -> Result<GuildRecord, Error> {
    Ok((&repository.database.game_systems, &repository.database.characters).transaction(|(systems, characters)| {
        let mut guild = member_guild(systems, inviter)?;
        if !guild.can_invite(inviter) {
            return abort("Your guild position cannot invite members");
        }
        tx_required::<CharacterRecord>(characters, &(target as i32).to_be_bytes())?;
        let mut state: CharacterGameSystems = tx_read(systems, &character_key(target))?.unwrap_or_default();
        if state.guild_id != 0 || guild.members.len() >= guild.max_members() {
            return abort("Target already belongs to a guild or this guild is full");
        }
        state.guild_id = guild.id;
        guild.members.push(target);
        write_state(systems, target, &mut state)?;
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn expel_member(repository: &SledRepository, actor: u32, target: u32) -> Result<GuildRecord, Error> {
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut guild = member_guild(systems, actor)?;
        if !guild.can_punish(actor) {
            return abort("Your guild position cannot expel members");
        }
        if target == guild.master_char_id || target == actor || !guild.members.contains(&target) {
            return abort("This member cannot be expelled");
        }
        let mut state: CharacterGameSystems = tx_required(systems, &character_key(target))?;
        state.guild_id = 0;
        guild_storage::release_member_lock(systems, target, guild.id)?;
        guild.members.retain(|id| *id != target);
        guild.member_positions.remove(&target);
        write_state(systems, target, &mut state)?;
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn form_alliance(repository: &SledRepository, master_a: u32, master_b: u32) -> Result<(GuildRecord, GuildRecord), Error> {
    if master_a == master_b {
        return Err(Error::new("A guild cannot ally with itself".into()));
    }
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut first = master_guild(systems, master_a)?;
        let mut second = master_guild(systems, master_b)?;
        if first.id == second.id
            || first.allies.contains(&second.id)
            || first.opposition.contains(&second.id)
            || second.opposition.contains(&first.id)
        {
            return abort("These guilds already have a relationship");
        }
        if first.allies.len() >= MAX_ALLIES || second.allies.len() >= MAX_ALLIES {
            return abort("A guild alliance limit was reached");
        }
        first.allies.push(second.id);
        second.allies.push(first.id);
        save(systems, &first)?;
        save(systems, &second)?;
        Ok((first, second))
    })?)
}

pub fn declare_opposition(repository: &SledRepository, master: u32, target_guild: u32) -> Result<GuildRecord, Error> {
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut guild = master_guild(systems, master)?;
        let _: GuildRecord = tx_required(systems, &guild_key(target_guild))?;
        if guild.id == target_guild || guild.allies.contains(&target_guild) || guild.opposition.contains(&target_guild) {
            return abort("This guild relationship is not allowed");
        }
        if guild.opposition.len() >= MAX_OPPOSITION {
            return abort("The guild opposition limit was reached");
        }
        guild.opposition.push(target_guild);
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn break_relation(repository: &SledRepository, master: u32, other_guild: u32) -> Result<GuildRecord, Error> {
    Ok(repository.database.game_systems.transaction(|systems| {
        let mut guild = master_guild(systems, master)?;
        let was_ally = guild.allies.contains(&other_guild);
        if !was_ally && !guild.opposition.contains(&other_guild) {
            return abort("These guilds have no relationship");
        }
        guild.allies.retain(|id| *id != other_guild);
        guild.opposition.retain(|id| *id != other_guild);
        if was_ally {
            if let Some(mut other) = tx_read::<GuildRecord>(systems, &guild_key(other_guild))? {
                other.allies.retain(|id| *id != guild.id);
                save(systems, &other)?;
            }
        }
        save(systems, &guild)?;
        Ok(guild)
    })?)
}

pub fn clear_relations(systems: &TransactionalTree, guild: &GuildRecord) -> Tx<()> {
    for other_id in guild.allies.iter().chain(guild.opposition.iter()) {
        if let Some(mut other) = tx_read::<GuildRecord>(systems, &guild_key(*other_id))? {
            other.allies.retain(|id| *id != guild.id);
            other.opposition.retain(|id| *id != guild.id);
            save(systems, &other)?;
        }
    }
    Ok(())
}

/// Highest castle data selector accepted from NPC scripts.
pub const CASTLE_FIELD_COUNT: u8 = 26;

fn castle_key(map: &str) -> Vec<u8> {
    [b"castle/".as_slice(), map.trim_end_matches(".gat").to_lowercase().as_bytes()].concat()
}

pub fn castle_value(repository: &SledRepository, map: &str, field: u8) -> Result<i32, Error> {
    if field == 0 || field > CASTLE_FIELD_COUNT {
        return Err(Error::new("Unknown castle data selector".into()));
    }
    let values: BTreeMap<u8, i32> = read(&repository.database.game_systems, &castle_key(map))?.unwrap_or_default();
    Ok(values.get(&field).copied().unwrap_or(0))
}

pub fn set_castle_value(repository: &SledRepository, map: &str, field: u8, value: i32) -> Result<(), Error> {
    if field == 0 || field > CASTLE_FIELD_COUNT {
        return Err(Error::new("Unknown castle data selector".into()));
    }
    repository.database.game_systems.transaction(|systems| {
        let mut values: BTreeMap<u8, i32> = tx_read(systems, &castle_key(map))?.unwrap_or_default();
        if field == 1 {
            let owner = u32::try_from(value).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::new("Invalid castle owner".into())))?;
            if owner != 0 && tx_read::<GuildRecord>(systems, &guild_key(owner))?.is_none() {
                return abort("Castle owner guild does not exist");
            }
        }
        if value == 0 {
            values.remove(&field);
        } else {
            values.insert(field, value);
        }
        tx_write(systems, &castle_key(map), &values)
    })?;
    Ok(())
}
