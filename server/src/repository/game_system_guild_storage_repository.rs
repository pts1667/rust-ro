use super::*;
use crate::server::model::game_systems::{GuildStorageAccess, GuildStorageOpenReceipt};

fn prepare_tx(
    systems: &TransactionalTree,
    char_id: u32,
    revision: u64,
    original_personal_open: bool,
    original_guild_open: Option<u32>,
    personal_open_in_script: bool,
) -> ConflictableTransactionResult<GuildStorageOpenReceipt, Error> {
    let character: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
    if character.revision != revision {
        return abort("Guild storage caller changed while the script was running");
    }
    if original_personal_open && original_guild_open.is_some() {
        return abort("Invalid storage caller state");
    }
    let mut expected_locker = None;
    let result_code = if character.guild_id == 0 {
        3
    } else if original_personal_open || personal_open_in_script {
        1
    } else if original_guild_open.is_some() {
        if original_guild_open != Some(character.guild_id) {
            return abort("Guild storage caller membership changed");
        }
        2
    } else {
        match tx_read::<GuildRecord>(systems, &key(b"guild/", character.guild_id))? {
            None => 3,
            Some(guild) if !guild.members.contains(&char_id) => 5,
            Some(guild) => {
                expected_locker = tx_read(systems, &key(b"guild_storage_locker/", guild.id))?;
                if expected_locker.is_some() { 2 } else { 0 }
            }
        }
    };
    Ok(GuildStorageOpenReceipt {
        char_id,
        character_revision: revision,
        guild_id: character.guild_id,
        original_personal_open,
        original_guild_open,
        personal_open_in_script,
        expected_locker,
        result_code,
    })
}

pub fn prepare(
    repository: &SledRepository,
    char_id: u32,
    revision: u64,
    original_personal_open: bool,
    original_guild_open: Option<u32>,
    personal_open_in_script: bool,
) -> Result<GuildStorageOpenReceipt, Error> {
    Ok(
        (&repository.database.game_systems, &repository.database.characters).transaction(|(systems, characters)| {
            let _: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
            prepare_tx(
                systems,
                char_id,
                revision,
                original_personal_open,
                original_guild_open,
                personal_open_in_script,
            )
        })?,
    )
}

pub fn commit_receipts(
    systems: &TransactionalTree,
    char_id: u32,
    plan: &WorldEffectPlan,
) -> ConflictableTransactionResult<Option<(u32, Vec<InventoryRecord>)>, Error> {
    let mut opened = None;
    for receipt in &plan.guild_storage_receipts {
        if receipt.char_id != char_id
            || receipt.character_revision != plan.expected_revision
            || (receipt.personal_open_in_script && !plan.open_personal_storage)
            || (receipt.result_code == 0 && plan.open_personal_storage)
        {
            return abort("Invalid staged guild storage request");
        }
        let expected = prepare_tx(
            systems,
            char_id,
            receipt.character_revision,
            receipt.original_personal_open,
            receipt.original_guild_open,
            receipt.personal_open_in_script,
        )?;
        if expected != *receipt {
            return abort("Guild storage changed while item effects were being prepared");
        }
        if receipt.result_code == 0 {
            if tx_read::<u32>(systems, &key(b"buyer/", char_id))?.is_some() || tx_read::<u32>(systems, &key(b"vender/", char_id))?.is_some() { return abort("Cannot open guild storage while operating a store"); }
            if opened.is_some() {
                return abort("Guild storage is opened more than once");
            }
            let items: Vec<InventoryRecord> = tx_read(systems, &key(b"guild_storage/", receipt.guild_id))?.unwrap_or_default();
            tx_write(systems, &key(b"guild_storage_locker/", receipt.guild_id), &char_id)?;
            opened = Some((receipt.guild_id, items));
        }
    }
    Ok(opened)
}

pub fn reset_locks(repository: &SledRepository) -> Result<(), Error> {
    let keys = repository
        .database
        .game_systems
        .scan_prefix(b"guild_storage_locker/")
        .map(|entry| entry.map(|(key, _)| key))
        .collect::<Result<Vec<_>, _>>()?;
    repository.database.game_systems.transaction(|tree| {
        for key in &keys {
            tree.remove(key.as_ref())?;
        }
        Ok::<_, sled::transaction::ConflictableTransactionError<Error>>(())
    })?;
    Ok(())
}

pub fn open(repository: &SledRepository, char_id: u32) -> Result<GuildStorageAccess, Error> {
    Ok(
        (&repository.database.game_systems, &repository.database.characters).transaction(|(systems, characters)| {
            let _: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
            let character: CharacterGameSystems = tx_read(systems, &key(b"character/", char_id))?.unwrap_or_default();
            if character.guild_id == 0 {
                return Ok(GuildStorageAccess::NoGuild);
            }
            let Some(guild) = tx_read::<GuildRecord>(systems, &key(b"guild/", character.guild_id))? else {
                return Ok(GuildStorageAccess::NoGuild);
            };
            if !guild.members.contains(&char_id) {
                return Ok(GuildStorageAccess::NoPermission);
            }
            let locker = key(b"guild_storage_locker/", guild.id);
            if tx_read::<u32>(systems, &locker)?.is_some() {
                return Ok(GuildStorageAccess::Busy);
            }
            let items: Vec<InventoryRecord> = tx_read(systems, &key(b"guild_storage/", guild.id))?.unwrap_or_default();
            tx_write(systems, &locker, &char_id)?;
            Ok(GuildStorageAccess::Open { guild_id: guild.id, items })
        })?,
    )
}

pub fn close(repository: &SledRepository, char_id: u32, guild_id: u32) -> Result<(), Error> {
    repository.database.game_systems.transaction(|systems| {
        let locker = key(b"guild_storage_locker/", guild_id);
        if let Some(owner) = tx_read::<u32>(systems, &locker)? {
            if owner != char_id {
                return abort("Guild storage is open for another member");
            }
            systems.remove(locker)?;
        }
        Ok(())
    })?;
    Ok(())
}

pub fn require_access(systems: &TransactionalTree, char_id: u32) -> ConflictableTransactionResult<u32, Error> {
    let character: CharacterGameSystems = tx_required(systems, &key(b"character/", char_id))?;
    if character.guild_id == 0 {
        return abort("Guild storage requires guild membership");
    }
    let guild: GuildRecord = tx_required(systems, &key(b"guild/", character.guild_id))?;
    if !guild.members.contains(&char_id) {
        return abort("Character has no guild storage permission");
    }
    if tx_read::<u32>(systems, &key(b"guild_storage_locker/", guild.id))? != Some(char_id) {
        return abort("Guild storage is not open for this character");
    }
    Ok(guild.id)
}

pub fn release_member_lock(systems: &TransactionalTree, char_id: u32, guild_id: u32) -> ConflictableTransactionResult<(), Error> {
    let locker = key(b"guild_storage_locker/", guild_id);
    if tx_read::<u32>(systems, &locker)? == Some(char_id) {
        systems.remove(locker)?;
    }
    Ok(())
}
